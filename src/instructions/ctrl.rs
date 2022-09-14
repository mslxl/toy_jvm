use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::{Frame, SlotValue};

#[derive(Debug)]
pub struct Goto{
    offset:i32
}
impl Goto{
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for Goto{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        let i = reader.read_i16();
        self.offset = i as i32
    }
    fn exec(&self, frame: &mut Frame) {
        let thread_pc = unsafe {
            frame.thread.as_ref().unwrap().pc()
        };
        frame.set_next_pc(thread_pc + self.offset as i32)
    }
}