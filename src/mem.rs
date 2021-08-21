use crate::{bytecode::Sib, reg::Reg64, BytesAtMost};

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
        scale: u8, // 0 ~ 3,
    },
}

impl Mem64 {
    pub fn reg(reg: Reg64) -> Self {
        Mem64::RegOffset(reg, 0)
    }

    pub fn reg_offset(reg: Reg64, offset: u32) -> Self {
        Mem64::RegOffset(reg, offset)
    }

    pub fn rip_offset(offset: u32) -> Self {
        Mem64::RipOffset(offset)
    }

    pub fn sib(base: Option<Reg64>, disp: u32, index: Reg64, scale: u8) -> Self {
        assert!(scale <= 3);
        Mem64::Sib {
            base,
            disp,
            index,
            scale,
        }
    }

    /// ModR/M operand の mode フィールドの値
    pub fn mode_bits(&self) -> u8 {
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

    pub fn rm_bits(&self) -> u8 {
        match self {
            Mem64::RegOffset(reg, _) => reg.rm_bits(),
            Mem64::RipOffset(_) => 0b101,
            Mem64::Sib { .. } => 0b100,
        }
    }

    pub fn sib_byte(&self) -> Option<Sib> {
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
            } => Some(Sib::new(*scale, index.reg_bits(), 0b101)),
            Mem64::Sib {
                base: Some(base),
                index,
                scale,
                ..
            } => Some(Sib::new(*scale, index.reg_bits(), base.reg_bits())),
        }
    }

    pub fn disp_bytes(&self) -> BytesAtMost<4> {
        use {Mem64::*, Reg64::*};

        match self {
            RegOffset(RBP | R13, 0) => BytesAtMost::from(0 as u8),
            RegOffset(_, 0) => BytesAtMost::new(0),
            RegOffset(_, disp @ 1..=256) => BytesAtMost::from(*disp as u8),
            RegOffset(_, disp) => BytesAtMost::from(*disp),
            RipOffset(disp) => BytesAtMost::from(*disp),
            Sib {
                base: None, disp, ..
            } => BytesAtMost::from(*disp),
            Sib {
                base: Some(RBP | R13),
                disp: 0,
                ..
            } => BytesAtMost::from(0u8),
            Sib { disp: 0, .. } => BytesAtMost::new(0),
            Sib {
                disp: disp @ 1..=256,
                ..
            } => BytesAtMost::from(*disp as u8),
            Sib { disp, .. } => BytesAtMost::from(*disp),
        }
    }

    pub fn rex_x_bit(&self) -> bool {
        use Reg64::*;

        match self {
            Mem64::Sib {
                index: R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15,
                ..
            } => true,
            _ => false,
        }
    }

    pub fn rex_b_bit(&self) -> bool {
        match self {
            Mem64::RegOffset(reg, _) => reg.rex_b_bit(),
            Mem64::RipOffset(_) => false,
            Mem64::Sib {
                base: Some(base), ..
            } => base.rex_b_bit(),
            Mem64::Sib { base: None, .. } => false,
        }
    }
}
