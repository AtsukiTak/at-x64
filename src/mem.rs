use crate::{bytecode::Sib, reg::Reg64, FlexBytes};

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

    pub fn sib(&self) -> Option<Sib> {
        use Reg64::*;

        match self {
            // scale=0, index=rsp, base=rsp/r12
            Mem64::RegOffset(RSP | R12, _) => Some(Sib::new(0, 0b100, 0b100)),
            Mem64::RegOffset(_, _) => None,
            Mem64::RipOffset(_) => None,
            Mem64::Sib {
                base: None,
                index,
                scale,
                ..
            } => Some(Sib::new(*scale, index.reg(), 0b101)),
            Mem64::Sib {
                base: Some(base),
                index,
                scale,
                ..
            } => Some(Sib::new(*scale, index.reg(), base.reg())),
        }
    }

    pub fn disp(&self) -> FlexBytes<4> {
        use {Mem64::*, Reg64::*};

        match self {
            RegOffset(RBP | R13, 0) => FlexBytes::from(0 as u8),
            RegOffset(_, 0) => FlexBytes::new(0),
            RegOffset(_, disp @ 1..=256) => FlexBytes::from(*disp as u8),
            RegOffset(_, disp) => FlexBytes::from(*disp),
            RipOffset(disp) => FlexBytes::from(*disp),
            Sib {
                base: None, disp, ..
            } => FlexBytes::from(*disp),
            Sib {
                base: Some(RBP | R13),
                disp: 0,
                ..
            } => FlexBytes::from(0u8),
            Sib { disp: 0, .. } => FlexBytes::new(0),
            Sib {
                disp: disp @ 1..=256,
                ..
            } => FlexBytes::from(*disp as u8),
            Sib { disp, .. } => FlexBytes::from(*disp),
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
