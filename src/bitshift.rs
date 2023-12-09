use crate::load::Opcode;
use num_traits::FromPrimitive;

type Umi = u32;

/// Represents a bit field of a 32-bit word given a width and least significant bit
pub struct Field {
    width: u32,
    lsb: u32,
}

pub static RA: Field = Field { width: 3, lsb: 6 };
pub static RB: Field = Field { width: 3, lsb: 3 };
pub static RC: Field = Field { width: 3, lsb: 0 };
pub static RL: Field = Field { width: 3, lsb: 25 };
pub static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };

/// Creates a bitmask of 'bits' length
fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// Given an instruction word, extract the opcode
pub fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32((instruction >> OP.lsb) & mask(OP.width))
}