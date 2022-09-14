use std::fs::read;
use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::Frame;

#[derive(Debug)]
pub struct BIPush {
    value: i8
}
impl BIPush{
    pub(crate) fn new() -> Self{
        Self{value: 0}
    }
}

impl Instr for BIPush{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.value = reader.read_i8();
    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(self.value as i32)
    }
}
#[derive(Debug)]
pub struct SIPush{
    value: i16
}

impl SIPush{
    pub(crate) fn new() -> Self{
        Self{
            value: 0
        }
    }
}
impl Instr for SIPush{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.value = reader.read_i16();
    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(self.value as i32);
    }
}