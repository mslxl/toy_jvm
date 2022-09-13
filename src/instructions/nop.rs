use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::Frame;

#[derive(Debug)]
pub struct NopInstr;

impl Instr for NopInstr{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {

    }
}