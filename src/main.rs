mod emu;
fn main() {
    let mut my_emulator = emu::Emulator::new();
    my_emulator.run(String::from("/home/jarrett/Documents/why/mm.gb"));
}
