# 测试总览

本项目包含 **22 个测试用例**，覆盖从最基础的 DC 工作点到复杂瞬态分析的各个层次。

## 测试分类

```
DC 工作点 (11)
├── 纯电阻网络:     IR, VRRR, IRRRR
├── 二极管电路:     IRD, IR_DREV, V_D_VS_D
├── 压控源:         VC_VS_CS
├── 桥式整流:       Bridge Loaded, Bridge Unloaded
└── 多分支测量:     IRDVV, IRRC

瞬态分析 (8)
├── 基础瞬态:       Transient IR
├── 正弦激励:       IR Sine, IRD Sine, IRRD Sine
├── RC 滤波器:      IRRC LPF, IRRC HPF
└── 桥式整流瞬态:   Bridge Loaded, Bridge RC Load

集成测试 (3)
├── 宏模型:         OpAmp
├── 子电路库:       Library Reading
└── 命令行:         Cmdline Tests
```

## 学习路径建议

1. 从 **DC 工作点** 的纯电阻网络开始，理解 MNA 矩阵构建和求解
2. 加入**二极管**，理解非线性迭代和伴随模型
3. 深入**桥式整流**，理解多二极管耦合
4. 进入**瞬态分析**，理解时间步进和储能元件
5. 最后看**集成测试**，理解子电路展平和宏模型

## 命名约定

| 前缀 | 含义 |
|------|------|
| `test_ir_*` | 电流源 (I) + 电阻 (R) |
| `test_ird_*` | 电流源 + 电阻 + 二极管 (D) |
| `test_irrc_*` | 电流源 + 电阻 + 电阻 + 电容 (C) |
| `test_irrd_*` | 电流源 + 电阻 + 电阻 + 二极管 |
| `test_trans_*` | 瞬态分析 (Transient) |
| `test_v_*` | 电压源 (V) 驱动 |
