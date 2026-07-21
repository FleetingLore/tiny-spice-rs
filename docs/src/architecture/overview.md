# 整体概览

Tiny-SPICE 采用**管道式架构 + trait 解耦**。核心引擎不包含任何解析代码，输入和输出均通过 trait 抽象。

## Crate 职责矩阵

| 层 | Crate | 外部依赖 | 核心 trait |
|---|---|---|---|
| 核心引擎 | `spice-core` | **无** | 定义 `CircuitSource`, `Reporter` |
| 解析器 | `spice-parser` | `spice-core` | 实现 `CircuitSource`（SPICE 语法） |
| 输出 | `spice-tui` | `spice-core` + `indicatif` | 实现 `Reporter`（进度条） |
| 组装 | `spice-cli` | `spice-core` + `spice-parser` + `spice-tui` | 选实现 + 注入 |

## 数据流

```mermaid
flowchart LR
    P[spice-parser<br/>CircuitSource impl] -->|Circuit + Config| G[spice-core::engine<br/>MNA 引擎]
    G -->|via Reporter| T[spice-tui<br/>Reporter impl]
    G -->|waveform| W[wavewriter]
```

## 模块依赖

```mermaid
flowchart TD
    CLI[spice-cli] --> Core[spice-core]
    CLI --> Parser[spice-parser]
    CLI --> TUI[spice-tui]
    Parser --> Core
    TUI --> Core

    Core --> Engine[engine]
    Core --> Element[element/]
    Core --> Circuit[circuit]
    Core --> Expander[expander]
    Core --> Analysis[analysis]
    Core --> Wave[wavewriter]
    Core --> Source[source]
    Core --> Report[report]
    Core --> Bracket[bracket_expression]

    Engine --> Element
    Engine --> Circuit
    Engine --> Analysis
    Engine --> Wave
    Engine --> Report

    Expander --> Circuit
    Expander --> Element
    Expander --> Bracket
```

## 关键设计决策

1. **零依赖核心** — `spice-core` 不依赖任何外部 crate，可直接嵌入其他项目
2. **Trait 解耦** — 输入（`CircuitSource`）和输出（`Reporter`）均通过 trait 抽象，可自由替换
3. **解析器外置** — SPICE 语法解析在独立 crate `spice-parser` 中，未来可加 JSON/YAML 等格式
4. **NodeId = usize** — 节点直接用整数索引，节点 0 硬编码为地，简化矩阵寻址
5. **统一 Stamp 接口** — 所有元件通过 `stamp_resistor`、`stamp_voltage_source`、`stamp_current_source` 贡献到 MNA
6. **伴随模型** — 非线性/储能元件通过 `linearize()` 输出 `(g_eq, i_eq)`，复用线性 Stamp
7. **一次性展平** — 子电路在解析阶段递归展平，引擎面对扁平电路
