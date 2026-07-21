# 简介

**Tiny-SPICE** 是一个用 Rust 编写的微型 SPICE（Simulation Program with Integrated Circuit Emphasis）电路仿真器。

## 特性

- **完整的 DC 工作点分析** — 求解电路的静态工作点
- **瞬态分析** — 模拟电路随时间变化的行为
- **改进节点分析法 (MNA)** — 手写高斯消元线性求解器
- **伴随模型框架** — 统一处理线性和非线性元件
- **子电路展平** — 支持层次化 SPICE 网表
- **可插拔架构** — 解析器、输出后端均通过 trait 解耦
- **约 2000 行 Rust** — 极简实现，适合学习电路仿真原理

## 支持的元件

| 类型 | 标识符 | 说明 |
|------|--------|------|
| 电阻 | `R` | 线性电阻 |
| 电容 | `C` | 储能元件（后向欧拉离散化） |
| 二极管 | `D` | 非线性元件（Newton-Raphson + Colon 电压限制） |
| 独立电流源 | `I` | DC 电流源 |
| 独立电压源 | `V` | DC 电压源 |
| 正弦电流源 | `ISIN` | 时变正弦电流源 |
| 正弦电压源 | `VSIN` | 时变正弦电压源 |
| 分段线性电压源 | `VPWL` | 自定义波形电压源 |
| VCVS | `E` | 电压控制电压源 |
| VCCS | `G` | 电压控制电流源 |

## 限制

- 节点上限硬编码为 256（仅限教学用途）
- 不支持电感、MOSFET、BJT
- 求解器为基本高斯消元（无稀疏矩阵优化）

## 项目结构（Cargo Workspace）

```
spice-tiny-fork/
├── Cargo.toml              # workspace 根，members = ["spice-core", "spice-tui", "spice-cli"]
├── spice-core/             # 核心库 crate（零外部依赖）
├── spice-tui/              # TUI 输出后端 crate（依赖 indicatif）
├── spice-cli/              # CLI 入口 crate（组装 core + tui）
├── tests/                  # 集成测试（目标 spice-core）
├── ngspice/                # 示例 SPICE 网表
├── docs/                   # mdBook 文档
├── Makefile                # 构建脚本
└── .github/workflows/      # CI + 文档部署
```

### `spice-core` — 核心库

所有仿真逻辑所在地，**零外部依赖**。如果你只需要仿真引擎而不需要 CLI 或 TUI，只依赖这个 crate 即可。

| 模块 | 职责 |
|------|------|
| `engine` | MNA 引擎：矩阵构建、Stamp、高斯消元求解、收敛判断 |
| `element/` | 元件模型：R / C / D / V / I / VCVS / VCCS / VSIN / ISIN / VPWL |
| `circuit` | 电路数据结构：节点 LUT、元件列表、子电路实例 |
| `spice` | SPICE 网表解析器，实现 `CircuitSource` trait |
| `expander` | 子电路递归展平 |
| `analysis` | 仿真配置（步长、容差、迭代上限等） |
| `wavewriter` | 波形数据写入文件 |
| `bracket_expression` | 解析 `{param}` 和工程记数法 (`10k`, `4.7u`) |
| `source` | `CircuitSource` trait —— 电路来源的抽象接口 |
| `report` | `Reporter` trait —— 仿真进度/诊断输出的抽象接口 |

**三个核心 trait：**

```rust
// 电路输入 —— SPICE 只是实现之一
pub trait CircuitSource {
    fn read(&mut self, path: &Path) -> bool;
    fn get_expanded_circuit(&self) -> Circuit;
    fn configuration(&self) -> &Configuration;
}

// 仿真进度/诊断 —— 终端输出、TUI 进度条只是实现之二
pub trait Reporter {
    fn banner(&self);
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
    fn begin_transient(&self, tstop: f64);
    fn tick_transient(&self, t_now: f64, t_stop: f64);
    fn finish_transient(&self);
}
```

`Engine` 本身不依赖任何具体实现，只接收 `&Circuit` + `&Configuration` 并通过 `Reporter` 输出。

### `spice-tui` — TUI 输出后端

为瞬态分析提供**进度条**，依赖 `indicatif`。

```rust
pub struct TuiReporter { bar: RefCell<Option<ProgressBar>> }
impl Reporter for TuiReporter { ... }
```

瞬态分析运行时会显示：
```
⠁ [00:00:03] [████████████░░░░░░░░░░░░░░░░░░░░░░] 30% transient analysis...
```

### `spice-cli` — 命令行入口

组装所有组件，**所有写死的逻辑只在这里**：

```rust
fn main() {
    // 1. 根据文件扩展名选 CircuitSource
    let source: Box<dyn CircuitSource> = match ext {
        "spi" | "cir" => Box::new(spice::Reader::new()),
        _ => ...
    };
    // 2. 注入 Reporter
    eng.set_reporter(Box::new(TuiReporter::new()));
    // 3. 运行
    eng.go(&ckt, cfg);
}
```

未来添加 JSON/YAML 格式的电路输入，只需新增一个 `impl CircuitSource` 的 crate，CLI 加一行 match。

## 快速开始

```bash
# 编译并运行示例
make release
./target/release/spice_cli ngspice/test_vvrrr.spi

# 仅构建核心库（零依赖）
cargo build -p tiny_spice

# 运行全部测试
make test

# 构建文档
make docs
```
