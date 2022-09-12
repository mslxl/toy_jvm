use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::{Frame, SlotValue};


fn lstore(frame : &mut Frame, index:usize){
    let slot = frame.operand_stack.pop();
    if let Some(SlotValue::LongSlot(_)) = slot.value {
        frame.local_vars.set_slot(index, slot)
    }else{
        panic!("lstore: var at stack top is not a long, it's: {:?}", slot)
    }
}

struct LStore {
    index: usize
}



impl Instr for LStore{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize
    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, self.index)
    }
}
struct LStore0;

impl Instr for LStore0{
    fn fetch(&mut self,_reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, 0)
    }
}
struct LStore1;
impl Instr for LStore1{
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, 1)
    }
}
struct LStore2;
impl Instr for LStore2{
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, 2)
    }
}
struct LStore3;
impl Instr for LStore3{
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, 3)
    }
}