use std::path::Path;

use color_eyre::eyre::{bail, Result};

pub enum Instruction {
    // STARTS with 0
    ///00E0 - CLS
    CLS,
    ///00EE - RET
    RET,
    ///0nnn - SYS addr
    /// nnn - 12bit value
    SYS(u16),

    // STARTS with 1
    ///1nnn - JP addr
    /// nnn - 12bit value
    JP(u16),

    // STARTS with 2
    ///2nnn - CALL addr
    CALL(u16),

    // STARTS with 3
    ///3xkk - SE Vx, byte
    SExByte(u8, u8),

    // STARTS with 4
    ///4xkk - SNE Vx, byte
    SNExByte(u8, u8),

    // STARTS with 5
    ///5xy0 - SE Vx, Vy
    SExy(u8, u8),

    // STARTS with 6
    ///6xkk - LD Vx, byte
    LDxByte(u8, u8),

    // STARTS with 7
    ///7xkk - ADD Vx, byte
    ADDxByte(u8, u8),

    // STARTS with 8
    ///8xy0 - LD Vx, Vy
    LDxy(u8, u8),
    ///8xy1 - OR Vx, Vy
    ORxy(u8, u8),
    ///8xy2 - AND Vx, Vy
    ANDxy(u8, u8),
    ///8xy3 - XOR Vx, Vy
    XORxy(u8, u8),
    ///8xy4 - ADD Vx, Vy
    ADDxy(u8, u8),
    ///8xy5 - SUB Vx, Vy
    SUBxy(u8, u8),
    ///8xy6 - SHR Vx {, Vy}
    SHRxy(u8, u8),
    ///8xy7 - SUBN Vx, Vy
    SUBNxy(u8, u8),
    ///8xyE - SHL Vx {, Vy}
    SHLxy(u8, u8),

    // STARTS with 9
    ///9xy0 - SNE Vx, Vy
    SNExy(u8, u8),

    // STARTS with A
    ///Annn - LD I, addr
    LDIAddr(u16),

    // STARTS with B
    ///Bnnn - JP V0, addr
    JPAddr(u16),

    // STARTS with C
    ///Cxkk - RND Vx, byte
    RNDxByte(u8, u8),

    // STARTS with D
    ///Dxyn - DRW Vx, Vy, nibble
    DRWxyn(u8, u8, u8),

    // STARTS with E
    ///Ex9E - SKP Vx
    SKPx(u8),
    ///ExA1 - SKNP Vx
    SKPNPx(u8),

    // STARTS with F
    ///Fx07 - LD Vx, DT
    LDxDt(u8),
    ///Fx0A - LD Vx, K
    LDxK(u8),
    ///Fx15 - LD DT, Vx
    LDDTx(u8),
    ///Fx18 - LD ST, Vx
    LDSTx(u8),
    ///Fx1E - ADD I, Vx
    ADDIx(u8),
    ///Fx29 - LD F, Vx
    LDFx(u8),
    ///Fx33 - LD B, Vx
    LDBx(u8),
    ///Fx55 - LD [I], Vx
    LDIx(u8),
    ///Fx65 - LD Vx, [I]
    LDxI(u8),
}
impl Instruction {
    /// decode u16 to enum
    fn decode(v: u16) -> Result<Self> {
        Ok(match v >> 14 {
            0x0 => match v {
                0x00E0 => Instruction::CLS,
                0x00EE => Instruction::RET,
                addr => Instruction::SYS(addr),
            },
            0x1 => Instruction::JP(v & 0x0FFF),
            0x2 => Instruction::CALL(v & 0x0FFF),
            0x3 => Instruction::SExByte((v & 0x0F00 >> 8) as u8, (v & 0x00FF) as u8),
            0x4 => Instruction::SNExByte((v & 0x0F00 >> 8) as u8, (v & 0x00FF) as u8),
            0x5 => Instruction::SExy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
            0x6 => Instruction::LDxByte((v & 0x0F00 >> 8) as u8, (v & 0x00FF) as u8),
            0x7 => Instruction::ADDxByte((v & 0x0F00 >> 8) as u8, (v & 0x00FF) as u8),
            0x8 => match v & 0xF {
                0x0 => Instruction::LDxy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
                0x1 => Instruction::ORxy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
                0x2 => Instruction::ANDxy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
                0x3 => Instruction::XORxy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
                0x4 => Instruction::ADDxy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
                0x5 => Instruction::SUBxy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
                0x6 => Instruction::SHRxy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
                0x7 => Instruction::SUBNxy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
                0xE => Instruction::SHLxy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
                _ => bail!("invalid instruction {v}"),
            },
            0x9 => Instruction::SNExy((v & 0x0F00 >> 8) as u8, (v & 0x00F0 >> 2) as u8),
            0xA => Instruction::LDIAddr(v & 0x0FFF),
            0xB => Instruction::JPAddr(v & 0x0FFF),
            0xC => Instruction::RNDxByte((v & 0x0F00 >> 8) as u8, (v & 0x00FF) as u8),
            0xD => Instruction::DRWxyn(
                (v & 0x0F00 >> 8) as u8,
                (v & 0x00F0 >> 2) as u8,
                (v & 0x000F >> 0) as u8,
            ),
            0xE => match v & 0xFF {
                0x9E => Instruction::SKPx((v & 0x0100) as u8),
                0xA1 => Instruction::SKPNPx((v & 0x0100) as u8),
                _ => bail!("invalid instruction {v}"),
            },
            0xF => match v & 0xFF {
                0x07 => Instruction::LDxDt((v & 0x0100) as u8),
                0x0A => Instruction::LDxK((v & 0x0100) as u8),
                0x15 => Instruction::LDDTx((v & 0x0100) as u8),
                0x18 => Instruction::LDSTx((v & 0x0100) as u8),
                0x1E => Instruction::ADDIx((v & 0x0100) as u8),
                0x29 => Instruction::LDFx((v & 0x0100) as u8),
                0x33 => Instruction::LDBx((v & 0x0100) as u8),
                0x55 => Instruction::LDIx((v & 0x0100) as u8),
                0x65 => Instruction::LDxI((v & 0x0100) as u8),
                _ => bail!("invalid instruction {v}"),
            },
            _ => bail!("invalid instruction {v}"),
        })
    }

    /// encode back into u16 from enum
    fn encode(&self) -> u16 {
        match self {
            Instruction::CLS => 0x00E0,
            Instruction::RET => 0x00EE,
            Instruction::SYS(u) => 0x0111 & u,

            Instruction::JP(u) => (0x0111 & u) | 0x1000,

            Instruction::CALL(u) => (0x0111 & u) | 0x2000,

            Instruction::SExByte(x, b) => (0x0100 & *x as u16) | (0x0011 & *b as u16) | 0x3000,

            Instruction::SNExByte(x, b) => (0x0100 & *x as u16) | (0x0011 & *b as u16) | 0x4000,

            Instruction::SExy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x5000,

            Instruction::LDxByte(x, b) => (0x0100 & *x as u16) | (0x0011 & *b as u16) | 0x6000,

            Instruction::ADDxByte(x, b) => (0x0100 & *x as u16) | (0x0011 & *b as u16) | 0x7000,

            Instruction::LDxy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x8000,
            Instruction::ORxy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x8001,
            Instruction::ANDxy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x8002,
            Instruction::XORxy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x8003,
            Instruction::ADDxy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x8004,
            Instruction::SUBxy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x8005,
            Instruction::SHRxy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x8006,
            Instruction::SUBNxy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x8007,
            Instruction::SHLxy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x800E,

            Instruction::SNExy(x, y) => (0x0100 & *x as u16) | (0x0010 & *y as u16) | 0x9000,

            Instruction::LDIAddr(u) => (0x0111 & u) | 0xA000,

            Instruction::JPAddr(u) => (0x0111 & u) | 0xB000,

            Instruction::RNDxByte(x, b) => (0x0100 & *x as u16) | (0x0011 & *b as u16) | 0xC000,

            Instruction::DRWxyn(x, y, n) => {
                (0x0100 & *x as u16) | (0x0010 & *y as u16) | (0x0001 & *n as u16) | 0xD000
            }

            Instruction::SKPx(x) => (0x0100 & *x as u16) | 0xE09E,
            Instruction::SKPNPx(x) => (0x0100 & *x as u16) | 0xE0A1,

            Instruction::LDxDt(x) => (0x0100 & *x as u16) | 0xF007,
            Instruction::LDxK(x) => (0x0100 & *x as u16) | 0xF00A,
            Instruction::LDDTx(x) => (0x0100 & *x as u16) | 0xF015,
            Instruction::LDSTx(x) => (0x0100 & *x as u16) | 0xF018,
            Instruction::ADDIx(x) => (0x0100 & *x as u16) | 0xF01E,
            Instruction::LDFx(x) => (0x0100 & *x as u16) | 0xF029,
            Instruction::LDBx(x) => (0x0100 & *x as u16) | 0xF033,
            Instruction::LDIx(x) => (0x0100 & *x as u16) | 0xF055,
            Instruction::LDxI(x) => (0x0100 & *x as u16) | 0xF065,
        }
    }
}

pub enum Rom {
    Base([u8; 3584]),
    ETI600([u8; 2560]),
}

impl Rom {
    pub fn load_from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        std::fs::read(path).and_then(|f| match f.len() {
            3584 => {
                let mut arr = [0; 3584];
                arr.copy_from_slice(&f[..3584]);
                Ok(Rom::Base(arr))
            }
            2560 => {
                let mut arr = [0; 2560];
                arr.copy_from_slice(&f[..2560]);
                Ok(Rom::ETI600(arr))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not-Valid ROM file.",
            )),
        })
    }

    fn insert(&mut self, index: usize, inst: Instruction) {
        match self {
            Rom::Base(r) => {
                let i = inst.encode();
                let (a, b) = (i >> 8, i & 0x00FF);
                let (a, b) = (a as u8, b as u8);
                r[index] = a;
                r[index + 1] = b;
            }
            Rom::ETI600(r) => {
                let i = inst.encode();
                let (a, b) = (i >> 8, i & 0x00FF);
                let (a, b) = (a as u8, b as u8);
                r[index] = a;
                r[index + 1] = b;
            }
        }
    }
}
impl Default for Rom {
    fn default() -> Self {
        let mut rom = Rom::Base([0; 3584]);
        // project
        rom
    }
}

impl From<[u8; 3584]> for Rom {
    fn from(value: [u8; 3584]) -> Self {
        Rom::Base(value)
    }
}
impl From<[u8; 2560]> for Rom {
    fn from(value: [u8; 2560]) -> Self {
        Rom::ETI600(value)
    }
}
/// represents colors packed in 2bit fields in a byte
/// red, green, blue, alpha (in-order)
/// left to right encoding
///
/// red   = (u8 >> 6) & 0b11
/// green = (u8 >> 4) & 0b11
/// blue  = (u8 >> 2) & 0b11
/// alpha = (u8 >> 0) & 0b11
#[derive(Debug, Clone, Copy)]
struct Color2Bit(u8);
impl Color2Bit {
    fn r(&self) -> u8 {
        (self.0 >> 6) & 0b11
    }
    fn g(&self) -> u8 {
        (self.0 >> 4) & 0b11
    }
    fn b(&self) -> u8 {
        (self.0 >> 2) & 0b11
    }
    fn a(&self) -> u8 {
        (self.0 >> 0) & 0b11
    }
    fn set_r(&mut self, val: u8) {
        let v = val & 0b11;
        self.0 = v << 6;
    }
    fn set_g(&mut self, val: u8) {
        let v = val & 0b11;
        self.0 = v << 4;
    }
    fn set_b(&mut self, val: u8) {
        let v = val & 0b11;
        self.0 = v << 2;
    }
    fn set_a(&mut self, val: u8) {
        let v = val & 0b11;
        self.0 = v << 0;
    }
}

#[derive(Debug)]
pub struct Framebuffer<const W: usize> {
    data: [[Color2Bit; W]; 64],
}

pub type FrameBuffer32 = Framebuffer<32>;
impl Default for FrameBuffer32 {
    fn default() -> Self {
        Self {
            data: [[Color2Bit(0); 32]; 64],
        }
    }
}
pub type FrameBuffer48 = Framebuffer<48>;
impl Default for FrameBuffer48 {
    fn default() -> Self {
        Self {
            data: [[Color2Bit(0); 48]; 64],
        }
    }
}
pub type FrameBuffer64 = Framebuffer<64>;
impl Default for FrameBuffer64 {
    fn default() -> Self {
        Self {
            data: [[Color2Bit(0); 64]; 64],
        }
    }
}

enum FrameBuffer {
    B32(FrameBuffer32),
    B48(FrameBuffer48),
    B64(FrameBuffer64),
}
impl Default for FrameBuffer {
    fn default() -> Self {
        FrameBuffer::B32(FrameBuffer32::default())
    }
}

#[derive(Debug)]
pub struct Memory([u8; 4096]);

impl Memory {
    fn load(&mut self, r: Rom) {
        match r {
            Rom::Base(m) => {
                let s = &mut self.0[0x200..];
                s.copy_from_slice(&m);
            }
            Rom::ETI600(m) => {
                let s = &mut self.0[0x600..];
                s.copy_from_slice(&m);
            }
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self([0; 4096])
    }
}

#[derive(Default)]
pub struct Processor {
    registers: [u8; 16],
    mem_addr: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_addr: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    memory: Memory,
    framebuffer: FrameBuffer,
}

impl Processor {
    pub fn with_rom(rom: Rom) -> Processor {
        let mut proc = Processor::default();
        proc.memory.load(rom);
        proc
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
