use std::env;
use std::os;

use crate::entry;
use crate::CmdArgs;

#[derive(Debug)]
struct ClassPath {
    boot_classpath: entry::Entry,
    ext_classpath: entry::Entry,
    user_classpath: entry::Entry,
}

impl ClassPath {
    fn get_jre_dir(args: &CmdArgs) -> String {
        if let Some(dir) = args.x_jre_option {
            dir.clone()
        } else if env::current_dir().unwrap().join("jre").exists() {
            env::current_dir()
                .unwrap()
                .join("jre")
                .to_str()
                .unwrap()
                .to_owned()
        } else if let Some(dir) = env::var_os("JAVA_HOME") {
            dir.to_str().unwrap().to_owned()
        } else {
            panic!("Can not find jre folder!")
        }
    }

    fn parse(args: &CmdArgs) -> ClassPath {
        let jreDir = ClassPath::get_jre_dir(args);
    }
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {}
}
