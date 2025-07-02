# mbarrier

跨平台的 Rust 内存屏障实现，参考 Linux 内核设计。

## 特性

- 🚀 跨平台支持：x86/x86_64, ARM/AArch64, RISC-V
- 🔧 内核级实现：基于 Linux 内核的内存屏障设计
- 📦 无标准库依赖：适用于嵌入式和内核开发
- ⚡ 高性能：内联汇编实现，零开销抽象
- 🎯 SMP 感知：支持单处理器和多处理器环境

## 内存屏障类型

| 函数 | 描述 |
|------|------|
| `rmb()` | 读内存屏障 - 确保屏障前的读操作在屏障后的读操作之前完成 |
| `wmb()` | 写内存屏障 - 确保屏障前的写操作在屏障后的写操作之前完成 |
| `mb()` | 通用内存屏障 - 确保屏障前的所有内存操作在屏障后的操作之前完成 |
| `smp_rmb()` | SMP 读屏障 - 在 SMP 系统上等同于 rmb()，在 UP 系统上仅为编译器屏障 |
| `smp_wmb()` | SMP 写屏障 - 在 SMP 系统上等同于 wmb()，在 UP 系统上仅为编译器屏障 |
| `smp_mb()` | SMP 通用屏障 - 在 SMP 系统上等同于 mb()，在 UP 系统上仅为编译器屏障 |
| `read_barrier_depends()` | 数据依赖屏障 - 确保依赖读操作的正确顺序 |

## 使用示例

```rust
use mbarrier::*;

// 基本用法
fn example_basic() {
    // 读内存屏障
    rmb();
    
    // 写内存屏障
    wmb();
    
    // 通用内存屏障
    mb();
}

// 生产者-消费者模式
fn producer_consumer_example() {
    // 生产者
    unsafe {
        // 写入数据
        core::ptr::write_volatile(data_ptr, 42);
        
        // 写屏障确保数据写入完成
        wmb();
        
        // 设置标志
        core::ptr::write_volatile(flag_ptr, true);
    }
    
    // 消费者
    unsafe {
        // 读取标志
        if core::ptr::read_volatile(flag_ptr) {
            // 读屏障确保标志读取完成
            rmb();
            
            // 读取数据
            let value = core::ptr::read_volatile(data_ptr);
        }
    }
}
```

## 架构支持

### x86/x86_64

- `rmb()`: 编译器屏障（x86 系列读操作天然有序）
- `wmb()`: `sfence` 指令
- `mb()`: `mfence` 指令

### ARM/AArch64

- `rmb()`: `dmb ld` / `dsb ld` 指令
- `wmb()`: `dmb st` / `dsb st` 指令
- `mb()`: `dmb sy` / `dsb sy` 指令

### RISC-V

- `rmb()`: `fence r,r` 指令
- `wmb()`: `fence w,w` 指令
- `mb()`: `fence rw,rw` 指令

## 特性标志

- `smp` (默认启用): 启用 SMP 感知的屏障
- `std`: 启用标准库特性（预留）

## 性能考虑

- 所有屏障函数都是内联的，提供零开销抽象
- 在不需要的架构上，某些屏障可能被优化为无操作
- SMP 版本的屏障在单处理器系统上仅为编译器屏障

## 安全性

此库使用 `unsafe` 内联汇编来实现内存屏障。虽然这些操作在设计上是安全的，但请确保：

1. 正确理解内存屏障的语义
2. 在多线程环境中正确使用
3. 了解目标架构的内存模型

## 许可证

此项目采用 MIT 或 Apache-2.0 双许可证。

## 致谢

此实现参考了 Linux 内核的内存屏障设计，特别是：

- `arch/x86/include/asm/barrier.h`
- `arch/arm/include/asm/barrier.h`
- `arch/arm64/include/asm/barrier.h`
- `arch/riscv/include/asm/barrier.h`
