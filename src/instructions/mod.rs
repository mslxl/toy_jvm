use std::fs::read;
use std::str::LinesAny;
use crate::bytecode_reader::BytecodeReader;
use crate::instructions::and::{IAnd, IOr, IXOr, LAnd, LOr, LXOr};
use crate::instructions::base::Instr;
use crate::instructions::cmp::{DCMP, FCMP, LCMP};
use crate::instructions::ctrl::Goto;
use crate::instructions::dup::{Dup, Dup2, Dup2X1, Dup2X2, DupX1, DupX2};
use crate::instructions::ifcond::{IFEQ, IFGE, IFGT, IfICmpEq, IfICmpGe, IfICmpGt, IfICmpLe, IfICmpLt, IfICmpNe, IFLE, IFLT, IFNE};
use crate::instructions::iinc::IInc;
use crate::instructions::iload::{ILoad, ILoad0, ILoad1, ILoad2, ILoad3};
use crate::instructions::ipush::{BIPush, SIPush};
use crate::instructions::math::{DAdd, DDiv, DMul, DNeg, DRem, DSub, FAdd, FDiv, FMul, FRem, FSub, IAdd, IDiv, IMul, INeg, IRem, ISub, LAdd, LDiv, LMul, LRem, LSub};
use crate::instructions::nop::NopInstr;
use crate::instructions::pop::{Pop, Pop2};
use crate::instructions::r#const::{AConstNull, DConst0, DConst1, FConst0, FConst1, FConst2, IConst0, IConst1, IConst2, IConst3, IConst4, IConst5, IConstM1, LConst0, LConst1};
use crate::instructions::sh::{IShl, IShr, IUShr, LShl, LShr, LUShr};
use crate::instructions::store::{IStore, IStore0, IStore1, IStore2, IStore3, LStore};
use crate::instructions::swap::Swap;
use crate::rtda::Frame;
use crate::rtda::SlotValue::DoubleSlot;

mod base;
mod nop;
mod r#const;
mod ipush;
mod iload;
mod store;
mod pop;
mod dup;
mod swap;
mod math;
mod sh;
mod and;
mod iinc;
mod cmp;
mod ifcond;
mod ctrl;

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
        0x10 => Some(Box::new(BIPush::new())),
        0x11 => Some(Box::new(SIPush::new())),
        //...
        0x15 => Some(Box::new(ILoad::new())),
        //...
        0x1a => Some(Box::new(ILoad0)),
        0x1b => Some(Box::new(ILoad1)),
        0x1c => Some(Box::new(ILoad2)),
        0x1d => Some(Box::new(ILoad3)),
        // ...
        0x36 => Some(Box::new(IStore::new())),
        0x37 => Some(Box::new(LStore::new())),
        // ...
        0x3b => Some(Box::new(IStore0)),
        0x3c => Some(Box::new(IStore1)),
        0x3d => Some(Box::new(IStore2)),
        0x3e => Some(Box::new(IStore3)),
        0x3f => Some(Box::new(IStore0)),
        0x40 => Some(Box::new(IStore1)),
        0x41 => Some(Box::new(IStore2)),
        0x42 => Some(Box::new(IStore3)),
        // ...
        0x57 => Some(Box::new(Pop)),
        0x58 => Some(Box::new(Pop2)),
        0x59 => Some(Box::new(Dup)),
        0x5a => Some(Box::new(DupX1)),
        0x5b => Some(Box::new(DupX2)),
        0x5c => Some(Box::new(Dup2)),
        0x5d => Some(Box::new(Dup2X1)),
        0x5e => Some(Box::new(Dup2X2)),
        0x5f => Some(Box::new(Swap)),
        0x60 => Some(Box::new(IAdd)),
        0x61 => Some(Box::new(LAdd)),
        0x62 => Some(Box::new(FAdd)),
        0x63 => Some(Box::new(DAdd)),
        0x64 => Some(Box::new(ISub)),
        0x65 => Some(Box::new(LSub)),
        0x66 => Some(Box::new(FSub)),
        0x67 => Some(Box::new(DSub)),
        0x68 => Some(Box::new(IMul)),
        0x69 => Some(Box::new(LMul)),
        0x6a => Some(Box::new(FMul)),
        0x6b => Some(Box::new(DMul)),
        0x6c => Some(Box::new(IDiv)),
        0x6d => Some(Box::new(LDiv)),
        0x6e => Some(Box::new(FDiv)),
        0x6f => Some(Box::new(DDiv)),
        0x70 => Some(Box::new(IRem)),
        0x71 => Some(Box::new(LRem)),
        0x72 => Some(Box::new(FRem)),
        0x73 => Some(Box::new(DRem)),
        0x74 => Some(Box::new(INeg)),
        0x75 => Some(Box::new(LRem)),
        0x76 => Some(Box::new(FRem)),
        0x77 => Some(Box::new(DNeg)),
        0x78 => Some(Box::new(IShl)),
        0x79 => Some(Box::new(LShl)),
        0x7a => Some(Box::new(IShr)),
        0x7b => Some(Box::new(LShr)),
        0x7c => Some(Box::new(IUShr)),
        0x7d => Some(Box::new(LUShr)),
        0x7e => Some(Box::new(IAnd)),
        0x7f => Some(Box::new(LAnd)),
        0x80 => Some(Box::new(IOr)),
        0x81 => Some(Box::new(LOr)),
        0x82 => Some(Box::new(IXOr)),
        0x83 => Some(Box::new(LXOr)),
        0x84 => Some(Box::new(IInc::new())),
        // ...
        0x94 => Some(Box::new(LCMP)),
        0x95 => Some(Box::new(FCMP)),
        0x96 => Some(Box::new(FCMP)),
        0x97 => Some(Box::new(DCMP)),
        0x98 => Some(Box::new(DCMP)),
        0x99 => Some(Box::new(IFEQ::new())),
        0x9a => Some(Box::new(IFNE::new())),
        0x9b => Some(Box::new(IFLT::new())),
        0x9c => Some(Box::new(IFGE::new())),
        0x9d => Some(Box::new(IFGT::new())),
        0x9e => Some(Box::new(IFLE::new())),
        0x9f => Some(Box::new(IfICmpEq::new())),
        0xa0 => Some(Box::new(IfICmpNe::new())),
        0xa1 => Some(Box::new(IfICmpLt::new())),
        0xa2 => Some(Box::new(IfICmpGe::new())),
        0xa3 => Some(Box::new(IfICmpGt::new())),
        0xa4 => Some(Box::new(IfICmpLe::new())),
        // ...
        0xa7 => Some(Box::new(Goto::new())),
        // ...
        _ => None,
    }
}
