use crate::reg::Reg64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mem64 {
    /// [reg + u32]
    ///
    /// ## NOTE
    /// - [RBP],[R13]は[RBP+0],[R13+0] として扱う
    /// - [RSP+d],[R12+d]は[SIB+d]として扱い、
    ///   SIB.base = RSP/R12, SIB.index = RSP
    ///   を設定する
    RegOffset(Reg64, u32),
    /// [RIP + u32]
    RipOffset(u32),
    /// [base + disp + index * scale]
    ///
    /// ## NOTE
    /// indexフィールドにRSPを指定した場合、
    /// 「index無し」として扱われる。
    Sib {
        base: Option<Reg64>,
        disp: u32,
        index: Reg64,
        scale: u8, // 1 ~ 4,
    },
}

/// address displacement
pub enum Disp {
    U8(u8),
    U32(u32),
}

impl Mem64 {
    /// ModR/M operand の mode フィールドの値
    pub fn mode(&self) -> u8 {
        use {Mem64::*, Reg64::*};

        match self {
            RegOffset(RBP | R13, 0) => 0b01,
            RegOffset(_, 0) => 0b00,
            RegOffset(_, 1..=256) => 0b01,
            RegOffset(_, _) => 0b10,
            RipOffset(_) => 0b00,
            Sib { base: None, .. } => 0b00,
            Sib {
                base: Some(RBP | R13),
                disp: 0..=256,
                ..
            } => 0b01,
            Sib {
                base: Some(RBP | R13),
                ..
            } => 0b10,
            Sib { disp: 0, .. } => 0b00,
            Sib { disp: 1..=256, .. } => 0b01,
            Sib { .. } => 0b10,
        }
    }

    pub fn rm(&self) -> u8 {
        match self {
            Mem64::RegOffset(reg, _) => reg.rm(),
            Mem64::RipOffset(_) => 0b101,
            Mem64::Sib { .. } => 0b100,
        }
    }

    pub fn disp(&self) -> Option<Disp> {
        use {Mem64::*, Reg64::*};

        match self {
            RegOffset(RBP | R13, 0) => Some(Disp::U8(0)),
            RegOffset(_, 0) => None,
            RegOffset(_, disp @ 1..=256) => Some(Disp::U8(*disp as u8)),
            RegOffset(_, disp) => Some(Disp::U32(*disp)),
            RipOffset(disp) => Some(Disp::U32(*disp)),
            Sib {
                base: None, disp, ..
            } => Some(Disp::U32(*disp)),
            Sib {
                base: Some(RBP | R13),
                disp: 0,
                ..
            } => Some(Disp::U8(0)),
            Sib { disp: 0, .. } => None,
            Sib {
                disp: disp @ 1..=256,
                ..
            } => Some(Disp::U8(*disp as u8)),
            Sib { disp, .. } => Some(Disp::U32(*disp)),
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
