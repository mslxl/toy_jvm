use crate::bytecode_reader::BytecodeReader;
use crate::instructions::base::Instr;
use crate::rtda::{Frame, Slot, SlotValue};

fn double_op<F>(frame:&mut Frame, op: F) where F: FnOnce(f64,f64)->f64{
    if let SlotValue::DoubleSlot(d1) = frame.operand_stack.pop().value.unwrap() {
        if let SlotValue::DoubleSlot(d2) = frame.operand_stack.pop().value.unwrap() {
            let mut s = Slot::new();
            s.set_double(op(d1,d2));
            frame.operand_stack.push(s);
            return;
        }
    }
    panic!("math: var at stack is not a double")
}
fn float_op<F>(frame:&mut Frame, op: F) where F: FnOnce(f32,f32)->f32{
    if let SlotValue::FloatSlot(d1) = frame.operand_stack.pop().value.unwrap() {
        if let SlotValue::FloatSlot(d2) = frame.operand_stack.pop().value.unwrap() {
            let mut s = Slot::new();
            s.set_float(op(d1,d2));
            frame.operand_stack.push(s);
            return;
        }
    }
    panic!("math: var at stack is not a float")
}

fn int_op<F>(frame:&mut Frame, op: F) where F: FnOnce(i32,i32)->i32{
    if let SlotValue::IntSlot(d1) = frame.operand_stack.pop().value.unwrap() {
        if let SlotValue::IntSlot(d2) = frame.operand_stack.pop().value.unwrap() {
            let mut s = Slot::new();
            s.set_int(op(d1,d2));
            frame.operand_stack.push(s);
            return;
        }
    }
    panic!("math: var at stack is not a int")
}

fn long_op<F>(frame:&mut Frame, op: F) where F: FnOnce(i64,i64)->i64{
    if let SlotValue::LongSlot(d1) = frame.operand_stack.pop().value.unwrap() {
        if let SlotValue::LongSlot(d2) = frame.operand_stack.pop().value.unwrap() {
            let mut s = Slot::new();
            s.set_long(op(d1,d2));
            frame.operand_stack.push(s);
            return;
        }
    }
    panic!("math: var at stack is not a long")
}

#[derive(Debug)]
struct DRem;
#[derive(Debug)]
struct FRem;
#[derive(Debug)]
struct IRem;
#[derive(Debug)]
struct LRem;
impl Instr for DRem{
    fn fetch(&mut self, _reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        double_op(frame, |a,b|{ a % b});
    }
}

impl Instr for FRem{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        float_op(frame, |a,b| {a % b})
    }
}
impl Instr for IRem{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        int_op(frame, |a,b| {a % b})
    }
}



impl Instr for LRem{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        long_op(frame, |a,b| {a % b})
    }
}


#[derive(Debug)]
struct DAdd;
#[derive(Debug)]
struct IAdd;
#[derive(Debug)]
struct FAdd;
#[derive(Debug)]
struct LAdd;

impl Instr for DAdd {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        double_op(frame, |a,b| {a + b})
    }
}
impl Instr for IAdd{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        int_op(frame, |a,b| {a  + b})
    }
}
impl Instr for FAdd{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        float_op(frame, |a,b| {a  + b})
    }
}

impl Instr for LAdd{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        long_op(frame, |a,b| {a  + b})
    }
}

#[derive(Debug)]
struct DSub;
#[derive(Debug)]
struct ISub;
#[derive(Debug)]
struct LSub;
#[derive(Debug)]
struct FSub;


impl Instr for DSub{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        double_op(frame, |a,b| { a - b })
    }
}

impl Instr for FSub{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        float_op(frame, |a,b| {a  - b})
    }
}

impl Instr for ISub{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        int_op(frame, |a,b| {a  - b})
    }
}
impl Instr for LSub{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        long_op(frame, |a,b| {a  - b})
    }
}
#[derive(Debug)]
struct DMul;
#[derive(Debug)]
struct FMul;
#[derive(Debug)]
struct IMul;
#[derive(Debug)]
struct LMul;
impl Instr for DMul {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame){
        double_op(frame, |a,b| {a * b})
    }
}
impl Instr for FMul{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        float_op(frame, |a,b| {a  * b})
    }
}
impl Instr for IMul{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        int_op(frame, |a,b| {a  * b})
    }
}
impl Instr for LMul{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        long_op(frame, |a,b| {a  * b})
    }
}

#[derive(Debug)]
struct DDiv;
#[derive(Debug)]
struct FDiv;
#[derive(Debug)]
struct IDiv;
#[derive(Debug)]
struct LDiv;


impl Instr for DDiv {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        double_op(frame, |a,b| {a/b})
    }
}

impl Instr for FDiv{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        float_op(frame, |a,b| {a / b})
    }
}

impl Instr for IDiv{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        int_op(frame, |a,b| {a / b})
    }
}

impl Instr for LDiv{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        long_op(frame, |a,b| {a / b})
    }
}
#[derive(Debug)]
struct FNeg;
#[derive(Debug)]
struct DNeg;
#[derive(Debug)]
struct INeg;
#[derive(Debug)]
struct LNeg;

impl Instr for FNeg{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::FloatSlot(f) = frame.operand_stack.pop().value.unwrap() {
            frame.operand_stack.push_float(-f)
        }else{
            panic!("math: var at stack is not a float")
        }
    }
}

impl Instr for DNeg {
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::DoubleSlot(f) = frame.operand_stack.pop().value.unwrap() {
            frame.operand_stack.push_double(-f)
        }else{
            panic!("math: var at stack is not a double")
        }
    }
}

impl Instr for INeg{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::IntSlot(f) = frame.operand_stack.pop().value.unwrap() {
            frame.operand_stack.push_int(-f)
        }else{
            panic!("math: var at stack is not a int")
        }
    }
}

impl Instr for LNeg{
    fn fetch(&mut self, reader: &mut BytecodeReader) {

    }

    fn exec(&self, frame: &mut Frame) {
        if let SlotValue::LongSlot(f) = frame.operand_stack.pop().value.unwrap() {
            frame.operand_stack.push_long(-f)
        }else{
            panic!("math: var at stack is not a int")
        }
    }
}



