# 伴随模型框架

伴随模型（Companion Model）是 Tiny-SPICE 处理非线性和储能元件的核心抽象。每个非线性/储能元件通过 `linearize()` 方法输出一对线性等效参数 \\((G_{eq}, I_{eq})\\)，然后复用线性元件的 Stamp 函数。

## 统一接口

```rust
fn linearize(&self, state: f64, dt: f64) -> (f64, f64) {
    // 返回 (g_eq, i_eq)
    // g_eq: 等效电导
    // i_eq: 等效电流源
}
```

## 电容：后向欧拉离散化

电容的微分方程 \\(I = C \\cdot \\frac{dV}{dt}\\) 使用后向欧拉法离散：

$$I^{n} = C \\cdot \\frac{V^{n} - V^{n-1}}{\\Delta t}$$

改写为诺顿等效形式：

$$I^{n} = \\underbrace{\\frac{C}{\\Delta t}}_{G_{eq}} \\cdot V^{n} - \\underbrace{\\frac{C}{\\Delta t} \\cdot V^{n-1}}_{I_{eq}}$$

```rust
pub fn linearize(&self, v_prev: f64, t_delta: f64) -> (f64, f64) {
    let g_eq = self.value / t_delta;
    let i_eq = g_eq * v_prev;
    (g_eq, i_eq)
}
```

## 二极管：Newton-Raphson + Colon 限制

二极管的指数特性 \\(I_D = I_S (e^{V_D/V_T} - 1)\\) 在每个迭代步做一阶泰勒展开：

$$I_D^{k+1} \\approx I_D^{k} + \\frac{\\partial I_D}{\\partial V_D}\\bigg|_k \\cdot (V_D^{k+1} - V_D^{k})$$

整理为伴随模型：

$$G_{eq} = \\frac{I_S}{V_T} e^{V_D/V_T}$$

$$I_{eq} = I_D - G_{eq} \\cdot V_D$$

### Colon 电压限制法

为防止 Newton-Raphson 迭代因指数增长而发散，Tiny-SPICE 实现了 Colon 电压限制法（参见 Nagel 论文）。核心思想：

- 当电压变化超过 \\(2V_T\\) 时，限制电压步长
- 在反向偏置区使用对数外推

```rust
// 简化伪代码
if v_hat < v_crit {
    v_d_i = v_hat;  // 反向偏置：直接使用
} else if delta.abs() <= 2.0 * v_thermal {
    v_d_i = v_hat;  // 小步长：直接使用
} else {
    v_d_i = v_thermal * (v_hat / v_thermal).ln();  // 大正向步：对数限制
}
```

## 收敛判断

每轮 Newton-Raphson 迭代后检查：

1. 所有节点电压的变化量是否小于 `VNTOL`
2. 所有支路电流的变化量是否小于 `ABSTOL`
3. 迭代次数是否超过 `ITL4` 上限

## 时变源

正弦源（`VSIN`/`ISIN`）和分段线性源（`VPWL`）通过 `evaluate(t)` 接口在每个时间步计算当前值，然后作为普通 DC 源 Stamp 到矩阵。

```rust
pub fn evaluate(&self, t: f64) -> f64 {
    self.vo + self.va * (2.0 * PI * self.freq * t).sin()
}
```
