# Diode Bridge (Loaded) — 二极管桥式整流（带载）

**测试文件**: `tests/test_dc_bridge_p_loaded.rs`
**测试函数**: `test_dc_bridge_loaded_2v0`, `test_dc_bridge_loaded_gnd`

## 电路

全波桥式整流器，四个二极管 + 1kΩ 负载电阻。

```
       D1 (→)       D2 (←)
    1 ──→── 3 ──→── 4 ──←──
    │        │        │     │
   V1       Rload     │    V1
  (10V)    (1kΩ)      │   (10V)
    │        │        │     │
    2 ──←── 3 ──←── 4 ──→──
       D3 (←)       D4 (→)
```

**网表 (floating source)**:
```spice
V1 1 2 10
V2 2 0 0
D1 1 3 1e-9 27
D2 4 1 1e-9 27
D3 2 3 1e-9 27
D4 4 2 1e-9 27
R1 3 4 1000
```

**网表 (grounded source)**:
```spice
V1 1 0 10
D1 1 3 1e-9 27
D2 2 1 1e-9 27
D3 0 3 1e-9 27
D4 2 0 1e-9 27
r1 3 2 1000
```

## 解析

两个变体：
1. **浮地电压源** + 零伏参考源 (V2=0)：模拟隔离电源
2. **接地电压源**：V1 直接接地

DC 工作点下，D1 和 D4 正向导通，D2 和 D3 反向截止。输出电压：

$$V_{out} = V_{in} - 2V_D \\approx 10 - 2 \\times 0.73 \\approx 8.54\\text{V}$$

## 验证

```rust
// 浮地版
assert_nearly(v[3] - v[4], 8.52);
// 接地版
assert_nearly(v[3] - v[2], 8.52);
```
