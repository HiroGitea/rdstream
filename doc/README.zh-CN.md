<div align="center">

<h1>rdstream</h1>

<p><strong>R 序列化对象的流式读取工具</strong></p>
<p>使用 Rust 实现的 R <code>.rds</code> 和 <code>.RData</code> 文件读取库与命令行工具。<br>支持对象检查、按需提取和解压，无需 R 运行环境。</p>

<p>
  <a href="../Cargo.toml"><img src="https://img.shields.io/badge/Rust-2024_edition-c45508?style=flat-square" alt="Rust 2024 edition"></a>
  <a href="#支持的格式"><img src="https://img.shields.io/badge/formats-.rds_%C2%B7_.RData-276DC3?style=flat-square" alt="格式：.rds 和 .RData"></a>
  <a href="#许可证"><img src="https://img.shields.io/badge/license-MIT_OR_Apache--2.0-357a50?style=flat-square" alt="许可证：MIT OR Apache-2.0"></a>
</p>

<p><a href="../README.md">English</a> · <strong>简体中文</strong> · <a href="README.ja.md">日本語</a></p>
<p><a href="#安装">安装</a> · <a href="#快速开始">快速开始</a> · <a href="#python-与-anndata">Python</a> · <a href="#rust-库">Rust 库</a> · <a href="#性能">性能</a> · <a href="#参与贡献">参与贡献</a></p>

</div>

---

Seurat、SingleCellExperiment 等分析流程生成的 R 对象可能包含数十 GB 的矩阵、
图和元数据，而具体分析任务通常仅需使用其中一部分。

rdstream 通过单次顺序遍历解析序列化流，并在读取各向量的数据前应用选择规则。
对象检查、细胞注释提取及解压数据输出均无需加载完整对象，也无需创建解压后的临时文件。

```sh
rdstream describe dataset.rds --depth 4
rdstream extract dataset.rds --path '*@meta.data*' --out meta/
```

## 主要功能

| 功能 | 说明 |
| :--- | :--- |
| **按需加载** | 根据路径和大小保留向量，跳过其余向量数据。 |
| **流式提取** | `extract` 将选定向量分块写入文件，无需在内存中保留完整向量。 |
| **单次顺序读取** | 直接处理普通文件、标准输入和下载流。 |
| **多层压缩** | 自动识别并逐层解压 gzip、bzip2、xz、zstd；支持对 GEO 重复压缩文件的两层 gzip 进行并行解压。 |
| **并行 gzip** | 根据可用内存分配解压线程，预算不足时回退至单线程。 |
| **R 序列化编码** | 支持序列化版本 2 和 3 的 XDR、ASCII（`ascii = TRUE` / `NA`）和本机二进制编码。 |
| **保留 R 对象结构** | 处理属性、S4 槽、因子、数据框、引用和字节码。 |
| **命令行、Rust、Python** | 通过命令行检查对象，使用 Rust 定义选择策略，或在 Python 中将 Seurat、SingleCellExperiment 对象转换为 AnnData。 |

跳过的向量数据不会保留在内存中，`extract` 以分块方式写出选定向量。
总内存占用包括对象结构、保留的向量和解压缓冲区。

## 安装

### 预编译二进制

从 [GitHub Releases](https://github.com/HiroGitea/rdstream/releases/latest)
下载与操作系统及架构对应的压缩包。

| 操作系统 | x86_64 | ARM64 |
| :--- | :--- | :--- |
| Linux | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu` |
| macOS | `x86_64-apple-darwin` | `aarch64-apple-darwin` |
| Windows | `x86_64-pc-windows-msvc` | `aarch64-pc-windows-msvc` |

解压 `.tar.gz`（Linux/macOS）或 `.zip`（Windows）文件后，将 `rdstream` 或
`rdstream.exe` 放入 `PATH` 中的目录。每个版本均附带 `SHA256SUMS` 文件用于校验下载内容。
Linux 二进制要求 glibc 2.35 或更新版本；macOS 二进制以 macOS 11 或更新版本为目标。

### Cargo

使用 Rust 1.88 或更新版本从 crates.io 安装：

```sh
cargo install rdstream --locked
rdstream --help
```

从本地源码构建时，在仓库根目录执行：

```sh
cargo install --path . --locked
rdstream --help
```

默认启用并行 gzip 解压。如需构建不包含并行解压器的版本：

```sh
cargo install rdstream --locked --no-default-features
```

## 快速开始

### 1. 查看对象结构

```sh
rdstream describe GSE183852_DCM_Integrated.Robj.gz --depth 4
```

Seurat 对象树示例（部分内容省略）：

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

`describe` 默认保留不超过 **1,000,000 个元素**的向量。更大的向量只显示类型和长度，
可通过 `--threshold` 调整阈值。

### 2. 提取选定数据

```sh
rdstream extract GSE183852_DCM_Integrated.Robj.gz \
  --path '$RefMerge@meta.data*' \
  --out meta/
```

| 输出 | 内容 |
| :--- | :--- |
| `*.npy` | 整数、逻辑值、实数、复数和原始字节向量。 |
| `*.json` | 字符向量。 |
| `manifest.json` | 路径、名称、类型、长度，以及可用的 `dim`、`class` 和因子 `levels` 元数据。 |

选定向量在读取序列化流时分块写出，无需在内存中保留完整向量数据。
路径选择包含其所有下级元素，例如 `--path '$RefMerge@meta.data'` 会提取所有列。

<details>
<summary><strong>使用 Python 和 NumPy 读取提取的因子</strong></summary>

```python
import json
from pathlib import Path

import numpy as np

root = Path("meta")
manifest = json.loads((root / "manifest.json").read_text())
entry = next(e for e in manifest["entries"] if e["name"] == "Names")
codes = np.load(root / entry["file"])
levels = np.asarray(entry["levels"], dtype=object)

# R 因子从 1 开始编号；缺失值保留为 None。
cell_type = np.full(codes.shape, None, dtype=object)
valid = (codes >= 1) & (codes <= len(levels))
cell_type[valid] = levels[codes[valid] - 1]
```

</details>

### 3. 处理数据流

```sh
# 通过标准输入直接读取 GEO 下载流。
curl -sL https://ftp.ncbi.nlm.nih.gov/geo/series/GSE183nnn/GSE183852/suppl/GSE183852_DCM_Cells.Robj.gz \
  | rdstream describe - --depth 3

# 从下载流中提取细胞注释。
curl -sL https://ftp.ncbi.nlm.nih.gov/geo/series/GSE183nnn/GSE183852/suppl/GSE183852_DCM_Cells.Robj.gz \
  | rdstream extract - --path '*@meta.data' --out meta/

# 通过文件重定向启用并行解压。
rdstream describe - < GSE183852_DCM_Cells.Robj.gz

# 将归档输出暂存至 /scratch，以启用并行解压。
tar -xOf archive.tar data/seurat.rds.gz \
  | rdstream extract - --path '*@meta.data' --out meta/ --spool-dir /scratch

# 解压所有压缩层，输出原始 R 序列化流。
rdstream cat dataset.rds.gz > dataset.rds
```

管道输入的每个压缩层使用一个解压线程，解压与解析以流水线方式执行。
指定 `--spool-dir DIR` 时，rdstream 先将压缩输入复制到 `DIR` 中的临时文件，再进行并行解压。
临时文件所需空间等于压缩输入大小，处理结束后自动删除。对于高速本地输入，暂存文件可能提高吞吐量，
具体效果取决于输入和存储速度。应使用磁盘目录，以避免临时文件占用内存。
从文件重定向的标准输入（`rdstream describe - < dataset.rds.gz`）无需暂存。

## 命令参考

| 命令 | 用途 | 主要选项 |
| :--- | :--- | :--- |
| `rdstream describe <FILE>` | 显示对象树。 | `--depth`、`--threshold` |
| `rdstream extract <FILE>` | 将选定向量以流式方式写入文件。 | `--path`（可重复）、`--out`、`--threshold` |
| `rdstream cat <FILE>` | 将解压后的字节写入标准输出。 | 下方的全局解压选项。 |

所有命令都支持用 `-` 作为输入文件名来读取标准输入。
运行 `rdstream <COMMAND> --help` 查看完整选项。

### 路径与选择规则

| 片段 | 含义 | 示例 |
| :--- | :--- | :--- |
| `$name` | 带标签的 pairlist 条目，例如保存在 `.RData` 中的对象。 | `$RefMerge` |
| `@name` | 属性或 S4 槽。 | `@meta.data`、`@counts` |
| `[k]` | 列表中的第 k 个元素，下标从 0 开始。 | `[24]` |

模式中的 `*` 匹配任意长度的字符序列，包括路径分隔符；其余字符按字面匹配。
在 shell 中应使用引号包围模式，例如 `'*@meta.data*'` 或 `'$obj@assays[0]@counts*'`。
对于 `extract`，模式还会选择匹配路径下的所有元素。

列表元素通过下标定位，因为 R 在序列化元素数据之后才写入元素名称。
`describe` 会在名称可用后同时显示下标和名称，例如 `[24]condition`。

### 解压内存与线程

```sh
# 将解压器内存预算设为 1 GiB。
rdstream describe dataset.rds.gz --memory-budget 1G

# 强制单线程解压，降低内存占用。
rdstream describe dataset.rds.gz --threads 1
```

默认预算为 Linux 报告的可用内存的一半；无法读取该信息时，使用 2 GiB 作为回退预算。
预计内存占用超出预算的压缩层使用单线程解压。管道输入在未指定 `--spool-dir` 时使用单线程解压。

`--memory-budget` 约束的是解压器的内存估计，并非整个进程的内存上限。
显式指定 `--threads N` 会覆盖自动规划；大于 `1` 的值会忽略预算。

## Python 与 AnnData

Python 绑定位于 [`python/`](../python/)。在装有 Rust 工具链的环境中，从本地仓库构建并安装：

```sh
pip install "./python[anndata]"
```

原子向量以 NumPy 数组返回，直接复用解析器的缓冲区。`rdstream.anndata` 可以转换
Seurat（v3/v4 的 `Assay` 和 v5 的 `Assay5`，包括拆分的图层）和 SingleCellExperiment 对象：

```python
import rdstream
from rdstream.anndata import read

adata = read("GSE183852_DCM_Integrated.Robj.gz", assay="RNA", X="counts", layers=())
adata.write_h5ad("dcm.h5ad")

doc = rdstream.read("seurat.rds", keep=["*@meta.data*"], threshold=10_000)
meta = rdstream.as_pandas(doc.root.attr("meta.data"))
```

AnnData 读取器选择表达矩阵、元数据和降维结果进行转换，稀疏矩阵的转置不产生复制。
数据映射与选项说明见 [python/README.md](../python/README.md)。

## Rust 库

将 rdstream 添加为依赖：

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

自定义选择策略时，可传入签名为 `(path: &str, sexptype: u8, len: u64) -> bool` 的闭包。
`Policy` 还可通过 `Policy::sink` 返回 `Sink`，分块接收向量元素，无需将完整向量载入内存。
`extract::Extractor` 使用该接口写出文件。
使用 `parse_source`，并传入 `Source` 和 `DecodeOptions`，可以控制输入流、解压线程和内存预算。
通过 `cargo doc --open --no-deps` 生成本地 API 文档。

## 性能

以下数据测于 AMD EPYC 7402（24 核、48 线程、62 GB 内存、NVMe 存储）。
输入为 2.9 GB 的类 Seurat 合成序列化流，先使用 gzip `-6` 压缩，再按下表所示级别进行二次压缩。

| 外层压缩级别 | 耗时 | 内存占用 |
| :--- | ---: | ---: |
| gzip `-6` | 1.96 s | 1.2 GB |
| gzip `-1` | 2.03 s | 1.3 GB |
| gzip `-9` | 2.03 s | 1.2 GB |

上述结果仅适用于记录的测试数据与环境。实际性能和内存占用取决于输入数据、压缩方式、
保留的对象及解压配置。

## 支持的格式

| 类别 | 支持范围 |
| :--- | :--- |
| 文件类型 | `saveRDS()`（`.rds`）和 `save()`（`.RData`、`.rda`、`.Robj`）。 |
| 序列化 | 版本 2 和 3；XDR、ASCII（十进制或 `ascii = NA` 的十六进制浮点数，任意换行符）和本机二进制。 |
| 压缩 | gzip、bzip2、xz、zstd（R 4.5 的 `zstdfile()` 或 `zstd` 命令）及其嵌套组合。 |
| 对象模型 | 属性、S4 槽（含伪 NULL）、因子、数据框、含自引用的环境、符号引用、字节码编译的闭包和长向量。 |
| ALTREP | 紧凑序列、wrapper 和延迟字符串；未知类保留原始序列化状态。 |
| 字符串 | UTF-8、Latin-1，以及本机编码为 Latin-1 或 Windows-1252 时未标记编码的字符串。 |

### 当前限制

- 本机二进制格式按小端序读取；R 不会记录写入端的字节序。
- 管道输入需通过 `--spool-dir` 暂存至磁盘后才能并行解压。
- 解压内存预算为估算值，不构成严格上限，且不包含保留对象的内存占用。

## 路线图

- [x] 使用 PyO3 提供 Python 绑定，以及 Seurat / SingleCellExperiment → AnnData 转换。
- [x] 解析时将选定向量直接写入磁盘。
- [x] 改进重复压缩外层的并行解压。
- [x] 在 CI 中加入由 R 生成的测试样本。
- [x] 支持 ASCII 序列化。
- [x] 支持 zstd 压缩。
- [x] 支持有限缓冲区的数据流解压，以及文件重定向或磁盘暂存标准输入的并行解压。

## 参与贡献

可通过问题报告或 Pull Request 参与贡献。报告解析问题时，请提供执行命令、错误信息、
已知的 R 版本与保存格式，以及可获取的最小复现输入。

分别在启用和禁用并行 gzip 的配置下运行测试，并运行 Python 测试：

```sh
cargo test --locked
cargo test --locked --no-default-features
pip install "./python[test]" && pytest python/tests
```

第三方测试样本的来源与内容记录在 [tests/fixtures/MANIFEST.md](../tests/fixtures/MANIFEST.md)。
CI 还会运行 [tests/r/generate.R](../tests/r/generate.R) 生成测试样本，并将结果与 R 的输出进行校验。
本地验证命令如下：

```sh
Rscript tests/r/generate.R target/r-fixtures
RDSTREAM_R_FIXTURES=target/r-fixtures cargo test --test r_generated
RDSTREAM_R_FIXTURES=target/r-fixtures pytest python/tests/test_r_generated.py
```

<details>
<summary><strong>相关项目</strong></summary>

其他处理 R 序列化数据的工具包括
[rds2rust](https://github.com/andrewwbutler/rds2rust)、
[rds2cpp](https://github.com/LTLA/rds2cpp) /
[rds2py](https://pypi.org/project/rds2py/)、
[rdata](https://github.com/vnmabus/rdata) 和
[readseurat](https://github.com/harryhaller001/readseurat)。
[rapidgzip](https://github.com/mxmlnkn/rapidgzip) 提供 gzip 并行解压实现。

</details>

## 许可证

本项目采用 [MIT](../LICENSE-MIT) 或 [Apache License 2.0](../LICENSE-APACHE) 双许可证，
使用者可选择其中任一许可证。
