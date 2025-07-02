/*!
Performance comparison example.

This example compares the performance of different barrier types
and demonstrates the overhead they introduce.
*/

use mbarrier::*;
use std::time::Instant;

const ITERATIONS: usize = 1_000_000;

fn benchmark_barrier<F: Fn()>(name: &str, barrier_fn: F) -> u128 {
    let start = Instant::now();
    
    for _ in 0..ITERATIONS {
        barrier_fn();
    }
    
    let duration = start.elapsed();
    let nanos = duration.as_nanos();
    
    println!("{}: {} ns total, {} ns per operation", 
             name, nanos, nanos / ITERATIONS as u128);
    
    nanos
}

fn main() {
    println!("Memory Barrier Performance Comparison");
    println!("=====================================");
    println!("Iterations: {}", ITERATIONS);
    println!("");
    
    // Benchmark no-op (baseline)
    let baseline = benchmark_barrier("No-op baseline", || {});
    
    // Benchmark read barriers
    let rmb_time = benchmark_barrier("rmb()", rmb);
    let smp_rmb_time = benchmark_barrier("smp_rmb()", smp_rmb);
    
    // Benchmark write barriers  
    let wmb_time = benchmark_barrier("wmb()", wmb);
    let smp_wmb_time = benchmark_barrier("smp_wmb()", smp_wmb);
    
    // Benchmark general barriers
    let mb_time = benchmark_barrier("mb()", mb);
    let smp_mb_time = benchmark_barrier("smp_mb()", smp_mb);
    
    // Benchmark dependency barriers
    let dep_time = benchmark_barrier("read_barrier_depends()", read_barrier_depends);
    let smp_dep_time = benchmark_barrier("smp_read_barrier_depends()", smp_read_barrier_depends);
    
    println!("");
    println!("Performance Analysis (overhead vs baseline):");
    println!("============================================");
    
    fn print_overhead(name: &str, time: u128, baseline: u128) {
        let overhead = time.saturating_sub(baseline);
        let percentage = if baseline > 0 { 
            (overhead as f64 / baseline as f64) * 100.0 
        } else { 
            0.0 
        };
        println!("{}: +{} ns ({:.1}% overhead)", name, overhead, percentage);
    }
    
    print_overhead("rmb()", rmb_time, baseline);
    print_overhead("wmb()", wmb_time, baseline);
    print_overhead("mb()", mb_time, baseline);
    print_overhead("smp_rmb()", smp_rmb_time, baseline);
    print_overhead("smp_wmb()", smp_wmb_time, baseline);
    print_overhead("smp_mb()", smp_mb_time, baseline);
    print_overhead("read_barrier_depends()", dep_time, baseline);
    print_overhead("smp_read_barrier_depends()", smp_dep_time, baseline);
    
    println!("");
    println!("Architecture: {}", std::env::consts::ARCH);
    
    #[cfg(feature = "smp")]
    println!("SMP mode: enabled");
    #[cfg(not(feature = "smp"))]
    println!("SMP mode: disabled");
}
