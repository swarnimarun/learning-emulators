use std::path::Path;

use color_eyre::{eyre::ContextCompat, Result};

use crate::core::instructions::Row;

use super::instructions::Instruction;

pub enum Rom {
    Base([u8; 3584], usize),
    ETI600([u8; 2560], usize),
}

impl Rom {
    pub fn load_from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        std::fs::read(path.as_ref()).and_then(|f| match f.len() {
            3584 => {
                let mut arr = [0; 3584];
                arr.copy_from_slice(&f[..3584]);
                Ok(Rom::Base(arr, 3584))
            }
            2560 => {
                let mut arr = [0; 2560];
                arr.copy_from_slice(&f[..2560]);
                Ok(Rom::ETI600(arr, 2560))
            }
            x if x < 3584 && x > 0 => {
                let mut arr = [0; 3584];
                arr[..x].copy_from_slice(&f[..x]);
                Ok(Rom::Base(arr, x))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not-Valid ROM file.",
            )),
        })
    }

    fn insert(&mut self, index: usize, inst: Instruction) {
        match self {
            Rom::Base(r, _) => {
                let i = inst.encode();
                let (a, b) = (i >> 8, i & 0x00FF);
                let (a, b) = (a as u8, b as u8);
                r[index] = a;
                r[index + 1] = b;
            }
            Rom::ETI600(r, _) => {
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
        let mut rom = Rom::Base([0; 3584], 3584);
        // project
        rom
    }
}

impl From<[u8; 3584]> for Rom {
    fn from(value: [u8; 3584]) -> Self {
        Rom::Base(value, 3584)
    }
}
impl From<[u8; 2560]> for Rom {
    fn from(value: [u8; 2560]) -> Self {
        Rom::ETI600(value, 2560)
    }
}

impl Rom {
    pub fn rom_disassemble(&self) -> Result<()> {
        let (s, l) = match self {
            Rom::Base(x, l) => (x.as_slice(), *l),
            Rom::ETI600(x, l) => (x.as_slice(), *l),
        };
        let mut i = 512usize;
        let mut x = 0;
        println!("  Idx |  Hex |     Binary |   Name |  Addr |  Byte |  Reg1 |  Reg2 |   N \n--------------------------------------------------------------------------",);
        for b in &s[..l] {
            if i % 2 == 0 {
                x += *b as u16;
                println!(
                    " {i:>4} | 0x{b:02x} | 0b{b:08b} | {}",
                    Instruction::decode(x)?
                );
            } else {
                x = (*b as u16) << 8;
                println!(" {i:>4} | 0x{b:02x} | 0b{b:08b} | {}", Row::empty());
            }
            i += 1;
        }
        Ok(())
    }
}
