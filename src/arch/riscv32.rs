/*!
RISC-V 32-bit architecture memory barrier implementations.

Based on Linux kernel arch/riscv/include/asm/barrier.h
*/

use core::arch::asm;
use core::sync::atomic::{fence, Ordering};

/// RISC-V 32 read memory barrier implementation.
/// 
/// Uses FENCE instruction with r,r (read-read) ordering.
#[inline(always)]
pub fn rmb_impl() {
    unsafe {
        asm!("fence r,r", options(nostack, preserves_flags));
    }
}

/// RISC-V 32 write memory barrier implementation.
/// 
/// Uses FENCE instruction with w,w (write-write) ordering.
#[inline(always)]
pub fn wmb_impl() {
    unsafe {
        asm!("fence w,w", options(nostack, preserves_flags));
    }
}

/// RISC-V 32 general memory barrier implementation.
/// 
/// Uses FENCE instruction with rw,rw (read-write, read-write) ordering.
#[inline(always)]
pub fn mb_impl() {
    unsafe {
        asm!("fence rw,rw", options(nostack, preserves_flags));
    }
}

/// RISC-V 32 data dependency barrier implementation.
/// 
/// RISC-V has a relaxed memory model, so we need an actual barrier.
#[inline(always)]
pub fn read_barrier_depends_impl() {
    // RISC-V needs explicit ordering for data dependencies
    unsafe {
        asm!("fence r,r", options(nostack, preserves_flags));
    }
}
