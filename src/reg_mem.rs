use crate::{Mem64, Reg64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegMem64 {
    Reg(Reg64),
    Mem(Mem64),
}

impl RegMem64 {
    pub fn rex_b(&self) -> bool {
        match self {
            RegMem64::Reg(reg) => reg.rex_b(),
            RegMem64::Mem(mem) => mem.rex_b(),
        }
    }

    pub fn mode(&self) -> u8 {
        match self {
            RegMem64::Reg(reg) => reg.mode(),
            RegMem64::Mem(mem) => mem.mode(),
        }
    }

    pub fn rm(&self) -> u8 {
        match self {
            RegMem64::Reg(reg) => reg.rm(),
            RegMem64::Mem(mem) => mem.rm(),
        }
    }
}
