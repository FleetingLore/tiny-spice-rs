# 命令行集成测试

**测试文件**: `tests/cmdline_tests.rs`
**测试函数**: `spice_reader`, `spice_irrrr`, `spice_irrc`

## 说明

三个端到端集成测试，通过命令行运行编译好的二进制文件。

## 测试

### `spice_reader`

```bash
./target/debug/tiny-spice-rs ./ngspice/test_reader.spi
```

验证二进制文件能正常启动并读取 SPICE 网表。

### `spice_irrrr`

```bash
./target/debug/tiny-spice-rs ./ngspice/test_irrrr.spi
```

通过命令行运行 IRRRR 电阻网络测试，验证端到端流程：
解析 → 展平 → DC 求解 → 输出。

### `spice_irrc`

```bash
./target/debug/tiny-spice-rs ./ngspice/test_irrc.spi
```

运行 RC 高通滤波器瞬态分析，验证完整的：
解析 → 展平 → 瞬态求解 → 波形输出。

## 实现

三个测试使用自定义宏 `spice!` 启动子进程：

```rust
macro_rules! spice {
    ($spice_file:expr) => {
        Command::new("target/debug/tiny-spice-rs")
            .arg($spice_file)
            .output()
    };
}
```
