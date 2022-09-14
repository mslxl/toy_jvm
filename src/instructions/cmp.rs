use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::{Frame, SlotValue};

#[derive(Debug)]
pub struct LCMP;
impl Instr for LCMP{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::LongSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::LongSlot(v1) = frame.operand_stack.pop().value.unwrap() {
                if v1 > v2 {
                    frame.operand_stack.push_int(1)
                }else if v1 == v2{
                    frame.operand_stack.push_int(0)
                }else {
                    frame.operand_stack.push_int(-1)
                }
                return
            }
        }
        panic!()
    }
}

#[derive(Debug)]
pub struct FCMP;
impl Instr for FCMP{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::FloatSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::FloatSlot(v1) = frame.operand_stack.pop().value.unwrap() {
                if v1 > v2 {
                    frame.operand_stack.push_int(1)
                }else if v1 == v2{
                    frame.operand_stack.push_int(0)
                }else {
                    frame.operand_stack.push_int(-1)
                }
                return
            }
        }
        panic!()
    }
}


#[derive(Debug)]
pub struct DCMP;
impl Instr for DCMP{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::DoubleSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::DoubleSlot(v1) = frame.operand_stack.pop().value.unwrap() {
                if v1 > v2 {
                    frame.operand_stack.push_int(1)
                }else if v1 == v2{
                    frame.operand_stack.push_int(0)
                }else {
                    frame.operand_stack.push_int(-1)
                }
                return
            }
        }
        panic!()
    }
}

