use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use clap::{Parser as _, Subcommand};
use rdstream::{Parsed, Rules, describe, extract};

/// Stream-parse R .rds/.RData files without loading them into memory.
#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
    /// gzip decoder threads per layer: 0 = size from the memory budget,
    /// 1 = single-threaded (lowest memory), n = exactly n.
    #[arg(long, global = true, default_value_t = 0)]
    threads: usize,
    /// Memory the parallel decoders may use, e.g. 4G or 512M (default: half of available memory).
    #[arg(long, global = true, value_parser = parse_size)]
    memory_budget: Option<u64>,
    /// Copy standard input to a temporary file in DIR first, so it decodes in
    /// parallel like a file (needs as much space as the compressed input).
    #[arg(long, global = true, value_name = "DIR")]
    spool_dir: Option<PathBuf>,
}

fn parse_size(s: &str) -> Result<u64, String> {
    let s = s.trim();
    let (num, mult) = match s.char_indices().find(|(_, c)| c.is_ascii_alphabetic()) {
        Some((i, _)) => {
            let mult = match s[i..]
                .to_ascii_uppercase()
                .trim_end_matches('B')
                .trim_end_matches('I')
            {
                "" => 1u64,
                "K" => 1 << 10,
                "M" => 1 << 20,
                "G" => 1 << 30,
                "T" => 1 << 40,
                u => return Err(format!("unknown size unit {u:?}")),
            };
            (&s[..i], mult)
        }
        None => (s, 1),
    };
    let n: f64 = num
        .trim()
        .parse()
        .map_err(|e| format!("bad size {s:?}: {e}"))?;
    Ok((n * mult as f64) as u64)
}

#[derive(Subcommand)]
enum Cmd {
    /// Print the object tree; large vectors are skipped and shown with their length.
    Describe {
        /// Input file (gzip/bzip2/xz/zstd, possibly nested), or `-` for stdin.
        file: PathBuf,
        #[arg(long, default_value_t = 5)]
        depth: usize,
        /// Load vectors up to this many elements.
        #[arg(long, default_value_t = 1_000_000)]
        threshold: u64,
    },
    /// Peel all compression layers and write the raw serialization stream to stdout.
    Cat { file: PathBuf },
    /// Write vectors under the given paths to .npy/.json files plus manifest.json,
    /// streaming them to disk as they are read.
    Extract {
        file: PathBuf,
        /// Path glob to extract with everything below it (repeatable), e.g.
        /// '$obj@meta.data' or '*@counts'.
        #[arg(long = "path", required = true)]
        paths: Vec<String>,
        #[arg(long)]
        out: PathBuf,
        /// Also load (but not write) vectors up to this many elements elsewhere.
        #[arg(long, default_value_t = 0)]
        threshold: u64,
    },
}

fn report(p: &Parsed, secs: f64) {
    let d = &p.document;
    let (maj, min, patch) = d.writer_version;
    eprintln!(
        "{:?} v{} written by R {maj}.{min}.{patch}, layers [{}]: {} bytes in {secs:.1}s ({:.0} MB/s)",
        d.format,
        d.version,
        p.layers
            .iter()
            .map(|l| format!("{:?}: {:?} x{}", l.compression, l.decoder, l.threads))
            .collect::<Vec<_>>()
            .join(", "),
        describe::thousands(d.bytes_read),
        d.bytes_read as f64 / 1e6 / secs
    );
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let opts = rdstream::DecodeOptions {
        threads: cli.threads,
        memory_budget: cli.memory_budget,
        spool_dir: cli.spool_dir,
    };
    match cli.cmd {
        Cmd::Describe {
            file,
            depth,
            threshold,
        } => {
            let t = Instant::now();
            let parsed = rdstream::parse_source(
                rdstream::source_for(&file),
                &mut Rules {
                    threshold,
                    ..Default::default()
                },
                &opts,
            )?;
            let secs = t.elapsed().as_secs_f64();
            let mut out = io::stdout().lock();
            describe::describe(&parsed.document, depth, &mut out)?;
            out.flush()?;
            report(&parsed, secs);
        }
        Cmd::Cat { file } => {
            let (mut inner, _) =
                rdstream::decompress::decompress_layers(rdstream::source_for(&file), &opts)?;
            io::copy(&mut inner, &mut io::stdout().lock())?;
        }
        Cmd::Extract {
            file,
            paths,
            out,
            threshold,
        } => {
            let t = Instant::now();
            let mut extractor = extract::Extractor::new(&paths, &out, threshold)?;
            let parsed =
                rdstream::parse_source(rdstream::source_for(&file), &mut extractor, &opts)?;
            let secs = t.elapsed().as_secs_f64();
            let entries = extractor.finish(&parsed.document)?;
            report(&parsed, secs);
            eprintln!("wrote {} files to {}", entries.len(), out.display());
        }
    }
    Ok(())
}
