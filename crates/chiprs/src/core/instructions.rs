use color_eyre::{eyre::bail, Result};

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
    JPAddr(u16),

    // STARTS with 2
    ///2nnn - CALL addr
    CALLAddr(u16),

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
    JPV0Addr(u16),

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

fn nibbles(v: u16) -> (u8, u8, u8, u8) {
    (
        ((v & 0xF000) >> 12) as u8,
        ((v & 0x0F00) >> 8) as u8,
        ((v & 0x00F0) >> 4) as u8,
        ((v & 0x000F) >> 0) as u8,
    )
}

impl Instruction {
    /// decode u16 to enum
    pub fn decode(v: u16) -> Result<Self> {
        let (i, x, y, n) = nibbles(v);
        let addr = v & 0x0FFF;
        let kk = (v & 0x00FF) as u8;
        Ok(match i {
            0x0 => match kk {
                0xE0 => Instruction::CLS,
                0xEE => Instruction::RET,
                _ => Instruction::SYS(addr),
            },
            0x1 => Instruction::JPAddr(addr),
            0x2 => Instruction::CALLAddr(addr),
            0x3 => Instruction::SExByte(x, kk),
            0x4 => Instruction::SNExByte(x, kk),
            0x5 => Instruction::SExy(x, y),
            0x6 => Instruction::LDxByte(x, kk),
            0x7 => Instruction::ADDxByte(x, kk),
            0x8 => match n {
                0x0 => Instruction::LDxy(x, y),
                0x1 => Instruction::ORxy(x, y),
                0x2 => Instruction::ANDxy(x, y),
                0x3 => Instruction::XORxy(x, y),
                0x4 => Instruction::ADDxy(x, y),
                0x5 => Instruction::SUBxy(x, y),
                0x6 => Instruction::SHRxy(x, y),
                0x7 => Instruction::SUBNxy(x, y),
                0xE => Instruction::SHLxy(x, y),
                _ => bail!("invalid instruction {v}"),
            },
            0x9 => Instruction::SNExy(x, y),
            0xA => Instruction::LDIAddr(addr),
            0xB => Instruction::JPV0Addr(addr),
            0xC => Instruction::RNDxByte(x, kk),
            0xD => Instruction::DRWxyn(x, y, n),
            0xE => match kk {
                0x9E => Instruction::SKPx(x),
                0xA1 => Instruction::SKPNPx(x),
                _ => bail!("invalid instruction {v}"),
            },
            0xF => match kk {
                0x07 => Instruction::LDxDt(x),
                0x0A => Instruction::LDxK(x),
                0x15 => Instruction::LDDTx(x),
                0x18 => Instruction::LDSTx(x),
                0x1E => Instruction::ADDIx(x),
                0x29 => Instruction::LDFx(x),
                0x33 => Instruction::LDBx(x),
                0x55 => Instruction::LDIx(x),
                0x65 => Instruction::LDxI(x),
                _ => bail!("invalid instruction {v}"),
            },
            _ => bail!("invalid instruction {v}"),
        })
    }

    /// encode back into u16 from enum
    pub fn encode(&self) -> u16 {
        fn addr(u: &u16) -> u16 {
            u & 0x0FFF
        }
        fn x_byte(x: &u8, b: &u8) -> u16 {
            (0x0F00 & ((*x as u16) << 8)) | *b as u16
        }
        fn xy(x: &u8, y: &u8) -> u16 {
            (0x0F00 & ((*x as u16) << 8)) | (0xF0 & y) as u16
        }
        fn xu16(x: &u8) -> u16 {
            0x0F00 & (*x as u16) << 8
        }
        match self {
            Instruction::CLS => 0x00E0,
            Instruction::RET => 0x00EE,
            Instruction::SYS(u) => 0x0111 & u,

            Instruction::JPAddr(u) => addr(u) | 0x1000,

            Instruction::CALLAddr(u) => addr(u) | 0x2000,

            Instruction::SExByte(x, b) => x_byte(x, b) | 0x3000,

            Instruction::SNExByte(x, b) => x_byte(x, b) | 0x4000,

            Instruction::SExy(x, y) => xy(x, y) | 0x5000,

            Instruction::LDxByte(x, b) => x_byte(x, b) | 0x6000,

            Instruction::ADDxByte(x, b) => x_byte(x, b) | 0x7000,

            Instruction::LDxy(x, y) => xy(x, y) | 0x8000,
            Instruction::ORxy(x, y) => xy(x, y) | 0x8001,
            Instruction::ANDxy(x, y) => xy(x, y) | 0x8002,
            Instruction::XORxy(x, y) => xy(x, y) | 0x8003,
            Instruction::ADDxy(x, y) => xy(x, y) | 0x8004,
            Instruction::SUBxy(x, y) => xy(x, y) | 0x8005,
            Instruction::SHRxy(x, y) => xy(x, y) | 0x8006,
            Instruction::SUBNxy(x, y) => xy(x, y) | 0x8007,
            Instruction::SHLxy(x, y) => xy(x, y) | 0x800E,

            Instruction::SNExy(x, y) => xy(x, y) | 0x9000,

            Instruction::LDIAddr(u) => addr(u) | 0xA000,

            Instruction::JPV0Addr(u) => addr(u) | 0xB000,

            Instruction::RNDxByte(x, b) => x_byte(x, b) | 0xC000,

            Instruction::DRWxyn(x, y, n) => {
                (0x0100 & *x as u16) | (0x0010 & *y as u16) | (0x0001 & *n as u16) | 0xD000
            }

            Instruction::SKPx(x) => xu16(x) | 0xE09E,
            Instruction::SKPNPx(x) => xu16(x) | 0xE0A1,

            Instruction::LDxDt(x) => xu16(x) | 0xF007,
            Instruction::LDxK(x) => xu16(x) | 0xF00A,
            Instruction::LDDTx(x) => xu16(x) | 0xF015,
            Instruction::LDSTx(x) => xu16(x) | 0xF018,
            Instruction::ADDIx(x) => xu16(x) | 0xF01E,
            Instruction::LDFx(x) => xu16(x) | 0xF029,
            Instruction::LDBx(x) => xu16(x) | 0xF033,
            Instruction::LDIx(x) => xu16(x) | 0xF055,
            Instruction::LDxI(x) => xu16(x) | 0xF065,
        }
    }
}
