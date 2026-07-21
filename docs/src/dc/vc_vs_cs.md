# VC_VS_CS — 压控源 (VCVS/VCCS)

**测试文件**: `tests/test_vc_vs_cs_basic.rs`
**测试函数**: `test_vc_vs_cs_basic`

## 电路

从 `ngspice/vc_vs_cs_basic.spi` 读取。

```
    Isrc (SIN → 3A DC)
    ┌──→──┐
    │     │
    0    ctl ─── R1(1Ω) ─── 0
    │     │
    │     ├──┤G├── n1 ─── Rc(1Ω) ─── 0
    │     │ VCCS(k=2)
    │     │
    │     ├──┤E├── n2 ─── Rv(10kΩ) ─── 0
    │       VCVS(k=3)
    │
    └──────────────────────────┘
```

**网表**:
```spice
Isrc 0 ctl SIN(0,3,800)
R1 ctl 0 1
Gvccs 0 n1 ctl 0 2
Rc n1 0 1
Evcvs n2 0 ctl 0 3
Rv n2 0 10k
```

## 解析

测试将正弦电流源 hack 为 3A DC 偏移，然后验证两个压控源：

- **VCCS (G)**: \\(I_{out} = k \\cdot V_{ctl}\\)，k=2
  - \\(V_{ctl} = 3A \\times 1\\Omega = 3V\\)
  - \\(I_{out} = 2 \\times 3 = 6A\\)
  - \\(V_{n1} = 6A \\times 1\\Omega = 6V\\)

- **VCVS (E)**: \\(V_{out} = k \\cdot V_{ctl}\\)，k=3
  - \\(V_{n2} = 3 \\times 3 = 9V\\)

## 验证

```rust
assert_nearly(v_ctl, 3.0);
assert_nearly(v_n1, 6.0);
assert_nearly(v_n2, 9.0);
```
