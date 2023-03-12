use crate::Processor;

#[test]
fn processor_construction_with_rom_load() {
    let _ = Processor::with_rom(&[0]);
    let _ = Processor::with_rom(&[]);
    let _ = Processor::with_rom(&[0; 100000]);
}
