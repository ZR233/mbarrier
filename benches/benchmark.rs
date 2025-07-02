/*!
Benchmarks for memory barrier operations.

These benchmarks measure the performance overhead of different memory barriers.
Note that the actual performance impact depends heavily on the CPU architecture
and the surrounding code patterns.
*/

#![no_std]
#![no_main]

use mbarrier::*;

/// Benchmark memory barrier operations
pub struct BarrierBench;

impl BarrierBench {
    /// Benchmark rmb() performance
    pub fn bench_rmb(iterations: usize) -> u64 {
        let start = Self::get_cycles();
        
        for _ in 0..iterations {
            rmb();
        }
        
        let end = Self::get_cycles();
        end - start
    }
    
    /// Benchmark wmb() performance
    pub fn bench_wmb(iterations: usize) -> u64 {
        let start = Self::get_cycles();
        
        for _ in 0..iterations {
            wmb();
        }
        
        let end = Self::get_cycles();
        end - start
    }
    
    /// Benchmark mb() performance
    pub fn bench_mb(iterations: usize) -> u64 {
        let start = Self::get_cycles();
        
        for _ in 0..iterations {
            mb();
        }
        
        let end = Self::get_cycles();
        end - start
    }
    
    /// Benchmark smp_rmb() performance
    pub fn bench_smp_rmb(iterations: usize) -> u64 {
        let start = Self::get_cycles();
        
        for _ in 0..iterations {
            smp_rmb();
        }
        
        let end = Self::get_cycles();
        end - start
    }
    
    /// Benchmark smp_wmb() performance
    pub fn bench_smp_wmb(iterations: usize) -> u64 {
        let start = Self::get_cycles();
        
        for _ in 0..iterations {
            smp_wmb();
        }
        
        let end = Self::get_cycles();
        end - start
    }
    
    /// Benchmark smp_mb() performance
    pub fn bench_smp_mb(iterations: usize) -> u64 {
        let start = Self::get_cycles();
        
        for _ in 0..iterations {
            smp_mb();
        }
        
        let end = Self::get_cycles();
        end - start
    }
    
    /// Get CPU cycle counter (architecture-specific)
    #[inline(always)]
    fn get_cycles() -> u64 {
        #[cfg(target_arch = "x86_64")]
        {
            unsafe {
                let mut high: u32;
                let mut low: u32;
                core::arch::asm!(
                    "rdtsc",
                    out("eax") low,
                    out("edx") high,
                    options(nostack, preserves_flags)
                );
                ((high as u64) << 32) | (low as u64)
            }
        }
        
        #[cfg(target_arch = "aarch64")]
        {
            unsafe {
                let mut cycles: u64;
                core::arch::asm!(
                    "mrs {}, cntvct_el0",
                    out(reg) cycles,
                    options(nostack, preserves_flags)
                );
                cycles
            }
        }
        
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            // Fallback: use a simple counter
            static mut COUNTER: u64 = 0;
            unsafe {
                COUNTER += 1;
                COUNTER
            }
        }
    }
    
    /// Run all benchmarks and print results
    pub fn run_all_benchmarks(iterations: usize) {
        let rmb_cycles = Self::bench_rmb(iterations);
        let wmb_cycles = Self::bench_wmb(iterations);
        let mb_cycles = Self::bench_mb(iterations);
        let smp_rmb_cycles = Self::bench_smp_rmb(iterations);
        let smp_wmb_cycles = Self::bench_smp_wmb(iterations);
        let smp_mb_cycles = Self::bench_smp_mb(iterations);
        
        // Note: In a real benchmark, you would print or return these values
        // Here we just force the compiler to not optimize them away
        unsafe {
            core::ptr::write_volatile(&mut core::ptr::null_mut::<u64>() as *mut _, rmb_cycles);
            core::ptr::write_volatile(&mut core::ptr::null_mut::<u64>() as *mut _, wmb_cycles);
            core::ptr::write_volatile(&mut core::ptr::null_mut::<u64>() as *mut _, mb_cycles);
            core::ptr::write_volatile(&mut core::ptr::null_mut::<u64>() as *mut _, smp_rmb_cycles);
            core::ptr::write_volatile(&mut core::ptr::null_mut::<u64>() as *mut _, smp_wmb_cycles);
            core::ptr::write_volatile(&mut core::ptr::null_mut::<u64>() as *mut _, smp_mb_cycles);
        }
    }
}

/// Benchmark producer-consumer pattern
pub struct ProducerConsumerBench;

impl ProducerConsumerBench {
    /// Benchmark with memory barriers
    pub fn bench_with_barriers(iterations: usize) -> u64 {
        static mut DATA: u32 = 0;
        static mut FLAG: bool = false;
        
        let start = BarrierBench::get_cycles();
        
        for i in 0..iterations {
            unsafe {
                // Producer
                core::ptr::write_volatile(&mut DATA, i as u32);
                wmb();
                core::ptr::write_volatile(&mut FLAG, true);
                
                // Consumer
                if core::ptr::read_volatile(&FLAG) {
                    rmb();
                    let _value = core::ptr::read_volatile(&DATA);
                    core::ptr::write_volatile(&mut FLAG, false);
                }
            }
        }
        
        let end = BarrierBench::get_cycles();
        end - start
    }
    
    /// Benchmark without memory barriers (unsafe but faster)
    pub fn bench_without_barriers(iterations: usize) -> u64 {
        static mut DATA: u32 = 0;
        static mut FLAG: bool = false;
        
        let start = BarrierBench::get_cycles();
        
        for i in 0..iterations {
            unsafe {
                // Producer (no barriers)
                core::ptr::write_volatile(&mut DATA, i as u32);
                core::ptr::write_volatile(&mut FLAG, true);
                
                // Consumer (no barriers)
                if core::ptr::read_volatile(&FLAG) {
                    let _value = core::ptr::read_volatile(&DATA);
                    core::ptr::write_volatile(&mut FLAG, false);
                }
            }
        }
        
        let end = BarrierBench::get_cycles();
        end - start
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_barrier_benchmarks() {
        const ITERATIONS: usize = 1000;
        
        // Run benchmarks (results would normally be compared)
        BarrierBench::run_all_benchmarks(ITERATIONS);
        
        let with_barriers = ProducerConsumerBench::bench_with_barriers(ITERATIONS);
        let without_barriers = ProducerConsumerBench::bench_without_barriers(ITERATIONS);
        
        // Ensure benchmarks actually ran
        assert!(with_barriers > 0);
        assert!(without_barriers > 0);
    }
}
