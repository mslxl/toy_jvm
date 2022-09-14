use std::rc::Rc;
use crate::bytecode_reader::BytecodeReader;
use crate::classfile::{AttributeInfo, MemberInfo};
use crate::instructions::bytes_into_instr;
use crate::rtda::{Frame, Thread};

pub fn interpret(method_info:&MemberInfo){
    if let AttributeInfo::Code {
        max_stack,
        max_locals,
        code,
        exception_table,
        attributes
    } = method_info.code_attr() {
        let mut thread = Thread::new();
        let frame = thread.new_frame(max_locals as usize, max_stack as usize);
        thread.push_frame(frame);

        loop_interp(&mut thread, &code)
    }
}

fn loop_interp(thread: &mut Thread, bytecode: &Vec<u8>){
    let bytecode = bytecode.clone();
    let mut reader = BytecodeReader::new(Rc::new(bytecode));
    let mut frame = thread.pop_frame();
    loop {
        let pc = frame.next_pc;
        thread.set_pc(pc);

        // decode
        reader.set_pc(pc as usize);
        let opcode = reader.read_u8();
        if let Some(mut instr) = bytes_into_instr(opcode) {
            instr.fetch(&mut reader);
            frame.next_pc = reader.pc() as i32;

            // execute
            println!("pc: {} instr: {:?}", pc, instr);
            instr.exec(&mut frame);
        }else{
            eprint!("{}", frame);
            panic!("Unsupported instr opcode {:x} at pc {}", opcode, pc)
        }
    }


}