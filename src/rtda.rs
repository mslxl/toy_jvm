use std::cell::RefCell;
use std::fmt::{Display, Formatter, write};
use std::rc::Rc;

#[derive(Debug,Clone)]
pub struct Obj {
    //todo
}

pub struct Thread {
    pc: i32,
    stack: Stack,
}

#[derive(Clone, Debug)]
pub struct Slot{
    pub value: Option<SlotValue>,
}
#[derive(Debug,Clone)]
pub enum SlotValue{
    IntSlot(i32),
    FloatSlot(f32),
    LongSlot(i64),
    DoubleSlot(f64),
    RefSlot(Rc<RefCell<Obj>>)
}

pub struct OperandStack(pub Vec<Slot>);

pub struct LocalVars(pub Vec<Slot>);


pub struct Frame {
    pub local_vars: LocalVars,
    pub operand_stack: OperandStack,

    pub next_pc: i32,
    pub thread: *mut Thread,
}
impl Frame{
    pub fn set_next_pc(&mut self, pc:i32) {
        self.next_pc = pc;
    }
}

impl Display for Frame{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Frame:\n").unwrap();
        write!(f,"\tVar table:\n").unwrap();
        for slot in self.local_vars.0.iter() {
            write!(f, "\t+ {:?}\n", slot.value).unwrap();
        }
        write!(f,"\tStack:\n").unwrap();
        for slot in self.operand_stack.0.iter() {
            write!(f, "\t+ {:?}\n", slot.value).unwrap();
        }
        write!(f, "\t+ Bottom\n")
    }
}

struct Stack(Vec<Frame>); // it didn't follow JVM rules, i.e. it never throw StackOverflowError due to it never be full

impl OperandStack{
    fn new(size:usize) -> Self{
        OperandStack(vec![Slot::new(); size])
    }

    pub fn push(&mut self, slot: Slot){
        self.0.push(slot);
    }
    pub fn pop(&mut self) -> Slot{
        self.0.pop().unwrap()
    }
    pub fn push_ref(&mut self, reference: Option<Rc<RefCell<Obj>>>){
        match reference {
           None => self.push(Slot::new()),
            Some(obj_ref) => {
                let mut slot = Slot::new();
                slot.value = Some(SlotValue::RefSlot(obj_ref))
            }
        }
    }

    pub fn push_double(&mut self, value:f64) {
        let mut  slot = Slot::new();
        slot.set_double(value);
        self.push(slot)
    }

    pub fn push_int(&mut self, value:i32) {
        let mut slot = Slot::new();
        slot.set_int(value);
        self.push(slot)
    }

    pub fn push_float(&mut self, value:f32) {
        let mut slot = Slot::new();
        slot.set_float(value);
        self.push(slot)
    }

    pub fn push_long(&mut self, value: i64) {
        let mut slot = Slot::new();
        slot.set_long(value);
        self.push(slot);
    }
}

impl LocalVars{
    fn new(size:usize) -> Self{
        Self(vec![Slot::new(); size])
    }

    pub fn get_slot(&mut self, index:usize) -> Option<&mut Slot>{
        self.0.get_mut(index)
    }

    pub fn set_slot(&mut self, index:usize, value:Slot){
        self.0[index] = value
    }
}
impl Slot{
    pub fn new() -> Self{
        Slot{
            value:None
        }
    }
    pub fn set_int(&mut self, value: i32){
        self.value = Some(SlotValue::IntSlot(value))
    }
    pub fn get_int(&self)->i32{
        if let SlotValue::IntSlot(value) = self.value.as_ref().unwrap() {
            *value
        }else{
            panic!("Type mismatched. Expect int, got {:?}", self.value);
        }
    }

    pub fn set_float(&mut self, value: f32){
        self.value = Some(SlotValue::FloatSlot(value))
    }
    pub fn get_float(&self)->f32{
        if let SlotValue::FloatSlot(value) = self.value.as_ref().unwrap() {
            *value
        }else{
            panic!("Type mismatched. Expect float, got {:?}", self.value);
        }
    }

    pub fn set_long(&mut self, value: i64){
        self.value = Some(SlotValue::LongSlot(value))
    }
    pub fn get_long(&self)->i64{
        if let SlotValue::LongSlot(value) = self.value.as_ref().unwrap() {
            *value
        }else{
            panic!("Type mismatched. Expect long, got {:?}", self.value);
        }
    }

    pub fn set_double(&mut self, value: f64){
        self.value = Some(SlotValue::DoubleSlot(value))
    }
    pub fn get_double(&self)->f64{
        if let SlotValue::DoubleSlot(value) = self.value.as_ref().unwrap() {
            *value
        }else{
            panic!("Type mismatched. Expect double, got {:?}", self.value);
        }
    }
}


impl Stack {
    fn new(size:usize) -> Self {
        Stack(Vec::new())
    }
}

impl Thread {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Stack::new(1024),
        }
    }
    pub fn new_frame(&mut self, max_locals:usize, max_stacks:usize) -> Frame {
        let ptr = self as *mut Self;
        Frame{
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_stacks),
            next_pc: 0,
            thread: ptr,
        }
    }
    pub fn pc(&self) -> i32 {
        self.pc
    }
    pub fn set_pc(&mut self, pc: i32) {
        self.pc = pc
    }
    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.0.push(frame)
    }
    pub fn pop_frame(&mut self) -> Frame {
        self.stack.0.pop().unwrap()
    }
    pub fn current_frame(&self) -> &Frame {
        self.stack.0.last().unwrap()
    }
}

impl Display for Thread{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "---Thread Display Beg---\n").unwrap();
        for frame in self.stack.0.iter() {
            write!(f, "{}", frame).unwrap();
        }
        write!(f, "---Thread Display End---\n")
    }
}