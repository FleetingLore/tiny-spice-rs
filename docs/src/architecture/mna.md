# 改进节点分析法 (MNA)

改进节点分析法（Modified Nodal Analysis）是 SPICE 类仿真器的核心算法。它将基尔霍夫电流定律（KCL）扩展为包含电压源支路电流的线性方程组。

## 基本原理

对于包含 N 个节点和 M 个电压源的电路，MNA 构建一个 \\((N+M) \\times (N+M)\\) 的矩阵：

```
┌──────────┬──────────┐   ┌─────┐   ┌───────┐
│    G     │    B     │   │  V  │   │   I   │
│  (N×N)   │  (N×M)   │ × │     │ = │       │
├──────────┼──────────┤   │ (N+M)│   │ (N+M) │
│    C     │    D     │   │      │   │       │
│  (M×N)   │  (M×M)   │   │  J  │   │   E   │
└──────────┴──────────┘   └─────┘   └───────┘
```

- **G 块**：节点电导矩阵（电阻/电导贡献）
- **B 块**：电压源对节点的连接关系（±1）
- **C 块**：B 的转置
- **D 块**：通常为零矩阵
- **V**：未知节点电压
- **J**：未知电压源电流
- **I**：已知电流源注入
- **E**：已知电压源值

## Stamp 机制

Tiny-SPICE 通过三个统一的 Stamp 函数将元件"印"到矩阵上：

### `stamp_resistor(m, a, b, g)`
将电导 g 加到矩阵的 G 块：

```
G[a][a] +=  g
G[b][b] +=  g
G[a][b] -=  g
G[b][a] -=  g
```

### `stamp_voltage_source(m, vsrc, idx)`
将电压源加到 B/C 块和已知向量：

```
B[a][idx] +=  1
B[b][idx] -=  1
C[idx][a] +=  1
C[idx][b] -=  1
knowns[idx] = vsrc.value
```

### `stamp_current_source(m, isrc)`
将电流源加到已知向量：

```
knowns[a] -= isrc.value
knowns[b] += isrc.value
```

## 求解器

Tiny-SPICE 使用手写的**带部分主元的高斯消元法**：

1. 对每一列，找到绝对值最大的行（部分主元）
2. 交换当前行和主元行
3. 用当前行消去下方所有行的该列

```rust
// 伪代码
for pivot in 0..size {
    // 找主元
    let max_row = find_max_in_column(pivot);
    swap_rows(pivot, max_row);
    // 消元
    for row in (pivot+1)..size {
        let factor = m[row][pivot] / m[pivot][pivot];
        for col in pivot..=size {
            m[row][col] -= factor * m[pivot][col];
        }
    }
}
// 回代
for row in (0..size).rev() {
    x[row] = (m[row][size] - sum) / m[row][row];
}
```
