use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::{Frame, SlotValue};

fn jump_to(frame: &mut Frame, offset: i32){
    frame.set_next_pc(frame.next_pc + offset)
}
#[derive(Debug)]
pub struct IFEQ{
    offset:i32
}
impl IFEQ {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IFEQ{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(val) = frame.operand_stack.pop().value.unwrap() {
            if val == 0 {
                jump_to(frame, self.offset as i32)
            }
            return
        }
        panic!()
    }
}
#[derive(Debug)]
pub struct IFNE{
    offset:i32
}
impl IFNE {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IFNE{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(val) = frame.operand_stack.pop().value.unwrap() {
            if val != 0 {
                jump_to(frame, self.offset as i32)
            }
            return
        }
        panic!()
    }
}
#[derive(Debug)]
pub struct IFLT{
    offset:i32
}
impl IFLT {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IFLT{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(val) = frame.operand_stack.pop().value.unwrap() {
            if val < 0 {
                jump_to(frame, self.offset as i32)
            }
            return
        }
        panic!()
    }
}
#[derive(Debug)]
pub struct IFLE{
    offset:i32
}
impl IFLE {
    pub fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IFLE{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(val) = frame.operand_stack.pop().value.unwrap() {
            if val <= 0 {
                jump_to(frame, self.offset as i32)
            }
            return
        }
        panic!()
    }
}
#[derive(Debug)]
pub struct IFGT{
    offset:i32
}
impl IFGT {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IFGT{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(val) = frame.operand_stack.pop().value.unwrap() {
            if val > 0 {
                jump_to(frame, self.offset as i32)
            }
            return
        }
        panic!()
    }
}
#[derive(Debug)]
pub struct IFGE{
    offset:i32
}
impl IFGE {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IFGE{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(val) = frame.operand_stack.pop().value.unwrap() {
            if val >= 0 {
                jump_to(frame, self.offset as i32)
            }
            return
        }
        panic!()
    }
}

#[derive(Debug)]
pub struct IfICmpEq{
    offset:i32
}
impl IfICmpEq {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}

impl Instr for IfICmpEq{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }
    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap(){
                if v1 == v2 {
                    jump_to(frame,self.offset as i32)
                }
            }
            return
        }
        panic!()
    }
}

#[derive(Debug)]
pub struct IfICmpNe{
    offset:i32
}
impl IfICmpNe {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IfICmpNe{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }
    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap(){
                if v1 != v2 {
                    jump_to(frame,self.offset as i32)
                }
            }
            return
        }
        panic!()
    }
}

#[derive(Debug)]
pub struct IfICmpLt{
    offset:i32
}
impl IfICmpLt {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IfICmpLt{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }
    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap(){
                if v1 < v2 {
                    jump_to(frame,self.offset as i32)
                }
            }
            return
        }
        panic!()
    }
}

#[derive(Debug)]
pub struct IfICmpLe{
    offset:i32
}
impl IfICmpLe {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IfICmpLe{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }
    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap(){
                if v1 <= v2 {
                    jump_to(frame,self.offset as i32)
                }
            }
            return
        }
        panic!()
    }
}

#[derive(Debug)]
pub struct IfICmpGt{
    offset:i32
}
impl IfICmpGt {
    pub(crate) fn new() -> Self{
        Self{
            offset: 0
        }
    }
}
impl Instr for IfICmpGt{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }
    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap(){
                if v1 > v2 {
                    jump_to(frame,self.offset as i32)
                }
            }
            return
        }
        panic!()
    }
}

#[derive(Debug)]
pub struct IfICmpGe{
    offset:i32
}
impl Instr for IfICmpGe{
    fn fetch(&mut self, reader: &mut BytecodeReader) {
        self.offset = reader.read_i16() as i32
    }
    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(v2) = frame.operand_stack.pop().value.unwrap() {
            if let SlotValue::IntSlot(v1) = frame.operand_stack.pop().value.unwrap(){
                if v1 >= v2 {
                    jump_to(frame,self.offset as i32)
                }
            }
            return
        }
        panic!()
    }
}
impl IfICmpGe {
    pub fn new() -> Self{
        Self{
            offset: 0
        }
    }
}