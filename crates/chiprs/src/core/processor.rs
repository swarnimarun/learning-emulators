use super::{framebuffer::FrameBuffer, instructions::Instruction, rom::Rom};

use color_eyre::{
    eyre::{bail, ContextCompat},
    Result,
};

#[derive(Debug)]
pub struct Memory([u8; 4096]);

impl Memory {
    fn get_slice<'a>(&'a self, idx: u16, n: u16) -> &'a [u8] {
        &self.0[(idx as usize)..((idx + n) as usize)]
    }
    fn load(&mut self, r: Rom) -> u16 {
        match r {
            Rom::Base(m) => {
                let s = &mut self.0[0x200..];
                s.copy_from_slice(&m);
                0x200
            }
            Rom::ETI600(m) => {
                let s = &mut self.0[0x600..];
                s.copy_from_slice(&m);
                0x600
            }
        }
    }

    fn get(&self, i: u16) -> Option<u16> {
        Some(((*self.0.get(i as usize)? as u16) << 8) + *self.0.get((i as usize) + 1)? as u16)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self([0; 4096])
    }
}

#[derive(Debug, Default)]
pub struct RendererState {
    pub instant: Option<std::time::Instant>,
}

#[derive(Default, Debug)]
pub struct Processor {
    registers: [u8; 16],
    /// memory addr
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    memory: Memory,
    framebuffer: Option<FrameBuffer>,
}

impl Processor {
    pub fn with_rom(rom: Rom) -> Processor {
        let mut proc = Processor::default();
        proc.program_counter = proc.memory.load(rom);
        proc
    }

    pub fn run(&mut self, window: &winit::window::Window, rs: &mut RendererState) -> Result<()> {
        if self.framebuffer.is_none() {
            _ = self.framebuffer.insert(FrameBuffer::new(window)?);
        }
        let Some(instant) = rs.instant.take() else {
            bail!("time not present");
        };
        let dt = instant.elapsed();
        _ = rs.instant.insert(std::time::Instant::now());
        while self.program_counter < 4096 {
            if let Some(x) = self.memory.get(self.program_counter) {
                let inst = Instruction::decode(x)?;
                match inst {
                    Instruction::SYS(_) => todo!("ignored on modern interpreters"),
                    Instruction::CLS => self
                        .framebuffer
                        .as_mut()
                        .context("window not initialized")?
                        .clear(),
                    Instruction::RET => {
                        // get from stack
                        self.stack_pointer -= 1;
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        continue;
                    }
                    Instruction::JPAddr(addr) => {
                        self.program_counter = addr;
                        continue;
                    }
                    Instruction::CALLAddr(addr) => {
                        self.stack[self.stack_pointer as usize] = self.program_counter;
                        self.stack_pointer += 1;
                        self.program_counter = addr;
                        continue;
                    }
                    Instruction::SExByte(x, b) => {
                        if self.registers[x as usize] == b {
                            self.program_counter += 2;
                        }
                    }
                    Instruction::SNExByte(x, b) => {
                        if self.registers[x as usize] == b {
                            self.program_counter += 2;
                        }
                    }
                    Instruction::SExy(x, y) => {
                        if self.registers[x as usize] == self.registers[y as usize] {
                            self.program_counter += 2;
                        }
                    }
                    Instruction::LDxByte(x, b) => {
                        self.registers[x as usize] = b;
                    }
                    Instruction::ADDxByte(x, b) => {
                        self.registers[x as usize] += b;
                    }
                    Instruction::LDxy(x, y) => {
                        self.registers[x as usize] = self.registers[y as usize];
                    }
                    Instruction::ORxy(x, y) => {
                        self.registers[x as usize] =
                            self.registers[x as usize] | self.registers[y as usize];
                    }
                    Instruction::ANDxy(x, y) => {
                        self.registers[x as usize] =
                            self.registers[x as usize] & self.registers[y as usize];
                    }
                    Instruction::XORxy(x, y) => {
                        self.registers[x as usize] =
                            self.registers[x as usize] ^ self.registers[y as usize];
                    }
                    Instruction::ADDxy(x, y) => {
                        let s =
                            self.registers[x as usize] as u16 + self.registers[y as usize] as u16;
                        if s > 0xFF {
                            self.registers[15] = 1;
                        } else {
                            self.registers[15] = 0;
                        }
                        self.registers[x as usize] = (s & 0x0011) as u8;
                    }
                    Instruction::SUBxy(x, y) => {
                        if self.registers[x as usize] > self.registers[y as usize] {
                            self.registers[15] = 1;
                            self.registers[x as usize] =
                                self.registers[x as usize] - self.registers[y as usize];
                        } else {
                            self.registers[15] = 0;
                            self.registers[x as usize] = 0;
                        }
                    }
                    Instruction::SHRxy(x, _y) => {
                        self.registers[0] = self.registers[x as usize] & 1;
                        self.registers[x as usize] >>= 1;
                    }
                    Instruction::SUBNxy(x, y) => {
                        if self.registers[x as usize] < self.registers[y as usize] {
                            self.registers[15] = 1;
                            self.registers[x as usize] =
                                self.registers[y as usize] - self.registers[x as usize];
                        } else {
                            self.registers[15] = 0;
                            self.registers[x as usize] = 0;
                        }
                    }
                    Instruction::SHLxy(x, _y) => {
                        self.registers[0] = (self.registers[x as usize] & 0b10000000) >> 7;
                        self.registers[x as usize] <<= 1;
                    }
                    Instruction::SNExy(x, y) => {
                        if self.registers[x as usize] != self.registers[y as usize] {
                            self.program_counter += 2;
                        }
                    }
                    Instruction::LDIAddr(addr) => self.i = addr,
                    Instruction::JPV0Addr(addr) => {
                        self.program_counter = addr + self.registers[0] as u16;
                        continue;
                    }
                    Instruction::RNDxByte(x, b) => {
                        let v: u8 = rand::random();
                        self.registers[x as usize] = v & b;
                    }
                    Instruction::DRWxyn(x, y, n) => {
                        let vx = self.registers[x as usize];
                        let vy = self.registers[y as usize];
                        let nslice = self.memory.get_slice(self.i, n as u16);
                        let collision = self
                            .framebuffer
                            .as_mut()
                            .context("window not initialized")?
                            .draw_at(vx, vy, nslice);
                        self.registers[15] = collision as u8;
                    }
                    Instruction::SKPx(x) => todo!(),
                    Instruction::SKPNPx(x) => todo!(),
                    Instruction::LDxDt(x) => todo!(),
                    Instruction::LDxK(x) => todo!(),
                    Instruction::LDDTx(x) => todo!(),
                    Instruction::LDSTx(x) => todo!(),
                    Instruction::ADDIx(x) => todo!(),
                    Instruction::LDFx(x) => todo!(),
                    Instruction::LDBx(x) => todo!(),
                    Instruction::LDIx(x) => todo!(),
                    Instruction::LDxI(x) => todo!(),
                };
            } else {
                bail!("bad program counter value {}", self.program_counter);
            }
            self.program_counter += 2;
        }
        Ok(())
    }

    pub fn text(i: char) -> [u8; 5] {
        match i {
            '0' => [0xF0, 0x90, 0x90, 0x90, 0xF0],
            '1' => [0x20, 0x60, 0x20, 0x20, 0x70],
            '2' => [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            '3' => [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            '4' => [0x90, 0x90, 0xF0, 0x10, 0x10],
            '5' => [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            '6' => [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            '7' => [0xF0, 0x10, 0x20, 0x40, 0x40],
            '8' => [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            '9' => [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            'a' => [0xF0, 0x90, 0xF0, 0x90, 0x90],
            'b' => [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            'c' => [0xF0, 0x80, 0x80, 0x80, 0xF0],
            'd' => [0xE0, 0x90, 0x90, 0x90, 0xE0],
            'e' => [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            'f' => [0xF0, 0x80, 0xF0, 0x80, 0x80],
            _ => panic!("invalid: {i}"),
        }
    }
}
