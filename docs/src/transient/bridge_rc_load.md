# Bridge RC Load — 桥式整流 + RC 负载

**测试文件**: `tests/test_trans_ir_bridge_rc_load.rs`
**测试函数**: `test_trans_ir_bridge_rc_1kHz`, `test_trans_ir_bridge_rc_failure_003`

## 电路

在桥式整流器基础上增加：
- 输出端并联 1µF 滤波电容
- 每个二极管并联 1pF 寄生电容

```
    Isin → 桥式整流器 → Rload(1kΩ) || Cload(1µF)
                        + 4 × C_parasitic(1pF)
```

**程序化网表**（简化）:
```spice
Isin 0 1 SIN(0, amp, f)
R1 1 0 10
Vmeas 2 0 0
D1 1 3 ...  C1 1 3 1pF  ; 二极管 + 寄生电容
D2 4 1 ...  C2 4 1 1pF
D3 2 3 ...  C3 2 3 1pF
D4 4 2 ...  C4 4 2 1pF
Rload 3 4 1000
Cload 3 4 1e-6
```

## 解析

- 这是**最复杂**的测试用例
- 5 个电容 + 4 个二极管 + 正弦源 = 高度非线性 + 多储能元件
- `test_trans_ir_bridge_rc_failure_003` 专门复现一个已知的收敛失败场景

### 已知问题

`test_trans_ir_bridge_rc_failure_003` 复现了特定参数组合下的收敛失败（amp=-2, 3kHz, cap=1µF），是后续优化的目标。
