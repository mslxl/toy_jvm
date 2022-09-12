use std::fs::read;
use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::{Frame, SlotValue};

fn iload(frame: &mut Frame, index:usize){
    if let Some(SlotValue::IntSlot(integer)) = frame.local_vars.get_slot(index).unwrap().value {
        frame.operand_stack.push_int(integer)
    }else{
        panic!("iload: var at localvar table is not a integer, it's: {:?}", frame.local_vars.get_slot(index))
    }
}

struct ILoad{
    index: usize
}

impl Instr for ILoad{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize
    }

    fn exec(&self, frame: &mut Frame) {
        iload(frame, self.index)
    }
}

struct ILoad0;
impl Instr for ILoad0 {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        iload(frame, 0)
    }
}
struct ILoad1;
impl Instr for ILoad1 {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        iload(frame, 1)
    }
}
struct ILoad2;
impl Instr for ILoad2 {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        iload(frame, 2)
    }
}
struct ILoad3;
impl Instr for ILoad3 {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        iload(frame, 3)
    }
}