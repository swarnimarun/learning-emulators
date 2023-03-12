/// RAM memory space: 4 kB (4096 B)
/// 16 general purpose 8-bit registers
/// The delay timer DT register (8-bit)
/// The sound timer ST register (8-bit)
/// The index register I (16-bit), used to store memory addresses
/// The program counter PC, another pseudo-register (16-bit) that points to the address in memory of the current instruction
/// The stack pointer SP, a pseudo-register (8 or 16-bit, depending on the size of your stack) that points to the top of the stack
/// The stack, a LIFO array of 16-bit values used for subroutines
/// The keyboard, which contains 16 keys used as input
///
/// default values are for processor allow easier creation
pub struct Processor<const MEM_SIZE: usize, const STACK_SIZE: usize> {
    memory: [u8; MEM_SIZE],
    program_counter: usize, // using `usize` as M can be arbitarily large
    stack: [u16; STACK_SIZE],
    stack_pointer: usize,
    registers: [u8; 16],
    index_register: usize, // to store a memory address
    delay_timer: u8,
    sound_timer: u8,
    keyboard: u16, // each bit represents a key input
}

impl<const MEM_SIZE: usize, const STACK_SIZE: usize> Default for Processor<MEM_SIZE, STACK_SIZE> {
    fn default() -> Self {
        Self {
            memory: [0; MEM_SIZE],
            program_counter: 0,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            registers: [0; 16],
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: 0,
        }
    }
}

impl<const MEM_SIZE: usize, const STACK_SIZE: usize> Processor<MEM_SIZE, STACK_SIZE> {
    pub fn with_rom(rom: &[u8]) -> Self {
        let mut memory = [0; MEM_SIZE];
        // ensure that that they are equal
        if rom.len() < MEM_SIZE - 200 {
            memory[200..rom.len() + 200].copy_from_slice(rom);
        } else if rom.len() > MEM_SIZE - 200 {
            memory[200..].copy_from_slice(&rom[..MEM_SIZE - 200]);
        } else {
            memory[200..].copy_from_slice(rom);
        };
        Self {
            memory,
            ..Default::default()
        }
    }
}
