use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::{Frame, SlotValue};

#[derive(Debug)]
pub struct IShl;
#[derive(Debug)]
pub struct IShr;
#[derive(Debug)]
pub struct IUShr;
#[derive(Debug)]
pub struct LShl;
#[derive(Debug)]
pub struct LShr;
#[derive(Debug)]
pub struct LUShr;

impl Instr for IShl{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_int(v1 << v2);
                return
            }
        }
        panic!()
    }
}

impl Instr for IShr {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_int(v1 >> v2);
                return
            }
        }
        panic!()
    }
}

impl Instr for IUShr {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_int( ((v1 as u32) >> v2) as i32);
                return
            }
        }
        panic!()
    }
}

impl Instr for LShl{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::LongSlot(v1) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_long( v1 << v2);
                return
            }
        }
        panic!()
    }
}
impl Instr for LShr{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::LongSlot(v1) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_long( v1 >> v2);
                return
            }
        }
        panic!()
    }
}
impl Instr for LUShr{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::LongSlot(v1) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_long( ((v1 as u64) >> v2) as i64);
                return
            }
        }
        panic!()
    }
}
