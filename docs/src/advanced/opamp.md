# OpAmp 宏模型

**测试文件**: `tests/test_opamp_basic.rs`
**测试函数**: `test_opamp_basic`

## 电路

从 `ngspice/opamp_basic.spi` 读取运放宏模型。

```
    VSIN (hack: DC=6V)
    ┌──┤V├── in ─── 运放宏模型 ──┬── out1 (VCCS×10)
    │                           └── out2 (VCVS×2)
    0
```

宏模型内部包含 VCCS 和 VCVS 实现运放行为。

## 解析

测试步骤：
1. 读取外部 SPICE 文件加载运放宏模型
2. 将正弦源的 DC 偏移 hack 为 6V
3. 运行 DC 工作点
4. 验证宏模型内部节点的电压

$$
V_{in} = 6\\text{V}
$$
$$
V_{out1} = V_{in} \\times 10 = 60\\text{V} \\quad \\text{(VCCS 增益)}
$$
$$
V_{out2} = V_{in} \\times 2 = 12\\text{V} \\quad \\text{(VCVS 增益)}
$$

## 验证

```rust
assert_nearly(v_in, 6.0);
assert_nearly(v_out1, -60.0);  // 注意极性
assert_nearly(v_out2, 12.0);
```

这是验证**子电路展平**和**外部网表读取**的端到端测试。
