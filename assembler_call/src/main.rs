use core::arch::global_asm;

global_asm!("
    idk:
      mov rax, 1337
      ret
");

global_asm!(include_str!("met.S"));

extern "C" {
    fn idk() -> u64;
    fn t6() -> u64;
}

fn main() {
    unsafe {
        println!("idk : {}", idk());
        println!("t6  : {}", t6());
    }
}