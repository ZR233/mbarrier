/*!
Assembly analysis test program.

This program tests each memory barrier and allows analysis of generated assembly.
*/

use mbarrier::*;

#[unsafe(no_mangle)]
pub fn test_rmb() {
    rmb();
}

#[unsafe(no_mangle)]
pub fn test_wmb() {
    wmb();
}

#[unsafe(no_mangle)]
pub fn test_mb() {
    mb();
}

#[unsafe(no_mangle)]
pub fn test_smp_rmb() {
    smp_rmb();
}

#[unsafe(no_mangle)]
pub fn test_smp_wmb() {
    smp_wmb();
}

#[unsafe(no_mangle)]
pub fn test_smp_mb() {
    smp_mb();
}

#[unsafe(no_mangle)]
pub fn test_read_barrier_depends() {
    read_barrier_depends();
}

#[unsafe(no_mangle)]
pub fn test_smp_read_barrier_depends() {
    smp_read_barrier_depends();
}

fn main() {
    println!("Testing memory barriers...");
    
    test_rmb();
    test_wmb();
    test_mb();
    test_smp_rmb();
    test_smp_wmb();
    test_smp_mb();
    test_read_barrier_depends();
    test_smp_read_barrier_depends();
    
    println!("All barriers tested successfully!");
}
