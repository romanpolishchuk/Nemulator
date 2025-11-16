mod cpu;
use cpu::CPU;

struct Emulator {
    cpu: CPU,
    ram: Vec<u8>,
}
fn main() {
    println!("Hello, world!");
}
