# IRD Sine — 正弦源 + 电阻 + 二极管

**测试文件**: `tests/test_trans_ird_sine.rs`
**测试函数**: `test_trans_ir_sine_1kHz_10us`, `test_trans_ir_sine_1kHz_1us`

## 电路

```
    Isin (正弦)
    ┌──→──┐
    │     │
    0     1
    │     ├── R1(10Ω) ── 0
    │     │
    │     └── D1(→) ── 0
    │
    └─────────────────┘
```

**程序化网表**:
```spice
Isin 0 1 SIN(0, amp, f)
R1 1 0 10
D1 1 0 1e-9 27
```

## 解析

第一个包含**非线性元件 + 时变源**的瞬态测试。

- 正弦电流在正半周时二极管导通，分流电阻
- 负半周时二极管截止，全部电流流经电阻
- 涉及每时间步的 Newton-Raphson 迭代

两个变体测试不同时间步长：
- **10 µs**：大步长，测试收敛鲁棒性
- **1 µs**：小步长，测试精度

## 验证

比较每个时间步的 \\(V_1\\) 与 Ngspice 参考值（通过 `--features ngspice_compare`）。
