use std::fs::read;
use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::{Frame, SlotValue};
#[derive(Debug)]
pub struct IInc{
    index: usize,
    const_value: i32
}

impl IInc{
    pub(crate) fn new() -> Self{
        Self{
            index: 0,
            const_value: 0
        }
    }
}

impl Instr for IInc{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize;
        self.const_value = reader.read_i8() as i32;
    }

    fn exec(&self, frame: &mut Frame) {
        let mut val = frame.local_vars.get_slot(self.index).unwrap().clone();
        if let SlotValue::IntSlot(v) = val.value.as_ref().unwrap().clone() {
            val.set_int(v + self.const_value)
        }else{
            panic!()
        }
        frame.local_vars.set_slot(self.index, val);
    }
}