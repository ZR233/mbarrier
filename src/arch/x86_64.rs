/*!
x86_64 architecture memory barrier implementations.

Based on Linux kernel arch/x86/include/asm/barrier.h
*/

use core::arch::asm;
use core::sync::atomic::{fence, Ordering};

/// x86_64 read memory barrier implementation.
/// 
/// On x86/x86_64, reads are not reordered with other reads, 
/// so rmb() is just a compiler barrier.
#[inline(always)]
pub fn rmb_impl() {
    fence(Ordering::Acquire);
}

/// x86_64 write memory barrier implementation.
/// 
/// On x86/x86_64, writes are not reordered with other writes,
/// but we need sfence for certain cases (streaming stores, etc).
#[inline(always)]
pub fn wmb_impl() {
    unsafe {
        asm!("sfence", options(nostack, preserves_flags));
    }
}

/// x86_64 general memory barrier implementation.
/// 
/// mfence provides a full memory barrier on x86/x86_64.
#[inline(always)]
pub fn mb_impl() {
    unsafe {
        asm!("mfence", options(nostack, preserves_flags));
    }
}

/// x86_64 data dependency barrier implementation.
/// 
/// x86/x86_64 has strong ordering guarantees, so this is a no-op.
#[inline(always)]  
pub fn read_barrier_depends_impl() {
    // No-op on x86/x86_64 - data dependencies provide ordering
}
