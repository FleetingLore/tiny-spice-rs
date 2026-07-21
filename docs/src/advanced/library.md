# 子电路库读取

**测试文件**: `tests/test_library_reading.rs`
**测试函数**: `test_library_reading`

## 说明

测试 SPICE 的 `.lib` / `.endl` 库定义解析。

从 `ngspice/drum-machine/libtest.spi` 读取。

## 测试内容

```spice
.lib mylib
.subckt X1 ...
.ends
.endl mylib
```

验证解析器能够：
1. 正确识别 `.lib <name>` 和 `.endl <name>` 边界
2. 展开库中的子电路
3. 展开后的顶层电路结构和节点数

## 验证

```rust
assert_eq!(ckt.count_nodes(), 8);
assert_eq!(ckt.count_voltage_sources(), 1);
// 加上子电路展平后的元件总数
assert_eq!(ckt.elements.len(), 13);
```

这是验证**子电路展平**正确性的关键测试——确保层次化网表能正确展平为扁平的 13 个基本元件。
