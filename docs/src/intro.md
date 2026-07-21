# 简介

**Tiny-SPICE** 是一个用 Rust 编写的微型 SPICE（Simulation Program with Integrated Circuit Emphasis）电路仿真器。

## 特性

- **完整的 DC 工作点分析** — 求解电路的静态工作点
- **瞬态分析** — 模拟电路随时间变化的行为
- **改进节点分析法 (MNA)** — 手写高斯消元线性求解器
- **伴随模型框架** — 统一处理线性和非线性元件
- **子电路展平** — 支持层次化 SPICE 网表
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

## 快速开始

```bash
# 编译并运行示例
make release
./target/release/tiny-spice-rs ngspice/test_vvrrr.spi

# 运行全部测试
make test

# 构建文档
make docs
```
