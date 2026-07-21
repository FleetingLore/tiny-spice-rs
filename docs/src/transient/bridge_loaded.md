# Bridge Loaded — 桥式整流瞬态

**测试文件**: `tests/test_trans_ir_bridge_loaded.rs`
**测试函数**: `test_trans_ir_bridge_1kHz_10us`, `test_trans_ir_bridge_1kHz_1us`

## 电路

正弦电流源驱动的全波桥式整流器 + 电阻负载。

```
    Isin (正弦)
    ┌──→──┐
    │     │
    0     1
    │     ├── R1(10Ω) ── 0
    │     │
    │     ├── Vmeas(0V) ── 2 ── ... 桥式整流器 ... ── Rload(1kΩ)
    │
    └──────────────────────────────────────────────────┘
```

**程序化网表**（简化）:
```spice
Isin 0 1 SIN(0, amp, f)
R1 1 0 10
Vmeas 2 0 0
D1 1 3 ... D2 4 1 ... D3 2 3 ... D4 4 2 ...
Rload 3 4 1000
```

## 解析

桥式整流器瞬态分析是最复杂的测试之一：

- 4 个二极管交替导通/截止
- 正弦输入正半周：D1、D4 导通
- 正弦输入负半周：D2、D3 导通
- 输出为"馒头波"（全波整流波形）
- 每时间步需解 4 个非线性二极管

## 验证

与 Ngspice 波形对比。`test_trans_ir_bridge_loaded_loop` (ignored) 可对参数空间做扫描。
