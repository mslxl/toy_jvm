use std::fs::read;
use crate::bytecode_reader::BytecodeReader;
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

