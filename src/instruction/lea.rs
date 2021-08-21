use crate::{ByteCode, BytesAtMost, Mem64, ModRM, Reg64, Rex};

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
        rex.set_r(dst.rex_r_bit());
        rex.set_x(src.rex_x_bit());
        rex.set_b(src.rex_b_bit());
        code.rex = Some(rex);

        // opcode
        code.opcode = BytesAtMost::from([0x8D]);

        // ModR/M
        let mut mod_rm = ModRM::new();
        mod_rm.set_mode(src.mode_bits());
        mod_rm.set_reg(dst.reg_bits());
        mod_rm.set_rm(src.rm_bits());
        code.mod_rm = Some(mod_rm);

        // SIB
        code.sib = src.sib_byte();

        // addr disp
        code.addr_disp = src.disp_bytes();

        code
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        use Reg64::*;

        let cases = [
            (Lea::new(RAX, Mem64::reg(RDI)), vec![0x48, 0x8D, 0x07]),
            (
                Lea::new(RAX, Mem64::reg_offset(RDI, 42)),
                vec![0x48, 0x8D, 0x47, 0x2A],
            ),
            (Lea::new(RSP, Mem64::reg(RSP)), vec![0x48, 0x8D, 0x24, 0x24]),
            (Lea::new(RAX, Mem64::reg(RSP)), vec![0x48, 0x8D, 0x04, 0x24]),
            (
                Lea::new(RDI, Mem64::rip_offset(42)),
                vec![0x48, 0x8D, 0x3D, 0x2A, 0x00, 0x00, 0x00],
            ),
            (
                Lea::new(RDI, Mem64::sib(Some(RAX), 0, RDI, 1)),
                vec![0x48, 0x8D, 0x3c, 0x78],
            ),
            (
                Lea::new(RDI, Mem64::sib(None, 0, RDI, 1)),
                vec![0x48, 0x8D, 0x3c, 0x7d, 0x00, 0x00, 0x00, 0x00],
            ),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().to_bytes().bytes(), expected);
        }
    }
}
