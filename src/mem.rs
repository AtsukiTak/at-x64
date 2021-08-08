use crate::reg::Reg64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mem64 {
    /// [reg + u32]
    RegOffset(Reg64, u32),
    /// [RIP + u32]
    RipOffset(u32),
    /// [base + disp + index * scale]
    Sib {
        base: Option<Reg64>,
        disp: u32,
        index: Reg64,
        scale: u8, // 1 ~ 4,
    },
}

impl Mem64 {
    /// ModR/M operand の mode フィールドの値
    pub fn mode(&self) -> u8 {
        match self {
            Mem64::RegOffset(_, 0) => 0b00,
            Mem64::RegOffset(_, 1..=256) => 0b01,
            Mem64::RegOffset(_, _) => 0b10,
            Mem64::RipOffset(_) => 0b00,
            Mem64::Sib { base: None, .. } => 0b00,
            Mem64::Sib {
                disp: 0..=256,
                base: Some(Reg64::RBP | Reg64::R13),
                ..
            } => 0b01,
            Mem64::Sib {
                base: Some(Reg64::RBP | Reg64::R13),
                ..
            } => 0b11,
            Mem64::Sib { disp: 0, .. } => 0b00,
            Mem64::Sib { disp: 1..=256, .. } => 0b01,
            Mem64::Sib { .. } => 0b10,
        }
    }

    pub fn rm(&self) -> u8 {
        match self {
            Mem64::RegOffset(reg, _) => reg.rm(),
            Mem64::RipOffset(_) => 0b101,
            Mem64::Sib { .. } => 0b100,
        }
    }

    pub fn rex_x(&self) -> bool {
        use Reg64::*;

        match self {
            Mem64::Sib {
                index: R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15,
                ..
            } => true,
            _ => false,
        }
    }

    pub fn rex_b(&self) -> bool {
        match self {
            Mem64::RegOffset(reg, _) => reg.rex_b(),
            Mem64::RipOffset(_) => false,
            Mem64::Sib {
                base: Some(base), ..
            } => base.rex_b(),
            Mem64::Sib { base: None, .. } => false,
        }
    }
}
