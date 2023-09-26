use std::arch::x86_64::{CpuidResult, __cpuid, _rdtsc};
use obfstr::obfstr as obf;
use crate::global::loader;

// Credits : github.com/PicoJr/inside-vm
fn cpuid_cycle_count_avg(low: usize, samples: usize, high: usize) -> u64 {
    let mut tsc1: u64;
    let mut tsc2: u64;
    let mut cycles: Vec<u64> = vec![];
    let mut cpuid = CpuidResult {
        eax: 0,
        ebx: 0,
        ecx: 0,
        edx: 0,
    };
    for _ in 0..(low + samples + high) {
        unsafe {
            tsc1 = _rdtsc();
            cpuid = __cpuid(0);
            tsc2 = _rdtsc();
        }
        cycles.push(tsc2 - tsc1);
    }
    unsafe {
        // call to __cpuid would be optimized away by the compiler in release mode
        // if it were not for this call
        std::ptr::read_volatile(&cpuid);
    }

    // remove low and high outliers, keep samples
    cycles.sort_unstable();
    let cycles_without_outliers = &cycles[low..low + samples];

    // compute average cycle count without outliers, make sure we do not divide by zero
    let avg = cycles_without_outliers.iter().sum::<u64>() / std::cmp::max(samples as u64, 1);
    avg
}

fn cpuid() -> bool {
    unsafe {
        let cpuid = __cpuid(0x40000000);
        if cpuid.ecx == 0x4D566572 && cpuid.edx == 0x65726177 {true} else {false}
    }
}

fn check_invalid_leaf() -> bool {
    let invalid_leaf = 0xFFFFFFFF; // An invalid CPUID leaf

    let result: u32;
    unsafe {
        std::arch::asm!(
            "cpuid",
            inout("eax") invalid_leaf => result,
            lateout("ecx") _,
            lateout("edx") _,
        );
    }

    // Check if the result is still equal to the invalid_leaf
    result == invalid_leaf
}

pub fn inside_vm() -> bool {
    cpuid_cycle_count_avg(5, 100, 5) > 1_000 || cpuid() || check_invalid_leaf()
}
