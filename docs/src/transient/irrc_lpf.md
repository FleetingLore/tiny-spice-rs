# IRRC LPF — RC 低通滤波器频响

**测试文件**: `tests/test_trans_irrc.rs`
**测试函数**: `test_irrc_trans_1kHz` ~ `10kHz`

## 电路

```
    Isin (正弦)
    ┌──→──┐
    │     │
    0     1
    │     ├── R_shunt(1Ω) ── 0
    │     │
    │     └── R_series(1kΩ) ── 2 ── C(0.032µF) ── 0
    │
    └────────────────────────────────────────────┘
```

**程序化网表**:
```spice
Isin 0 1 SIN(0, 10, f)
Rshunt 1 0 1
Rseries 1 2 1000
Cshunt 2 0 0.032e-6
```

## 解析

一阶 RC 低通滤波器，截止频率：

$$f_c = \\frac{1}{2\\pi R C} = \\frac{1}{2\\pi \\times 1000 \\times 32 \\times 10^{-9}} \\approx 5\\text{kHz}$$

四个频率变体测试滤波器的频响：

| 测试 | 频率 | 预期 |
|------|------|------|
| 1 kHz | 远低于 \\(f_c\\) | 几乎无衰减 |
| 2 kHz | 低于 \\(f_c\\) | 轻微衰减 |
| 5 kHz | 等于 \\(f_c\\) | -3dB 衰减 |
| 10 kHz | 高于 \\(f_c\\) | 明显衰减 |

电容的 Backward Euler 离散化使 \\(G_{eq} = C/\\Delta t\\)，在瞬态中电容不再是开路，而是动态变化的等效电导。

## 验证

与 Ngspice 波形对比。
