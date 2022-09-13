use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::Frame;
#[derive(Debug)]
struct Swap;
impl Instr for Swap{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        let s1 = frame.operand_stack.pop();
        let s2 = frame.operand_stack.pop();
        frame.operand_stack.push(s1);
        frame.operand_stack.push(s2);
    }
}