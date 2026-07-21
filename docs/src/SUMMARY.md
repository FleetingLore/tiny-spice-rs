# 目录

[简介](./intro.md)

# 项目架构

- [整体概览](./architecture/overview.md)
- [核心算法：改进节点分析法 (MNA)](./architecture/mna.md)
- [伴随模型框架](./architecture/companion.md)
- [子电路展平](./architecture/expansion.md)

# 测试用例

- [测试总览](./tests/overview.md)

## DC 工作点

- [IR — 电流源 + 电阻](./dc/ir.md)
- [VRRR — 双电压源 + 电阻网络](./dc/vvrrr.md)
- [IRRRR — 电流源 + 四电阻梯形网络](./dc/irrrr.md)
- [IRD — 电流源 + 电阻 + 二极管](./dc/ird.md)
- [IR_DREV — 反向偏置二极管](./dc/ir_drev.md)
- [V_D_VS_D — 电压源 + 正反二极管对](./dc/v_d_vs_d.md)
- [VC_VS_CS — 压控源 (VCVS/VCCS)](./dc/vc_vs_cs.md)
- [Diode Bridge (Loaded) — 二极管桥式整流（带载）](./dc/bridge_loaded.md)
- [Diode Bridge (Unloaded) — 二极管桥式整流（空载）](./dc/bridge_unloaded.md)
- [IRDVV — 零伏源电流测量](./dc/irdvv.md)
- [IRRC — RC 低通滤波器（DC 视角）](./dc/irrc.md)

## 瞬态分析

- [Transient IR — 电阻瞬态响应](./transient/ir.md)
- [IR Sine — 正弦电流源 + 电阻](./transient/ir_sine.md)
- [IRD Sine — 正弦源 + 电阻 + 二极管](./transient/ird_sine.md)
- [IRRD Sine — 正弦源 + 分压电阻 + 二极管](./transient/irrd_sine.md)
- [IRRC LPF — RC 低通滤波器频响](./transient/irrc_lpf.md)
- [IRRC HPF — RC 高通滤波器频响](./transient/irrc_hpf.md)
- [Bridge Loaded — 桥式整流瞬态](./transient/bridge_loaded.md)
- [Bridge RC Load — 桥式整流 + RC 负载](./transient/bridge_rc_load.md)

## 集成测试

- [OpAmp 宏模型](./advanced/opamp.md)
- [子电路库读取](./advanced/library.md)
- [命令行集成测试](./advanced/cmdline.md)
