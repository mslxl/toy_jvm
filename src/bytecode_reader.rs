use std::rc::Rc;

pub struct BytecodeReader{
    code: Rc<Vec<u8>>,
    pc: usize,
}

impl BytecodeReader{
    pub fn pc(&self) -> usize{
        self.pc
    }
    pub fn new(code: Rc<Vec<u8>>)->Self {
        Self{
            code,
            pc: 0
        }
    }

    pub fn set_pc(&mut self, pc:usize) {
        self.pc = pc;
    }
    pub fn reset(&mut self, code:&Rc<Vec<u8>>, pc:usize){
        self.code = Rc::clone(code);
        self.peek(pc);
    }
    pub fn peek(&mut self, pc:usize){
        self.pc = pc;
    }

    pub fn read_u8(&mut self) -> u8{
        let i = self.code[self.pc];
        self.pc+=1;
        i
    }
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8().try_into().unwrap()
    }

    pub fn read_u16(&mut self)->u16{
        let b1:u16 = self.read_u8().try_into().unwrap();
        let b2:u16 = self.read_u8().try_into().unwrap();
        b1 << 8 | b2
    }

    pub fn read_i16(&mut self) -> i16 {
        let b = self.read_u16();
        b as i16
    }

    pub fn read_i32(&mut self) -> i32{
        let b1: i32 = self.read_u8().try_into().unwrap();
        let b2: i32 = self.read_u8().try_into().unwrap();
        let b3: i32 = self.read_u8().try_into().unwrap();
        let b4: i32 = self.read_u8().try_into().unwrap();
        (b1 << 24) | (b2 << 16) | (b3 << 8) | b4
    }

}