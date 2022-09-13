use std::fs::read;
use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::instructions::nop::NopInstr;
use crate::instructions::r#const::{AConstNull, DConst0, DConst1, FConst0, FConst1, FConst2, IConst0, IConst1, IConst2, IConst3, IConst4, IConst5, IConstM1, LConst0, LConst1};
use crate::rtda::Frame;

mod base;
mod nop;
mod r#const;
mod ipush;
mod iload;
mod lstore;
mod pop;
mod dup;
mod swap;
mod math;
mod sh;
mod and;
mod iinc;

pub fn bytes_into_instr(opcode: u8) -> Option<Box<dyn Instr>> {
    match opcode {
        0x00 => Some(Box::new(NopInstr)),
        0x01 => Some(Box::new(AConstNull)),
        0x02 => Some(Box::new(IConstM1)),
        0x03 => Some(Box::new(IConst0)),
        0x04 => Some(Box::new(IConst1)),
        0x05 => Some(Box::new(IConst2)),
        0x06 => Some(Box::new(IConst3)),
        0x07 => Some(Box::new(IConst4)),
        0x08 => Some(Box::new(IConst5)),
        0x09 => Some(Box::new(LConst0)),
        0x0a => Some(Box::new(LConst1)),
        0x0b => Some(Box::new(FConst0)),
        0x0c => Some(Box::new(FConst1)),
        0x0d => Some(Box::new(FConst2)),
        0x0e => Some(Box::new(DConst0)),
        0x0f => Some(Box::new(DConst1)),
        _ => None,
    }
}
