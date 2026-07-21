# 整体概览

Tiny-SPICE 采用经典的**管道式架构**，数据从 SPICE 网表流入，经过解析、展开、仿真，最终输出波形。

## 数据流

```mermaid
flowchart LR
    A[SPICE 网表] --> B[解析器<br/>spice.rs]
    B --> C[子电路展开器<br/>expander.rs]
    C --> D[仿真引擎<br/>engine.rs]
    D --> E[波形输出<br/>wavewriter.rs]
```

## 模块职责

| 模块 | 文件 | 职责 |
|------|------|------|
| 解析器 | `spice.rs` | 读取 SPICE 网表，词法/语法分析，构建 `Circuit` 结构 |
| 展开器 | `expander.rs` | 递归展平子电路，解析参数，生成扁平元件列表 |
| 电路模型 | `circuit.rs` | 电路数据结构（节点 LUT、元件列表、子电路实例） |
| 元件模型 | `element/` | 各类元件的结构体和伴随模型实现 |
| 仿真引擎 | `engine.rs` | MNA 矩阵构建、Stamp、求解、收敛判断 |
| 分析配置 | `analysis.rs` | 仿真参数（步长、容差、迭代上限等） |
| 波形输出 | `wavewriter.rs` | 将每个时间步的节点电压写入文件 |
| bracket表达式 | `bracket_expression.rs` | 解析 `{expression}` 格式的参数表达式 |

## 元件模型目录

```
element/
├── mod.rs          # Element 枚举 + 统一 Display
├── resistor.rs     # 电阻
├── capacitor.rs    # 电容（Backward Euler 离散化）
├── diode.rs        # 二极管（Newton-Raphson + Colon 限制）
├── independent.rs  # 独立 DC 源
├── isine.rs        # 正弦电流源
├── vsine.rs        # 正弦电压源
├── vpwl.rs         # 分段线性电压源
└── vdepsrc.rs      # VCVS / VCCS
```

## 关键设计决策

1. **NodeId = usize**：节点直接用整数索引，节点 0 硬编码为地（gnd），简化矩阵寻址
2. **统一 Stamp 接口**：所有元件最终通过 `stamp_resistor`、`stamp_voltage_source`、`stamp_current_source` 贡献到 MNA 矩阵
3. **伴随模型**：非线性/储能元件通过 `linearize()` 输出 `(g_eq, i_eq)`，然后复用线性元件的 Stamp
4. **一次性展平**：子电路在解析阶段递归展平，引擎面对的是完全扁平的电路
