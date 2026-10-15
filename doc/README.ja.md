<div align="center">

<h1>rdstream</h1>

<p><strong>R シリアライズオブジェクトのストリーミング読み込み</strong></p>
<p>R の <code>.rds</code> および <code>.RData</code> ファイルを読み込む、Rust 製のライブラリとコマンドラインツールです。<br>R の実行環境を必要とせず、構造の確認、選択的な抽出、圧縮データの展開に対応します。</p>

<p>
  <a href="../Cargo.toml"><img src="https://img.shields.io/badge/Rust-2024_edition-c45508?style=flat-square" alt="Rust 2024 edition"></a>
  <a href="#対応フォーマット"><img src="https://img.shields.io/badge/formats-.rds_%C2%B7_.RData-276DC3?style=flat-square" alt="対応形式：.rds と .RData"></a>
  <a href="#ライセンス"><img src="https://img.shields.io/badge/license-MIT_OR_Apache--2.0-357a50?style=flat-square" alt="ライセンス：MIT OR Apache-2.0"></a>
</p>

<p><a href="../README.md">English</a> · <a href="README.zh-CN.md">简体中文</a> · <strong>日本語</strong></p>
<p><a href="#インストール">インストール</a> · <a href="#クイックスタート">クイックスタート</a> · <a href="#python-と-anndata">Python</a> · <a href="#rust-ライブラリ">Rust ライブラリ</a> · <a href="#パフォーマンス">パフォーマンス</a> · <a href="#開発への参加">開発への参加</a></p>

</div>

---

Seurat や SingleCellExperiment などの解析で保存される R オブジェクトには、
数十 GB の行列、グラフ、メタデータが含まれることがあります。
個々の解析では、その一部のみを使用する場合があります。

rdstream はシリアライズストリームを一度の順次走査で解析し、各ベクトルのデータを
読み込む前に選択ルールを適用します。オブジェクトの構造確認、細胞アノテーションの抽出、
展開データの出力は、オブジェクト全体の読み込みや展開済みの一時ファイルを必要としません。

```sh
rdstream describe dataset.rds --depth 4
rdstream extract dataset.rds --path '*@meta.data*' --out meta/
```

## rdstream の特徴

| 機能 | 説明 |
| :--- | :--- |
| **選択的な読み込み** | パスとサイズに基づいてベクトルを保持し、その他のベクトルデータをスキップします。 |
| **ストリーミング抽出** | `extract` は選択したベクトルをチャンク単位でファイルに書き出し、ベクトル全体をメモリに保持しません。 |
| **一度の順次読み込み** | 通常のファイル、標準入力、ダウンロードストリームを処理します。 |
| **多重圧縮に対応** | gzip・bzip2・xz・zstd を自動検出して順に展開します。GEO の再圧縮ファイルでは、両方の gzip 層の並列展開に対応します。 |
| **gzip の並列展開** | 利用可能メモリに応じてワーカー数を決定し、不足する場合はシングルスレッドに切り替えます。 |
| **R シリアライズのエンコーディング** | バージョン 2・3 の XDR、ASCII（`ascii = TRUE` / `NA`）、ネイティブバイナリに対応します。 |
| **R のオブジェクト構造** | 属性、S4 スロット、因子、データフレーム、参照、バイトコードを保持します。 |
| **CLI・Rust・Python** | コマンドラインでの構造確認、Rust による選択ポリシーの定義、Python での Seurat・SingleCellExperiment から AnnData への変換に対応します。 |

スキップしたベクトルのデータはメモリに保持せず、`extract` は選択したベクトルを順次書き出します。
メモリ使用量には、オブジェクト構造、保持したベクトル、デコーダーのバッファが含まれます。

## インストール

### ビルド済みバイナリ

[GitHub Releases](https://github.com/HiroGitea/rdstream/releases/latest) から、
OS とアーキテクチャに対応するアーカイブをダウンロードしてください。

| OS | x86_64 | ARM64 |
| :--- | :--- | :--- |
| Linux | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu` |
| macOS | `x86_64-apple-darwin` | `aarch64-apple-darwin` |
| Windows | `x86_64-pc-windows-msvc` | `aarch64-pc-windows-msvc` |

`.tar.gz`（Linux/macOS）または `.zip`（Windows）を展開し、`rdstream` または
`rdstream.exe` を `PATH` に含まれるディレクトリに配置してください。
各リリースにはダウンロード内容を検証するための `SHA256SUMS` が含まれます。
Linux バイナリには glibc 2.35 以降が必要です。macOS バイナリは macOS 11 以降を対象とします。

### Cargo

Rust 1.88 以降を使用して crates.io からインストールします。

```sh
cargo install rdstream --locked
rdstream --help
```

ローカルのソースからビルドする場合は、リポジトリのルートで以下を実行してください。

```sh
cargo install --path . --locked
rdstream --help
```

gzip の並列展開はデフォルトで有効です。並列デコーダーを含めずにビルドする場合：

```sh
cargo install rdstream --locked --no-default-features
```

## クイックスタート

### 1. オブジェクトの構造を確認する

```sh
rdstream describe GSE183852_DCM_Integrated.Robj.gz --depth 4
```

Seurat オブジェクトの表示例（一部省略）：

```text
$RefMerge: S4 <Seurat>
  @assays: list[4]
    [0]RNA: S4 <Assay>
      @counts: S4 <dgCMatrix>
        @i: int[426,652,860] (skipped)
        @p: int[269,795] [0, 919, 1923, 3081, …]
        @Dim: int[2] [45068, 269794]
        @x: dbl[426,652,860] (skipped)
  …
  @meta.data: list[55] <data.frame>
    …
    [24]condition: chr[269,794] ["Donor", "Donor", "Donor", "Donor", …]
    …
    [50]Names: int[269,794] [13, 13, 13, 13, …] <factor>
```

`describe` はデフォルトで **1,000,000 要素以下**のベクトルを保持します。
それより大きなベクトルは型と長さだけを表示します。しきい値は `--threshold` で変更できます。

### 2. 選択したデータを抽出する

```sh
rdstream extract GSE183852_DCM_Integrated.Robj.gz \
  --path '$RefMerge@meta.data*' \
  --out meta/
```

| 出力 | 内容 |
| :--- | :--- |
| `*.npy` | 整数、論理値、実数、複素数、raw ベクトル。 |
| `*.json` | 文字列ベクトル。 |
| `manifest.json` | パス、名前、型、長さ、および存在する場合は `dim`、`class`、因子の `levels`。 |

選択したベクトルはストリームの読み込み時にチャンク単位で書き出すため、ベクトル全体をメモリに保持する必要はありません。
パスを指定すると、その配下の全要素が選択されます。例えば `--path '$RefMerge@meta.data'` はすべての列を抽出します。

<details>
<summary><strong>抽出した因子を Python と NumPy で読み込む</strong></summary>

```python
import json
from pathlib import Path

import numpy as np

root = Path("meta")
manifest = json.loads((root / "manifest.json").read_text())
entry = next(e for e in manifest["entries"] if e["name"] == "Names")
codes = np.load(root / entry["file"])
levels = np.asarray(entry["levels"], dtype=object)

# R の因子コードは 1 始まり。欠損値は None として保持します。
cell_type = np.full(codes.shape, None, dtype=object)
valid = (codes >= 1) & (codes <= len(levels))
cell_type[valid] = levels[codes[valid] - 1]
```

</details>

### 3. ストリームを処理する

```sh
# GEO のダウンロードストリームを標準入力から読み込みます。
curl -sL https://ftp.ncbi.nlm.nih.gov/geo/series/GSE183nnn/GSE183852/suppl/GSE183852_DCM_Cells.Robj.gz \
  | rdstream describe - --depth 3

# ダウンロードストリームから細胞アノテーションを抽出します。
curl -sL https://ftp.ncbi.nlm.nih.gov/geo/series/GSE183nnn/GSE183852/suppl/GSE183852_DCM_Cells.Robj.gz \
  | rdstream extract - --path '*@meta.data' --out meta/

# ファイルのリダイレクトにより並列展開を有効にします。
rdstream describe - < GSE183852_DCM_Cells.Robj.gz

# アーカイブの出力を /scratch に一時保存して並列展開します。
tar -xOf archive.tar data/seurat.rds.gz \
  | rdstream extract - --path '*@meta.data' --out meta/ --spool-dir /scratch

# すべての圧縮層を取り除き、R のシリアライズストリームを出力します。
rdstream cat dataset.rds.gz > dataset.rds
```

パイプ入力は圧縮層ごとに 1 スレッドで展開し、解析とパイプラインで処理します。
`--spool-dir DIR` を指定すると、圧縮された入力を `DIR` 内の一時ファイルにコピーしてから並列展開します。
一時ファイルには圧縮入力と同じ容量が必要で、処理終了後に削除されます。高速なローカル入力では
スループットが改善する場合があり、その効果は入力速度とストレージ速度に依存します。
一時ファイルによるメモリ消費を避けるため、ディスク上のディレクトリを指定してください。
ファイルからリダイレクトした標準入力（`rdstream describe - < dataset.rds.gz`）は一時保存を必要としません。

## コマンド一覧

| コマンド | 用途 | 主なオプション |
| :--- | :--- | :--- |
| `rdstream describe <FILE>` | オブジェクトツリーを表示。 | `--depth`、`--threshold` |
| `rdstream extract <FILE>` | 選択したベクトルを読み込みながらファイルへ書き出す。 | `--path`（複数指定可）、`--out`、`--threshold` |
| `rdstream cat <FILE>` | 展開したバイト列を標準出力に書き出す。 | 下記の共通デコーダーオプション。 |

すべてのコマンドで、入力ファイルに `-` を指定すると標準入力から読み込みます。
すべてのオプションは `rdstream <COMMAND> --help` で確認できます。

### パスと選択ルール

| 要素 | 意味 | 例 |
| :--- | :--- | :--- |
| `$name` | タグ付き pairlist の要素。たとえば `.RData` に保存されたオブジェクト。 | `$RefMerge` |
| `@name` | 属性または S4 スロット。 | `@meta.data`、`@counts` |
| `[k]` | 0 始まりのインデックスで指定するリスト要素。 | `[24]` |

パターン中の `*` は、パス区切りを含む任意の長さの文字列に一致します。
それ以外の文字は、その文字自体に一致します。シェルではパターンを引用符で囲んでください。
例：`'*@meta.data*'`、`'$obj@assays[0]@counts*'`。
`extract` では、パターンに一致したパスの下にあるものもすべて選択されます。

R はリスト要素の名前をデータの**後に**保存するため、パスにはインデックスを使います。
`describe` は名前を読み取れた時点で、`[24]condition` のように両方を表示します。

### 展開時のメモリとスレッド

```sh
# 1 GiB の予算でデコーダーを自動調整します。
rdstream describe dataset.rds.gz --memory-budget 1G

# シングルスレッドで展開し、メモリ使用量を抑えます。
rdstream describe dataset.rds.gz --threads 1
```

デフォルトでは、Linux が報告する利用可能メモリの半分を予算にします。
その情報を取得できない場合は 2 GiB を使います。推定使用量が予算に収まらない圧縮層は、
シングルスレッドで展開します。パイプ入力は `--spool-dir` を指定しない限りシングルスレッドで展開します。

`--memory-budget` はデコーダーの推定使用量に対する予算であり、プロセス全体のメモリ上限ではありません。
`--threads N` を明示すると自動調整を上書きし、`1` より大きい値では予算を無視します。

## Python と AnnData

Python バインディングは [`python/`](../python/) にあります。Rust ツールチェーンのある環境で、
ローカルのソースからビルドしてインストールします。

```sh
pip install "./python[anndata]"
```

アトミックなベクトルはパーサーのバッファを再利用した NumPy 配列として返ります。
`rdstream.anndata` は Seurat（v3/v4 の `Assay` と、分割レイヤーを含む v5 の `Assay5`）と
SingleCellExperiment のオブジェクトを変換します。

```python
import rdstream
from rdstream.anndata import read

adata = read("GSE183852_DCM_Integrated.Robj.gz", assay="RNA", X="counts", layers=())
adata.write_h5ad("dcm.h5ad")

doc = rdstream.read("seurat.rds", keep=["*@meta.data*"], threshold=10_000)
meta = rdstream.as_pandas(doc.root.attr("meta.data"))
```

AnnData の読み込みでは、発現行列、メタデータ、次元削減の結果を選択して変換します。
疎行列の転置ではコピーが発生しません。対応関係とオプションは
[python/README.md](../python/README.md) を参照してください。

## Rust ライブラリ

rdstream を依存関係に追加します。

```toml
[dependencies]
rdstream = "0.1"
```

```rust
use rdstream::Rules;

fn main() -> rdstream::Result<()> {
    let mut rules = Rules {
        threshold: 10_000,
        keep: vec!["*@meta.data*".into()],
        ..Default::default()
    };

    let parsed = rdstream::parse_file("seurat.rds", &mut rules)?;
    for (name, object) in parsed.document.objects() {
        println!("{name}: {:?}", object.classes());
    }
    Ok(())
}
```

独自の選択ルールには、`(path: &str, sexptype: u8, len: u64) -> bool` という
シグネチャのクロージャーを渡せます。`Policy` は `Policy::sink` から `Sink` を返すこともでき、
その場合ベクトル全体をメモリに保持せずに、要素をチャンク単位で受け取れます。`extract::Extractor` はこのインターフェースで
ファイルを書き出します。`parse_source` に `Source` と `DecodeOptions` を
渡すと、入力ストリーム、デコーダーのスレッド数、メモリ予算を制御できます。
`cargo doc --open --no-deps` でローカルの API ドキュメントを生成できます。

## パフォーマンス

以下の測定は AMD EPYC 7402（24 コア、48 スレッド、メモリ 62 GB、NVMe ストレージ）で実施しました。
入力には Seurat の構造を模した 2.9 GB の合成シリアライズストリームを使用し、
gzip `-6` で圧縮した後、下表のレベルで再圧縮しています。

| 外側の圧縮レベル | 経過時間 | メモリ使用量 |
| :--- | ---: | ---: |
| gzip `-6` | 1.96 s | 1.2 GB |
| gzip `-1` | 2.03 s | 1.3 GB |
| gzip `-9` | 2.03 s | 1.2 GB |

これらの結果は、記録されたテストデータと環境における測定値です。
実際の性能とメモリ使用量は、入力データ、圧縮方式、保持するオブジェクト、展開設定に依存します。

## 対応フォーマット

| 分類 | 対応範囲 |
| :--- | :--- |
| ファイル | `saveRDS()`（`.rds`）と `save()`（`.RData`、`.rda`、`.Robj`）。 |
| シリアライズ | バージョン 2・3。XDR、ASCII（10 進数、または `ascii = NA` の 16 進浮動小数点。改行コードは問わない）、ネイティブバイナリ。 |
| 圧縮 | gzip、bzip2、xz、zstd（R 4.5 の `zstdfile()` または `zstd` コマンド）、およびそれらの多重圧縮。 |
| オブジェクトモデル | 属性、疑似 NULL を含む S4 スロット、因子、データフレーム、自己参照を含む環境、シンボル参照、バイトコードにコンパイルされたクロージャー、長いベクトル。 |
| ALTREP | コンパクトな数列、ラッパー、遅延文字列。未知のクラスはシリアライズされた状態を保持。 |
| 文字列 | UTF-8、Latin-1、およびネイティブエンコーディングが Latin-1 か Windows-1252 の場合のエンコーディング未指定の文字列。 |

### 現在の制限

- ネイティブバイナリはリトルエンディアンとして読み込みます。R は書き込み側のバイト順を記録しません。
- パイプ入力は、`--spool-dir` でディスクに書き出した場合にのみ並列で展開します。
- デコーダーのメモリ予算は推定値であり、厳密な上限ではありません。保持するオブジェクトのメモリも含みません。

## ロードマップ

- [x] PyO3 による Python バインディングと、Seurat / SingleCellExperiment → AnnData 変換。
- [x] 解析中に選択したベクトルをディスクへ直接書き出す。
- [x] 再圧縮された外側の層の並列展開を改善。
- [x] R で生成したテストフィクスチャを CI に追加。
- [x] ASCII シリアライズに対応。
- [x] zstd 圧縮に対応。
- [x] ストリームをメモリを抑えて展開し、ファイルからリダイレクトした標準入力や、一時ファイルに書き出した標準入力は並列で展開。

## 開発への参加

問題報告やプルリクエストを通じて開発に参加できます。解析に関する問題の報告には、
実行したコマンド、エラーメッセージ、判明している R のバージョンと保存形式、
および提供可能な最小再現入力を添えてください。

並列 gzip を有効にした構成と無効にした構成でテストを実行し、Python のテストも実行できます。

```sh
cargo test --locked
cargo test --locked --no-default-features
pip install "./python[test]" && pytest python/tests
```

第三者由来のテストフィクスチャの出典と内容は [tests/fixtures/MANIFEST.md](../tests/fixtures/MANIFEST.md) に記録しています。
CI では [tests/r/generate.R](../tests/r/generate.R) でフィクスチャを生成し、結果を R の出力と照合します。
同じ検証をローカルで実行するコマンドは以下のとおりです。

```sh
Rscript tests/r/generate.R target/r-fixtures
RDSTREAM_R_FIXTURES=target/r-fixtures cargo test --test r_generated
RDSTREAM_R_FIXTURES=target/r-fixtures pytest python/tests/test_r_generated.py
```

<details>
<summary><strong>関連プロジェクト</strong></summary>

R のシリアライズデータを扱うツールには、
[rds2rust](https://github.com/andrewwbutler/rds2rust)、
[rds2cpp](https://github.com/LTLA/rds2cpp) /
[rds2py](https://pypi.org/project/rds2py/)、
[rdata](https://github.com/vnmabus/rdata)、
[readseurat](https://github.com/harryhaller001/readseurat) などがあります。
[rapidgzip](https://github.com/mxmlnkn/rapidgzip) は gzip の並列展開を扱うプロジェクトです。

</details>

## ライセンス

[MIT](../LICENSE-MIT) または [Apache License 2.0](../LICENSE-APACHE) の
いずれかを選択して利用できます。
