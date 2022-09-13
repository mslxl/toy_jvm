use std::fmt::Debug;
use crate::bytecode_reader::BytecodeReader;
use crate::rtda::Frame;

pub trait Instr: Debug {
    fn fetch(&mut self, reader: &mut BytecodeReader);
    fn exec(&self, frame: &mut Frame);
}

