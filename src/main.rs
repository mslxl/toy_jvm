extern crate core;

use clap::Parser;
use crate::classpath::ClassPath;

mod entry;
mod classpath;

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
    println!("class data: {:?}", cp.read_class(&class_name));
}

fn main() {
    let args = CmdArgs::parse();
    start_jvm(&args);
}
