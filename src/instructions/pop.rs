use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::Frame;

#[derive(Debug)]
pub struct Pop;
#[derive(Debug)]
pub struct Pop2;

impl Instr for Pop{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.pop();
    }
}

impl Instr for Pop2 {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.pop();
        frame.operand_stack.pop();
    }
}