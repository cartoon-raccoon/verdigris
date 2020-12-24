pub mod vm;
pub mod instruction;

pub use self::vm::VM;
pub use self::instruction::{
    Instruction,
    Opcode,
};