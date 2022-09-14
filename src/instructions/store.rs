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
#[derive(Debug)]
pub struct LStore {
    index: usize
}

impl LStore{
    pub(crate) fn new() -> Self{
        Self{
            index: 0
        }
    }
}


impl Instr for LStore{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize
    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, self.index)
    }
}
#[derive(Debug)]
pub struct LStore0;

impl Instr for LStore0{
    fn fetch(&mut self,_reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, 0)
    }
}
#[derive(Debug)]
pub struct LStore1;
impl Instr for LStore1{
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, 1)
    }
}
#[derive(Debug)]
pub struct LStore2;
impl Instr for LStore2{
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, 2)
    }
}
#[derive(Debug)]
pub struct LStore3;
impl Instr for LStore3{
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        lstore(frame, 3)
    }
}

fn istore(frame : &mut Frame, index:usize){
    let slot = frame.operand_stack.pop();
    if let Some(SlotValue::IntSlot(_)) = slot.value {
        frame.local_vars.set_slot(index, slot)
    }else{
        panic!("istore: var at stack top is not a int, it's: {:?}", slot)
    }
}

#[derive(Debug)]
pub struct IStore{
    index:usize
}

impl IStore{
    pub(crate) fn new() ->Self{
        Self{
            index: 0
        }
    }
}

impl Instr for IStore{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u8() as usize
    }

    fn exec(&self, frame: &mut Frame) {
        istore(frame, self.index)
    }
}

#[derive(Debug)]
pub struct IStore0;
impl Instr for IStore0{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        istore(frame, 0)
    }
}
#[derive(Debug)]
pub struct IStore1;
impl Instr for IStore1{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        istore(frame, 1)
    }
}
#[derive(Debug)]
pub struct IStore2;
impl Instr for IStore2{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        istore(frame, 2)
    }
}
#[derive(Debug)]
pub struct IStore3;
impl Instr for IStore3{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        istore(frame, 3)
    }
}