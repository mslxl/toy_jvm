
use std::rc::Rc;
use clap::Parser;
use crate::classfile::{ClassFile, ConstantInfo, MemberInfo};
use crate::classpath::ClassPath;
use crate::interp::interpret;

mod rtda;
mod entry;
mod classpath;
mod classfile;
mod bytecode_reader;
mod instructions;
mod interp;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = Some("Practice of book Write Your Own Java Virtual Machine"))]
pub struct CmdArgs {
    /// Classpath
    #[clap(long, value_parser, default_value = ".")]
    classpath: String,

    /// Path to jre
    #[clap(long("XjreOption"), value_parser)]
    x_jre_option: Option<String>,

    #[clap(value_parser)]
    clazz: String,
}

fn load_class(class_name: &str, cp:&ClassPath ) -> ClassFile{
    let data = Rc::new(cp.read_class(class_name).unwrap());
    ClassFile::from(data)
}

fn get_main_method(class_file: &ClassFile) -> Option<MemberInfo> {
    for method in class_file.methods.clone(){
        if method.name == "main" && class_file.constant_pool.get_utf8(method.descriptor_index)  == "([Ljava/lang/String;)V" {
            return Some(method)
        }
    }
    None
}

fn start_jvm(cmd: &CmdArgs){
    let cp = ClassPath::parse(cmd);
    let class_name = cmd.clazz.replace(".", "/");
    let class_file = load_class(&class_name, &cp);
    let method = get_main_method(&class_file).expect(&format!("Can't find static main method at {}", class_name));
    interpret(&method);
}



fn info_class(cmd: &CmdArgs) {
    let cp = ClassPath::parse(cmd);
    let class_name = cmd.clazz.replace(".", "/");
    let bytes = Rc::new(cp.read_class(&class_name).unwrap());
    let clazz = ClassFile::from(bytes);

    println!("version: \t{}.{}", clazz.major_version, clazz.minor_version);
    println!("constants count: \t{}", clazz.constant_pool.len());
    println!("access flags: \t{:#b}", clazz.access_flags);
    println!("this class: \t{}", clazz.constant_pool.get_class_name(clazz.this_class).unwrap());
    println!("super class: \t{:?}", clazz.constant_pool.get_class_name(clazz.super_class));

    let interface_name: Vec<&str> = clazz.interfaces.iter().map(|x| clazz.constant_pool.get_class_name(*x).unwrap()).collect();
    println!("interfaces: \t{:?}", interface_name);
    println!("fields count: \t{}", clazz.fields.len());
    for ref field in clazz.fields {
        println!("\t- {}", field.name);
    }
    println!("method count: \t{}", clazz.methods.len());
    for ref method in clazz.methods {
        println!("\t- {}", method.name);
    }
}

fn main() {
    let args = CmdArgs::parse();
    start_jvm(&args);
}
