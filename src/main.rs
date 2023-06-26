mod emu;
fn main() {
    let mut my_emulator = emu::Emulator::new();
    my_emulator.run("/home/jarrett/Documents/why/mm.gb");
}
