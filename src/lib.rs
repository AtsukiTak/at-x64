mod bytecode;
mod flexbytes;
pub mod instruction;
mod mem;
mod reg;

pub use bytecode::{ByteCode, ModRM, Rex, Sib};
pub use flexbytes::FlexBytes;
pub use mem::Mem64;
pub use reg::{Reg, Reg16, Reg32, Reg64, Reg8};
