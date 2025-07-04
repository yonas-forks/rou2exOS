use core::arch::asm;
use crate::vga::{
    write::{string, newline},
    buffer::Color,
};
use super::result;

pub fn check_mode() -> crate::init::result::InitResult {
    let mode = check_cpu_mode();

    if mode.as_bytes().len() > 5 && mode.as_bytes()[0..4] == *b"Long" {
        return result::InitResult::Passed;
    }

    result::InitResult::Failed
}

/// Function to check CPU mode using CPUID instruction
fn check_cpu_mode() -> &'static str {
    let cpuid_supported = cpuid(0x1);

    if cpuid_supported == 0 {
        return "Real Mode (CPUID not supported)";
    }

    let cpuid_value = cpuid(0x80000000);

    // Check for 64-bit long mode (if CPUID supports extended functions)
    if cpuid_value >= 0x80000001 {
        return "Long Mode (64-bit mode)";
    }

    // Otherwise, it is protected mode
    "Protected Mode (32-bit)"
}

/// Inline assembly function to execute CPUID
fn cpuid(eax: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "cpuid",
            // Store eax into result
            inout("eax") eax => result,    
            out("ecx") _,                  
            out("edx") _,                  
        );
    }
    result
}
