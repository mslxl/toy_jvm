extern crate core;

use clap::Parser;
use crate::classfile::ClassFile;
use crate::classpath::ClassPath;

mod entry;
mod classpath;
mod classfile;

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

fn start_jvm(cmd: &CmdArgs) {
    let cp = ClassPath::parse(cmd);

    println!("classpath: {:#?}", cp);
    let class_name = cmd.clazz.replace(".", "/");
    let bytes = cp.read_class(&class_name).unwrap();
    let clazz = ClassFile::from(bytes);
    println!("Class: {:#?}",clazz)
}

fn main() {
    let args = CmdArgs::parse();
    start_jvm(&args);
}
