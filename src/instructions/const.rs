use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::Frame;

struct AConstNull;

impl Instr for AConstNull{
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_ref(None)
    }
}


struct DConst0;

impl Instr for DConst0 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_double(0.0);
    }
}

struct DConst1;

impl Instr for DConst1 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_double(1.0)
    }
}

struct FConst0;

impl Instr for FConst0 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_float(0.0);
    }
}

struct FConst1;

impl Instr for FConst1 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_float(1.0)
    }
}

struct FConst2;

impl Instr for FConst2 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_float(2.0)
    }
}

struct IConstM1;

impl Instr for IConstM1 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(-1)
    }
}


struct IConst0;

impl Instr for IConst0 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(0)
    }
}


struct IConst1;


impl Instr for IConst1 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(1)
    }
}

struct IConst2;


impl Instr for IConst2 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(2)
    }
}

struct IConst3;


impl Instr for IConst3 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(3)
    }
}

struct IConst4;


impl Instr for IConst4 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(4)
    }
}

struct IConst5;


impl Instr for IConst5 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_int(5)
    }
}

struct LConst0;


impl Instr for LConst0 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_long(0)
    }
}

struct LConst1;

impl Instr for LConst1 {
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        frame.operand_stack.push_long(1)
    }
}


