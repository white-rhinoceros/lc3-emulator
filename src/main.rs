mod computer;

fn main() {
    let mut cpu = computer::CPU::new();

    cpu.tick();

    println!("Hello, {}", cpu.reg_get_val(&computer::Registers::RegR0).unwrap());
}
