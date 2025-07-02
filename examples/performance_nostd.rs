/*!
Performance comparison example.

This example compares the performance of different barrier types
and demonstrates the overhead they introduce.

Note: This is a simplified version that works in no_std environments.
For detailed timing measurements, use a std environment.
*/

#![no_std]
#![no_main]

use mbarrier::*;

const ITERATIONS: usize = 1_000_000;

// Simple cycle counter for basic performance indication
#[inline(always)]
fn get_cycles() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_rdtsc()
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let mut cycles: u64;
        core::arch::asm!("mrs {}, cntvct_el0", out(reg) cycles);
        cycles
    }
    #[cfg(target_arch = "riscv64")]
    unsafe {
        let mut cycles: u64;
        core::arch::asm!("rdcycle {}", out(reg) cycles);
        cycles
    }
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    )))]
    {
        // Fallback: just return iteration count
        0
    }
}

fn benchmark_barrier<F: Fn()>(_name: &str, barrier_fn: F) -> u64 {
    let start = get_cycles();

    for _ in 0..ITERATIONS {
        barrier_fn();
    }

    let end = get_cycles();
    end.wrapping_sub(start)
}

// Simple panic handler for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Entry point for no_std
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}

fn main() {
    // Benchmark no-op (baseline)
    let _baseline = benchmark_barrier("No-op baseline", || {});

    // Benchmark read barriers
    let _rmb_time = benchmark_barrier("rmb()", rmb);
    let _smp_rmb_time = benchmark_barrier("smp_rmb()", smp_rmb);

    // Benchmark write barriers
    let _wmb_time = benchmark_barrier("wmb()", wmb);
    let _smp_wmb_time = benchmark_barrier("smp_wmb()", smp_wmb);

    // Benchmark general barriers
    let _mb_time = benchmark_barrier("mb()", mb);
    let _smp_mb_time = benchmark_barrier("smp_mb()", smp_mb);

    // Note: In no_std environment, we can't print results
    // The benchmark still runs and measures cycles if supported
}
