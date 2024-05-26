use std::path::Path;

use super::instructions::Instruction;

pub enum Rom {
    Base([u8; 3584]),
    ETI600([u8; 2560]),
}

impl Rom {
    pub fn load_from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        std::fs::read(path.as_ref()).and_then(|f| match f.len() {
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
