use crate::{ByteCode, FlexBytes, Mem64, ModRM, Reg64, Rex};

pub struct Mov<Dst, Src>(pub Dst, pub Src);

impl Mov<Mem64, Reg64> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        let mut code = ByteCode::new();

        // REX prefix
        let mut rex = Rex::new();
        rex.set_w(true);
        rex.set_r(src.rex_r());
        rex.set_x(dst.rex_x());
        rex.set_b(dst.rex_b());
        code.rex = Some(rex);

        // opcode
        code.opcode = FlexBytes::from([0x89]);

        // ModR/M
        let mut mod_rm = ModRM::new();
        mod_rm.set_mode(dst.mode());
        mod_rm.set_reg(src.reg());
        mod_rm.set_rm(dst.rm());
        code.mod_rm = Some(mod_rm);

        // SIB
        code.sib = dst.sib();

        // addr disp
        code.addr_disp = dst.disp();

        code
    }
}

impl Mov<Reg64, Reg64> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        let mut code = ByteCode::new();

        // set REX prefix
        let mut rex = Rex::new();
        rex.set_w(true);
        rex.set_r(src.rex_r());
        rex.set_x(false);
        rex.set_b(dst.rex_b());
        code.rex = Some(rex);

        // set opcode
        code.opcode = FlexBytes::from([0x89]);

        // set ModR/M
        let mut mod_rm = ModRM::new();
        mod_rm.set_mode(dst.mode());
        mod_rm.set_reg(src.reg());
        mod_rm.set_rm(dst.rm());
        code.mod_rm = Some(mod_rm);

        code
    }
}

impl Mov<Reg64, u64> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        let mut code = ByteCode::new();

        // REX prefix
        let mut rex = Rex::new();
        rex.set_w(true);
        rex.set_b(dst.rex_b());
        code.rex = Some(rex);

        // opcode
        code.opcode = FlexBytes::from([0xB8 + dst.reg()]);

        // immutable val
        code.imm = FlexBytes::from(src);

        code
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mov_mem64_reg64() {
        use {Mem64::*, Reg64::*};

        let cases = [
            (Mov(RegOffset(RDI, 0), RAX), vec![0x48, 0x89, 0x07]),
            (Mov(RegOffset(RDI, 42), RAX), vec![0x48, 0x89, 0x47, 0x2A]),
            (
                Mov(RipOffset(42), RAX),
                vec![0x48, 0x89, 0x05, 0x2A, 0x00, 0x00, 0x00],
            ),
            (
                Mov(
                    Sib {
                        base: Some(RBP),
                        disp: 42,
                        index: RAX,
                        scale: 3,
                    },
                    R13,
                ),
                vec![0x4C, 0x89, 0x6C, 0xC5, 0x2A],
            ),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().to_bytes().bytes(), expected);
        }
    }
}
