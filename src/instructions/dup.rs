use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::Frame;
#[derive(Debug)]
pub struct Dup;
impl Instr for Dup{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        let v = frame .operand_stack.pop();
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v);
    }
}
#[derive(Debug)]
pub struct DupX1;
impl Instr for DupX1{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        let v = frame .operand_stack.pop();
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v);
    }
}
#[derive(Debug)]
pub struct DupX2;
impl Instr for DupX2{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        let v = frame .operand_stack.pop();
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v);
    }
}
#[derive(Debug)]
pub struct Dup2;
impl Instr for Dup2{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        let v = frame .operand_stack.pop();
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v);
    }
}
#[derive(Debug)]
pub struct Dup2X1;
impl Instr for Dup2X1{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        let v = frame .operand_stack.pop();
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v);
    }
}

#[derive(Debug)]
pub struct Dup2X2;

impl Instr for Dup2X2{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        let v = frame .operand_stack.pop();
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v.clone());
        frame.operand_stack.push(v);
    }
}