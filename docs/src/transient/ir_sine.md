# IR Sine — 正弦电流源 + 电阻

**测试文件**: `tests/test_trans_ir_sine.rs`
**测试函数**: `test_trans_ir_sine_10Hz`, `test_trans_ir_sine_1kHz`

## 电路

```
    Isin (正弦)
    ┌──→──┐
    │     │
    0     1
    │     │
    │    R1(10Ω)
    │     │
    └─────┘
    0
```

**程序化网表**:
```spice
Isin 0 1 SIN(3, 1, f)
R1 1 0 10
```

## 解析

验证正弦源的 `evaluate(t)` 在不同频率下的正确性：

$$V_1(t) = R \\times [I_{offset} + I_{amp} \\cdot \\sin(2\\pi f t)]$$

$$V_1(t) = 10 \\times [3 + 1 \\cdot \\sin(2\\pi f t)]$$

两个变体：
- **10 Hz**：慢速正弦，容易跟踪
- **1 kHz**：快速正弦，考验时间步自适应

## 验证

在每个时间步检查 \\(V_1(t)\\) 是否符合预期正弦值。
