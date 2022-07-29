use clap::Parser;

mod entry;
mod classpath;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=Some("Practice of book Write Your Own Java Virtual Machine"))]
struct CmdArgs{

    /// Classpath
    #[clap(long, value_parser)]
    classpath: Option<String>,

    /// Path to jre
    #[clap(long("XjreOption"), value_parser)]
    x_jre_option: Option<String>,

}

fn main() {
    let args = CmdArgs::parse();

}
