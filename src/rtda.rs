use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug,Clone)]
struct Obj {
    //todo
}

struct Thread {
    pc: i32,
    stack: Stack,
}

#[derive(Clone)]
struct Slot{
    value: Option<SlotValue>,
}
#[derive(Debug,Clone)]
enum SlotValue{
    IntSlot(i32),
    FloatSlot(f32),
    LongSlot(i64),
    DoubleSlot(f64),
    RefSlot(Rc<RefCell<Obj>>)
}

struct OperandStack(Vec<Slot>);

struct LocalVars(Vec<Slot>);


struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

struct Stack(Vec<Frame>); // it didn't follow JVM rules, i.e. it never throw StackOverflowError due to it never be full

impl OperandStack{
    fn new(size:usize) -> Self{
        OperandStack(vec![Slot::new(); size])
    }

    fn push(&mut self, slot: Slot){
        self.0.push(slot);
    }
}

impl LocalVars{
    fn new(size:usize) -> Self{
        Self(vec![Slot::new(); size])
    }

    fn get_slot(&mut self, index:usize) -> Option<&mut Slot>{
        self.0.get_mut(index)
    }
}
impl Slot{
    fn new() -> Self{
        Slot{
            value:None
        }
    }
    fn set_int(&mut self, value: i32){
        self.value = Some(SlotValue::IntSlot(value))
    }
    fn get_int(&self)->i32{
        if let SlotValue::IntSlot(value) = self.value.as_ref().unwrap() {
            *value
        }else{
            panic!("Type mismatched. Expect int, got {:?}", self.value);
        }
    }

    fn set_float(&mut self, value: f32){
        self.value = Some(SlotValue::FloatSlot(value))
    }
    fn get_float(&self)->f32{
        if let SlotValue::FloatSlot(value) = self.value.as_ref().unwrap() {
            *value
        }else{
            panic!("Type mismatched. Expect float, got {:?}", self.value);
        }
    }

    fn set_long(&mut self, value: i64){
        self.value = Some(SlotValue::LongSlot(value))
    }
    fn get_long(&self)->i64{
        if let SlotValue::LongSlot(value) = self.value.as_ref().unwrap() {
            *value
        }else{
            panic!("Type mismatched. Expect long, got {:?}", self.value);
        }
    }

    fn set_double(&mut self, value: f64){
        self.value = Some(SlotValue::DoubleSlot(value))
    }
    fn get_double(&self)->f64{
        if let SlotValue::DoubleSlot(value) = self.value.as_ref().unwrap() {
            *value
        }else{
            panic!("Type mismatched. Expect double, got {:?}", self.value);
        }
    }
}

impl Frame{
    fn new(max_locals:usize, max_stack:usize) -> Self{
        Frame{
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_stack)
        }
    }
}

impl Stack {
    fn new(size:usize) -> Self {
        Stack(Vec::new())
    }
}

impl Thread {
    fn new() -> Self {
        Self {
            pc: 0,
            stack: Stack::new(1024),
        }
    }
    fn pc(&self) -> i32 {
        self.pc
    }
    fn set_pc(&mut self, pc: i32) {
        self.pc = pc
    }
    fn push_frame(&mut self, frame: Frame) {
        self.stack.0.push(frame)
    }
    fn pop_frame(&mut self) -> Frame {
        self.stack.0.pop().unwrap()
    }
    fn current_frame(&self) -> &Frame {
        self.stack.0.last().unwrap()
    }
}
