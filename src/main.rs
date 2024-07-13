pub mod cpu;
pub mod opcodes;


fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.register_a = 10;
    cpu.load_and_run(vec![0xaa, 0x00]);
    println!("NES 6502 CPU, {}", cpu.stack_pointer);
}