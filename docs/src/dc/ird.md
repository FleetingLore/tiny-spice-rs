# IRD — 电流源 + 电阻 + 二极管

**测试文件**: `tests/test_ird.rs`
**测试函数**: `test_dc_ird`, `test_dc_ird_isat_1pA` ⚠️

## 电路

```
    I1 (3A)
    ┌──→──┐
    │     │
    0     1
    │     ├──┤R├──┐
    │     │ 10Ω  │
    │     │      │
    │     ├──┤D├──┤
    │        正向  │
    └─────────────┘
          0
```

**网表**:
```spice
I1 0 1 3.0
r1 1 0 10
D1 1 0 1e-9 27
```

## 解析

- 节点数：2
- 电压源数：0
- 非线性二极管引入 Newton-Raphson 迭代

正向偏置的二极管与电阻并联分流。二极管饱和电流 `Is` 影响导通电压：

$$V_D = V_T \\ln\\left(\\frac{I_D}{I_S} + 1\\right)$$

两个变体测试不同 `Is` 值：
- `test_dc_ird`: Is = 1e-9 → VD ≈ 0.73V
- `test_dc_ird_isat_1pA`: Is = 1e-12 → VD ≈ 0.91V（⚠️ 已知精度问题）

## 验证

```rust
// test_dc_ird: Is=1e-9
assert_nearly(v[1], 0.73217);
```

### ⚠️ 已知问题

`test_dc_ird_isat_1pA` 因数值精度问题未通过，返回约 30V 而非预期的 0.91V。这可能是 Colon 限制或收敛参数的调优问题。
