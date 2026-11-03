use std::arch::asm;

// Compile this function ONLY IF we build programm for x86_64 processors
#[cfg(target_arch = "x86_64")]
fn main() {
    let x: u64;
    unsafe {
        asm!(
            "mov {0}, 100",
            "add {0}, 25",
            "sub {0}, 75",
            out(reg) x
        )
    }
    println!("{x}");
}
