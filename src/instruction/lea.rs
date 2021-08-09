use crate::{ByteCode, FlexBytes, Mem64, ModRM, Reg64, Rex};

pub struct Lea<Dst, Src>(Dst, Src);

impl Lea<Reg64, Mem64> {
    pub fn new(dst: Reg64, src: Mem64) -> Self {
        Lea(dst, src)
    }

    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        let mut code = ByteCode::new();

        // REX prefix
        let mut rex = Rex::new();
        rex.set_w(true);
        rex.set_r(dst.rex_r());
        rex.set_x(src.rex_x());
        rex.set_b(src.rex_b());
        code.rex = Some(rex);

        // opcode
        code.opcode = FlexBytes::from([0x8D]);

        // ModR/M
        let mut mod_rm = ModRM::new();
        mod_rm.set_mode(src.mode());
        mod_rm.set_reg(dst.reg());
        mod_rm.set_rm(src.rm());
        code.mod_rm = Some(mod_rm);

        // SIB
        code.sib = src.sib();

        // addr disp
        code.addr_disp = src.disp();

        code
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        use {Mem64::*, Reg64::*};

        let cases = [
            (Lea::new(RAX, RegOffset(RDI, 0)), vec![0x48, 0x8D, 0x07]),
            (
                Lea::new(RAX, RegOffset(RDI, 42)),
                vec![0x48, 0x8D, 0x47, 0x2A],
            ),
            (
                Lea::new(RSP, RegOffset(RSP, 0)),
                vec![0x48, 0x8D, 0x24, 0x24],
            ),
            (
                Lea::new(RAX, RegOffset(RSP, 0)),
                vec![0x48, 0x8D, 0x04, 0x24],
            ),
            (
                Lea::new(RDI, RipOffset(42)),
                vec![0x48, 0x8D, 0x3D, 0x2A, 0x00, 0x00, 0x00],
            ),
            (
                Lea::new(
                    RDI,
                    Sib {
                        base: Some(RAX),
                        index: RDI,
                        scale: 1,
                        disp: 0,
                    },
                ),
                vec![0x48, 0x8D, 0x3c, 0x78],
            ),
            (
                Lea::new(
                    RDI,
                    Sib {
                        base: None,
                        index: RDI,
                        scale: 1,
                        disp: 0,
                    },
                ),
                vec![0x48, 0x8D, 0x3c, 0x7d, 0x00, 0x00, 0x00, 0x00],
            ),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().to_bytes().bytes(), expected);
        }
    }
}
