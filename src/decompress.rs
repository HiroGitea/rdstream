//! Detect and peel compression layers (gzip, bzip2, xz, zstd). Each layer
//! decodes on its own threads, so nested layers (e.g. GEO re-gzipping R's
//! gzipped `save()` output) and the parser run as a pipeline.
//!
//! With the `parallel-gzip` feature, gzip in a regular file is decoded on
//! multiple cores by `rapidgzip-core` when there is memory for it. It buffers
//! speculative chunks, so the number of workers is sized against a memory
//! budget (by default half of the currently available memory) and falls back to
//! single-threaded zlib-rs when the budget is too small.
//!
//! Forward-only streams (pipes) decode with zlib-rs, one thread per layer.
//! Parallel decoders need random access: rusty-rapidgzip's streaming mode was
//! at most 1.4× faster while using gigabytes, and it can mistake bytes of a
//! recompressed layer for a block start. [`DecodeOptions::spool_dir`] copies a
//! stream to a temporary file so it can be decoded like one.
//!
//! A regular file that is gzip inside gzip (GEO re-compresses R's gzipped
//! `save()` output) is decoded in parallel on both layers. The outer layer
//! holds already-compressed bytes, which rapidgzip-core only decodes
//! sequentially; rusty-rapidgzip decodes them in parallel from the
//! memory-mapped file, but is very slow on some encoders' output (stored or
//! fixed-Huffman blocks), so it is tried on a sample of the file first. A
//! counting pass over the outer layer gives the inner layer's exact length,
//! and rapidgzip-core then decodes the inner layer while reading the outer
//! decoder's output through a bounded window (see [`window`]).

use std::fs::File;
use std::io::{self, BufReader, Cursor, Read, Seek};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, sync_channel};
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    Gzip,
    Bzip2,
    Xz,
    /// R 4.5's `zstdfile()` connections, or the `zstd` command.
    Zstd,
}

impl Compression {
    pub fn detect(magic: &[u8]) -> Option<Self> {
        if magic.starts_with(&[0x1f, 0x8b]) {
            Some(Compression::Gzip)
        } else if magic.starts_with(b"BZh") {
            Some(Compression::Bzip2)
        } else if magic.starts_with(&[0xfd, b'7', b'z', b'X', b'Z', 0x00]) {
            Some(Compression::Xz)
        } else if magic.starts_with(&[0x28, 0xb5, 0x2f, 0xfd])
            // A skippable frame (magic 0x184D2A5?), as some zstd tools write first.
            || (magic.len() >= 4 && magic[0] & 0xf0 == 0x50 && magic[1..4] == [0x2a, 0x4d, 0x18])
        {
            Some(Compression::Zstd)
        } else {
            None
        }
    }

    /// Single-threaded decoder, run on its own thread by the caller.
    fn decoder(self, r: Box<dyn Read + Send>) -> io::Result<Box<dyn Read + Send>> {
        let r = BufReader::with_capacity(1 << 20, r);
        Ok(match self {
            Compression::Gzip => Box::new(flate2::bufread::MultiGzDecoder::new(r)),
            Compression::Bzip2 => Box::new(bzip2::bufread::MultiBzDecoder::new(r)),
            Compression::Xz => Box::new(liblzma::bufread::XzDecoder::new_multi_decoder(r)),
            Compression::Zstd => {
                // Concatenated frames are decoded in turn. Allow the windows of
                // `zstd --long` (up to 2 GiB); a frame only allocates what it declares.
                let mut d = zstd::stream::read::Decoder::with_buffer(r)?;
                d.window_log_max(if cfg!(target_pointer_width = "64") {
                    31
                } else {
                    30
                })?;
                Box::new(d)
            }
        })
    }
}

/// Implementation chosen for a layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decoder {
    /// Single-threaded (zlib-rs, bzip2, liblzma).
    Serial,
    /// rapidgzip-core over a seekable file.
    ParallelFile,
    /// rusty-rapidgzip over a memory-mapped file (recompressed outer layers).
    ParallelMapped,
    /// rapidgzip-core over a bounded window of the layer above, whose length
    /// was measured by an extra pass over the file.
    ParallelWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layer {
    pub compression: Compression,
    pub decoder: Decoder,
    pub threads: usize,
}

/// Where the compressed bytes come from.
pub enum Source {
    /// A path; regular files can be decoded with random access.
    File(PathBuf),
    /// A forward-only stream.
    Stream(Box<dyn Read + Send>),
}

#[derive(Debug, Clone, Default)]
pub struct DecodeOptions {
    /// Decoder threads per gzip layer. `0` sizes them from the memory budget,
    /// `1` forces single-threaded decoding (lowest memory), `n > 1` uses `n`
    /// workers regardless of the budget.
    pub threads: usize,
    /// Bytes the parallel decoders may use; `None` = half of available memory.
    pub memory_budget: Option<u64>,
    /// Copy a forward-only stream to a temporary file in this directory first,
    /// so that it can be decoded in parallel like a file. The file is as large
    /// as the compressed input and is removed when decoding ends.
    pub spool_dir: Option<PathBuf>,
}

// Peak memory measured on GEO's GSE183852 Seurat objects: rapidgzip-core grows
// ~100 MB per worker (4: 0.47 GB, 8: 0.84 GB, 16: 1.58 GB on a 2.2 GB layer).
const FILE_BYTES_PER_THREAD: u64 = 110 << 20;
const FILE_MAX_THREADS: usize = 16;
/// Budget assumed where available memory cannot be read.
const FALLBACK_BUDGET: u64 = 2 << 30;

/// `MemAvailable` from /proc/meminfo (Linux only).
pub fn available_memory() -> Option<u64> {
    let info = std::fs::read_to_string("/proc/meminfo").ok()?;
    let line = info.lines().find(|l| l.starts_with("MemAvailable:"))?;
    let kb: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kb * 1024)
}

/// Hands out decoder threads per layer while tracking the memory they claim.
struct Planner {
    explicit_threads: Option<usize>,
    budget: u64,
    cores: usize,
}

impl Planner {
    fn new(opts: &DecodeOptions) -> Self {
        let budget = opts
            .memory_budget
            .unwrap_or_else(|| available_memory().map_or(FALLBACK_BUDGET, |m| m / 2));
        Planner {
            explicit_threads: (opts.threads > 0).then_some(opts.threads),
            budget,
            cores: thread::available_parallelism().map_or(1, |n| n.get()),
        }
    }

    fn file_threads(&mut self) -> usize {
        if !cfg!(feature = "parallel-gzip") {
            return 1;
        }
        if let Some(t) = self.explicit_threads {
            return t;
        }
        let t = self
            .cores
            .min(FILE_MAX_THREADS)
            .min((self.budget / FILE_BYTES_PER_THREAD) as usize);
        if t < 2 {
            return 1;
        }
        self.budget -= t as u64 * FILE_BYTES_PER_THREAD;
        t
    }
}

const PEEK: usize = 6;
const CHUNK: usize = 4 << 20;

/// Read up to `PEEK` bytes and return them together with a reader that replays them.
fn peek(mut r: Box<dyn Read + Send>) -> io::Result<(Vec<u8>, Box<dyn Read + Send>)> {
    let mut head = Vec::with_capacity(PEEK);
    (&mut r).take(PEEK as u64).read_to_end(&mut head)?;
    let replay = Cursor::new(head.clone()).chain(r);
    Ok((head, Box::new(replay)))
}

/// Peel every recognized compression layer. Returns the innermost stream and the
/// layers found, outermost first.
pub fn decompress_layers(
    source: Source,
    opts: &DecodeOptions,
) -> io::Result<(Box<dyn Read + Send>, Vec<Layer>)> {
    if let Source::Stream(_) = source
        && let Some(dir) = &opts.spool_dir
        && opts.threads != 1
        && cfg!(feature = "parallel-gzip")
    {
        let Source::Stream(s) = source else {
            unreachable!()
        };
        return spool(s, dir, opts);
    }
    let mut planner = Planner::new(opts);
    let mut layers = Vec::new();
    let (mut head, mut r) = match source {
        Source::Stream(s) => peek(s)?,
        Source::File(path) => {
            let mut f = File::open(&path)?;
            let mut magic = Vec::with_capacity(PEEK);
            (&mut f).take(PEEK as u64).read_to_end(&mut magic)?;
            f.rewind()?;
            let threads = match Compression::detect(&magic) {
                Some(Compression::Gzip) if f.metadata()?.is_file() => planner.file_threads(),
                _ => 1,
            };
            if threads > 1 && inner_is_gzip(&path)? {
                let outer = if mapped_decoder_keeps_up(&path, threads)? {
                    Decoder::ParallelMapped
                } else {
                    Decoder::ParallelFile
                };
                layers.push(Layer {
                    compression: Compression::Gzip,
                    decoder: outer,
                    threads,
                });
                let inner_threads = planner.file_threads();
                if inner_threads > 1 {
                    let decoder = Decoder::ParallelWindow;
                    layers.push(Layer {
                        compression: Compression::Gzip,
                        decoder,
                        threads: inner_threads,
                    });
                    peek(open_recompressed_gzip_file(
                        &path,
                        outer,
                        threads,
                        inner_threads,
                    )?)?
                } else {
                    peek(open_outer(&path, outer, threads)?)? // the inner layer is peeled below
                }
            } else if threads > 1 {
                layers.push(Layer {
                    compression: Compression::Gzip,
                    decoder: Decoder::ParallelFile,
                    threads,
                });
                peek(open_parallel_gzip_file(path, threads)?)?
            } else {
                peek(Box::new(f))?
            }
        }
    };
    // Layers below the first, and streams, decode on one thread each; the
    // threads of successive layers and the parser still run as a pipeline.
    while let Some(compression) = Compression::detect(&head) {
        layers.push(Layer {
            compression,
            decoder: Decoder::Serial,
            threads: 1,
        });
        r = Box::new(spawn_pipe(compression.decoder(r)?));
        (head, r) = peek(r)?;
    }
    Ok((r, layers))
}

/// Copy `stream` into a temporary file in `dir` and decode that as a file. If
/// the copy fails (say the disk fills up), decoding continues single-threaded
/// from what was copied followed by the rest of the stream.
fn spool(
    mut stream: Box<dyn Read + Send>,
    dir: &std::path::Path,
    opts: &DecodeOptions,
) -> io::Result<(Box<dyn Read + Send>, Vec<Layer>)> {
    use std::io::Write;

    let temp = Spooled::create(dir)?;
    let mut file = File::options().append(true).open(&temp.0)?;
    let mut buf = vec![0u8; 4 << 20];
    // Bytes read from the stream that did not make it into the file.
    let mut unwritten = None;
    loop {
        let n = match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        let mut done = 0;
        while done < n {
            match file.write(&buf[done..n]) {
                Ok(0) | Err(_) => break,
                Ok(k) => done += k,
            }
        }
        if done < n {
            unwritten = Some(buf[done..n].to_vec());
            break;
        }
    }
    drop(file);
    let (inner, layers) = match unwritten {
        None => decompress_layers(
            Source::File(temp.0.clone()),
            &DecodeOptions {
                spool_dir: None,
                ..opts.clone()
            },
        )?,
        Some(rest) => {
            let replay = File::open(&temp.0)?.chain(Cursor::new(rest)).chain(stream);
            decompress_layers(
                Source::Stream(Box::new(replay)),
                &DecodeOptions {
                    spool_dir: None,
                    ..opts.clone()
                },
            )?
        }
    };
    Ok((Box::new(SpooledReader { inner, _temp: temp }), layers))
}

/// A spool file, removed when dropped.
struct Spooled(PathBuf);

impl Spooled {
    fn create(dir: &std::path::Path) -> io::Result<Self> {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let path = dir.join(format!("rdstream-spool-{}-{nanos:x}", std::process::id()));
        File::options().write(true).create_new(true).open(&path)?;
        Ok(Spooled(path))
    }
}

impl Drop for Spooled {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Keeps the spool file until the decoded stream is dropped.
struct SpooledReader {
    inner: Box<dyn Read + Send>,
    _temp: Spooled,
}

impl Read for SpooledReader {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        self.inner.read(out)
    }
}

#[cfg(feature = "parallel-gzip")]
fn open_parallel_gzip_file(path: PathBuf, threads: usize) -> io::Result<Box<dyn Read + Send>> {
    let decoder = rapidgzip_core::Decoder::builder()
        .decoder_threads(threads)
        .build()
        .map_err(io::Error::other)?;
    Ok(Box::new(decoder.open(&path).map_err(io::Error::other)?))
}

#[cfg(not(feature = "parallel-gzip"))]
fn open_parallel_gzip_file(path: PathBuf, _threads: usize) -> io::Result<Box<dyn Read + Send>> {
    Ok(Box::new(spawn_pipe(
        Compression::Gzip.decoder(Box::new(File::open(path)?))?,
    )))
}

/// Whether a gzip file holds another gzip stream (decodes just its first bytes).
fn inner_is_gzip(path: &std::path::Path) -> io::Result<bool> {
    if !cfg!(feature = "parallel-gzip") {
        return Ok(false);
    }
    let mut head = Vec::with_capacity(PEEK);
    flate2::read::MultiGzDecoder::new(File::open(path)?)
        .take(PEEK as u64)
        .read_to_end(&mut head)?;
    Ok(Compression::detect(&head) == Some(Compression::Gzip))
}

/// rusty-rapidgzip over a regular file, which it memory-maps.
#[cfg(feature = "parallel-gzip")]
fn mapped_gzip_file(path: &std::path::Path, threads: usize) -> Box<dyn Read + Send> {
    let (tx, rx) = crossbeam_channel::bounded(8);
    let config = rusty_rapidgzip::Config {
        num_threads: threads,
        ..Default::default()
    };
    let path = path.to_path_buf();
    let decoder = thread::spawn(move || {
        rusty_rapidgzip::read_gz(&path, tx, config)
            .map(|_| ())
            .map_err(io::Error::other)
    });
    Box::new(ChunkReader {
        rx,
        cur: None,
        pos: 0,
        decoder: Some(decoder),
    })
}

#[cfg(not(feature = "parallel-gzip"))]
fn mapped_gzip_file(_: &std::path::Path, _: usize) -> Box<dyn Read + Send> {
    unreachable!("inner_is_gzip is false without parallel-gzip")
}

/// Whether rusty-rapidgzip decodes the start of a gzip file at least as fast
/// as a sequential decoder would. It is run on a copy of the first 32 MiB in
/// the background and given a deadline, since on unsuitable input it can take
/// many seconds for that much.
#[cfg(all(feature = "parallel-gzip", unix))]
fn mapped_decoder_keeps_up(path: &std::path::Path, threads: usize) -> io::Result<bool> {
    use std::time::{Duration, SystemTime};

    const SAMPLE: u64 = 32 << 20;
    const MIN_RATE: f64 = 200e6;
    let len = std::fs::metadata(path)?.len();
    let (sample, temp) = if len <= SAMPLE {
        (path.to_path_buf(), None)
    } else {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| d.subsec_nanos());
        let temp = std::env::temp_dir().join(format!(
            "rdstream-sample-{}-{nanos:x}.gz",
            std::process::id()
        ));
        io::copy(
            &mut File::open(path)?.take(SAMPLE),
            &mut File::create(&temp)?,
        )?;
        (temp.clone(), Some(temp))
    };
    let size = len.min(SAMPLE);
    let (tx, rx) = sync_channel(1);
    thread::spawn(move || {
        // A sample ends mid-stream, so the decoder's eventual error is expected.
        let mut r = mapped_gzip_file(&sample, threads);
        let mut buf = vec![0u8; 1 << 20];
        let mut n = 0u64;
        while let Ok(k @ 1..) = r.read(&mut buf) {
            n += k as u64;
        }
        let _ = tx.send(n);
    });
    let deadline = Duration::from_secs_f64(0.2 + size as f64 / MIN_RATE);
    let decoded = rx.recv_timeout(deadline).unwrap_or(0);
    if let Some(temp) = temp {
        let _ = std::fs::remove_file(temp); // fine while still mapped
    }
    // Recompressed data hardly shrinks: a working decoder yields about `size` bytes.
    Ok(decoded >= size / 2)
}

#[cfg(not(all(feature = "parallel-gzip", unix)))]
fn mapped_decoder_keeps_up(_: &std::path::Path, _: usize) -> io::Result<bool> {
    Ok(false)
}

fn open_outer(
    path: &std::path::Path,
    decoder: Decoder,
    threads: usize,
) -> io::Result<Box<dyn Read + Send>> {
    match decoder {
        Decoder::ParallelMapped => Ok(mapped_gzip_file(path, threads)),
        _ => open_parallel_gzip_file(path.to_path_buf(), threads),
    }
}

/// Both layers of a gzip-in-gzip file, each decoded in parallel. Returns the
/// inner layer's output.
#[cfg(feature = "parallel-gzip")]
fn open_recompressed_gzip_file(
    path: &std::path::Path,
    outer: Decoder,
    threads: usize,
    inner_threads: usize,
) -> io::Result<Box<dyn Read + Send>> {
    // Counting pass: the inner decoder needs the inner layer's exact length up front.
    let len = io::copy(&mut open_outer(path, outer, threads)?, &mut io::sink())?;
    let source = window::WindowSource::spawn(open_outer(path, outer, threads)?, len, inner_threads);
    let inner = rapidgzip_core::Decoder::builder()
        .decoder_threads(inner_threads)
        .build()
        .map_err(io::Error::other)?;
    let reader = inner.reader(source.clone()).map_err(io::Error::other)?;
    Ok(Box::new(window::WindowReader::new(reader, source)))
}

#[cfg(not(feature = "parallel-gzip"))]
fn open_recompressed_gzip_file(
    _: &std::path::Path,
    _: Decoder,
    _: usize,
    _: usize,
) -> io::Result<Box<dyn Read + Send>> {
    unreachable!("inner_is_gzip is false without parallel-gzip")
}

#[cfg(feature = "parallel-gzip")]
pub mod window {
    //! A forward-only stream exposed as a positional [`ReadAt`] source.
    //!
    //! rapidgzip-core decodes in parallel by reading compressed chunks ahead of
    //! the output it hands out: task `k` reads about `[k·S, (k+1)·S + 512 KiB)`
    //! for chunk size `S`, at most `threads` tasks are in flight, and apart
    //! from a scan of the first 8 MiB it never reads far behind the newest
    //! request. So it can read the output of the layer above while that is
    //! still being produced, from a window of `threads·S + 16 MiB` behind the
    //! furthest requested byte. Reads of bytes not produced yet block; reads
    //! of evicted bytes fail, which rapidgzip-core answers by decoding the rest
    //! sequentially (counted in [`WindowSource::misses`]).

    use std::collections::VecDeque;
    use std::io::{self, Read};
    use std::sync::{Arc, Condvar, Mutex, MutexGuard};
    use std::thread;

    use rapidgzip_core::ReadAt;

    /// rapidgzip-core's default spacing of speculative chunk starts.
    const CHUNK: u64 = 1 << 20;
    const MARGIN: u64 = 16 << 20;
    /// How far the producer may run ahead of the furthest request.
    const AHEAD: u64 = 32 << 20;
    const BLOCK: usize = 4 << 20;

    struct Block {
        start: u64,
        data: Vec<u8>,
    }

    #[derive(Default)]
    struct State {
        blocks: VecDeque<Block>,
        produced: u64,
        max_end: u64,
        /// Producer finished: `Ok` at EOF, or its error message.
        done: Option<Result<(), String>>,
        closed: bool,
        misses: u64,
    }

    struct Shared {
        state: Mutex<State>,
        changed: Condvar,
        len: u64,
        behind: u64,
    }

    impl Shared {
        fn lock(&self) -> MutexGuard<'_, State> {
            self.state.lock().unwrap_or_else(|e| e.into_inner())
        }

        fn evict(&self, st: &mut State) {
            let keep_from = st.max_end.saturating_sub(self.behind);
            while st
                .blocks
                .front()
                .is_some_and(|b| b.start + b.data.len() as u64 <= keep_from)
                && st.blocks.len() > 1
            {
                st.blocks.pop_front();
            }
        }
    }

    /// `len` bytes produced by `src` on a background thread, readable at any
    /// offset inside the retained window.
    #[derive(Clone)]
    pub struct WindowSource(Arc<Shared>);

    impl WindowSource {
        pub fn spawn(mut src: Box<dyn Read + Send>, len: u64, threads: usize) -> Self {
            let shared = Arc::new(Shared {
                state: Mutex::new(State::default()),
                changed: Condvar::new(),
                len,
                behind: threads as u64 * CHUNK + MARGIN,
            });
            let s = shared.clone();
            thread::Builder::new()
                .name("rdstream-window".into())
                .spawn(move || {
                    let done = loop {
                        let mut data = vec![0u8; BLOCK];
                        let mut n = 0;
                        let res = loop {
                            match src.read(&mut data[n..]) {
                                Ok(0) => break Ok(true),
                                Ok(k) => {
                                    n += k;
                                    if n == BLOCK {
                                        break Ok(false);
                                    }
                                }
                                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                                Err(e) => break Err(e.to_string()),
                            }
                        };
                        data.truncate(n);
                        let mut st = s.lock();
                        while !st.closed && st.produced >= st.max_end + AHEAD {
                            st = s.changed.wait(st).unwrap_or_else(|e| e.into_inner());
                        }
                        if st.closed {
                            return;
                        }
                        if n > 0 {
                            let start = st.produced;
                            st.produced += n as u64;
                            st.blocks.push_back(Block { start, data });
                            s.evict(&mut st);
                        }
                        drop(st);
                        s.changed.notify_all();
                        match res {
                            Ok(false) => {}
                            Ok(true) => break Ok(()),
                            Err(e) => break Err(e),
                        }
                    };
                    let mut st = s.lock();
                    st.done = Some(match done {
                        Ok(()) if st.produced != s.len => Err(format!(
                            "inner layer is {} bytes, expected {}",
                            st.produced, s.len
                        )),
                        other => other,
                    });
                    drop(st);
                    s.changed.notify_all();
                })
                .expect("failed to spawn window producer");
            WindowSource(shared)
        }

        /// Wake and fail every pending and future read, and stop the producer.
        pub fn close(&self) {
            self.0.lock().closed = true;
            self.0.changed.notify_all();
        }

        /// Reads that asked for bytes already evicted from the window.
        pub fn misses(&self) -> u64 {
            self.0.lock().misses
        }
    }

    impl ReadAt for WindowSource {
        fn len(&self) -> io::Result<u64> {
            Ok(self.0.len)
        }

        fn read_at(&self, offset: u64, out: &mut [u8]) -> io::Result<usize> {
            let sh = &*self.0;
            if out.is_empty() || offset >= sh.len {
                return Ok(0);
            }
            let mut st = sh.lock();
            let end = (offset + out.len() as u64).min(sh.len);
            if end > st.max_end {
                st.max_end = end;
                sh.evict(&mut st);
                sh.changed.notify_all();
            }
            loop {
                if st.closed {
                    return Err(io::Error::other("inner gzip layer closed"));
                }
                let base = st.blocks.front().map_or(st.produced, |b| b.start);
                if offset < base {
                    st.misses += 1;
                    return Err(io::Error::other(format!(
                        "offset {offset} left the decoding window"
                    )));
                }
                if offset < st.produced {
                    let mut n = 0;
                    for b in st
                        .blocks
                        .iter()
                        .skip_while(|b| b.start + (b.data.len() as u64) <= offset)
                    {
                        let from = (offset + n as u64 - b.start) as usize;
                        let k = (out.len() - n).min(b.data.len() - from);
                        out[n..n + k].copy_from_slice(&b.data[from..from + k]);
                        n += k;
                        if n == out.len() {
                            break;
                        }
                    }
                    return Ok(n);
                }
                if let Some(done) = &st.done {
                    return Err(io::Error::other(match done {
                        Err(e) => e.clone(),
                        Ok(()) => "inner gzip layer ended early".into(),
                    }));
                }
                st = sh.changed.wait(st).unwrap_or_else(|e| e.into_inner());
            }
        }
    }

    /// The inner decoder's output. Dropping it closes the window first, so
    /// decoder threads blocked on it can be joined.
    pub struct WindowReader {
        reader: Option<rapidgzip_core::DecoderReader>,
        source: WindowSource,
    }

    impl WindowReader {
        pub fn new(reader: rapidgzip_core::DecoderReader, source: WindowSource) -> Self {
            WindowReader {
                reader: Some(reader),
                source,
            }
        }
    }

    impl Read for WindowReader {
        fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
            let n = self.reader.as_mut().map_or(Ok(0), |r| r.read(out))?;
            if n == 0
                && !out.is_empty()
                && let Some(Err(e)) = self.source.0.lock().done.clone()
            {
                return Err(io::Error::other(e));
            }
            Ok(n)
        }
    }

    impl Drop for WindowReader {
        fn drop(&mut self) {
            self.source.close();
            self.reader.take();
        }
    }
}

/// Decoded chunks from rusty-rapidgzip's decoder thread.
#[cfg(feature = "parallel-gzip")]
struct ChunkReader {
    rx: crossbeam_channel::Receiver<std::sync::Arc<Vec<u8>>>,
    cur: Option<std::sync::Arc<Vec<u8>>>,
    pos: usize,
    decoder: Option<thread::JoinHandle<io::Result<()>>>,
}

#[cfg(feature = "parallel-gzip")]
impl Read for ChunkReader {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        loop {
            if let Some(c) = &self.cur
                && self.pos < c.len()
            {
                let k = out.len().min(c.len() - self.pos);
                out[..k].copy_from_slice(&c[self.pos..self.pos + k]);
                self.pos += k;
                return Ok(k);
            }
            match self.rx.recv() {
                Ok(chunk) => {
                    self.cur = Some(chunk);
                    self.pos = 0;
                }
                Err(_) => {
                    // The decoder hung up: report how it ended.
                    self.cur = None;
                    if let Some(decoder) = self.decoder.take() {
                        decoder
                            .join()
                            .unwrap_or_else(|_| Err(io::Error::other("gzip decoder panicked")))?;
                    }
                    return Ok(0);
                }
            }
        }
    }
}

/// Run `r` on a background thread, handing over decoded chunks through a bounded channel.
fn spawn_pipe(mut r: Box<dyn Read + Send>) -> PipeReader {
    let (tx, rx) = sync_channel::<io::Result<Vec<u8>>>(8);
    thread::spawn(move || {
        loop {
            let mut buf = vec![0u8; CHUNK];
            let mut n = 0;
            let res = loop {
                match r.read(&mut buf[n..]) {
                    Ok(0) => break Ok(()),
                    Ok(k) => {
                        n += k;
                        if n == CHUNK {
                            break Ok(());
                        }
                    }
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => break Err(e),
                }
            };
            if n > 0 {
                buf.truncate(n);
                if tx.send(Ok(buf)).is_err() {
                    return; // consumer gone (parse finished or failed)
                }
            }
            match res {
                Err(e) => {
                    let _ = tx.send(Err(e));
                    return;
                }
                Ok(()) if n < CHUNK => return,
                Ok(()) => {}
            }
        }
    });
    PipeReader {
        rx,
        cur: Vec::new(),
        pos: 0,
    }
}

struct PipeReader {
    rx: Receiver<io::Result<Vec<u8>>>,
    cur: Vec<u8>,
    pos: usize,
}

impl Read for PipeReader {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        while self.pos == self.cur.len() {
            match self.rx.recv() {
                Ok(chunk) => {
                    self.cur = chunk?;
                    self.pos = 0;
                }
                Err(_) => return Ok(0), // producer finished
            }
        }
        let k = out.len().min(self.cur.len() - self.pos);
        out[..k].copy_from_slice(&self.cur[self.pos..self.pos + k]);
        self.pos += k;
        Ok(k)
    }
}
