use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::{Frame, SlotValue};
#[derive(Debug)]
pub struct IAnd;
#[derive(Debug)]
pub struct LAnd;


impl Instr for IAnd{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_int(v1 & v2);
                return
            }
        }
        panic!()
    }
}

impl Instr for LAnd{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::LongSlot(v1) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::LongSlot(v2) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_long(v1 & v2);
                return
            }
        }
        panic!()
    }
}

#[derive(Debug)]
pub struct IOr;
#[derive(Debug)]
pub struct LOr;


impl Instr for IOr{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_int(v1 | v2);
                return
            }
        }
        panic!()
    }
}

impl Instr for LOr{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::LongSlot(v1) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::LongSlot(v2) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_long(v1 | v2);
                return
            }
        }
        panic!()
    }
}

#[derive(Debug)]
pub struct IXOr;
#[derive(Debug)]
pub struct LXOr;


impl Instr for IXOr{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_int(v1 ^ v2);
                return
            }
        }
        panic!()
    }
}

impl Instr for LXOr{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::LongSlot(v1) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::LongSlot(v2) = frame.operand_stack.pop().value.unwrap() {
                frame.operand_stack.push_long(v1 ^ v2);
                return
            }
        }
        panic!()
    }
}
