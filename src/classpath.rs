use std::env;
use std::os;
use std::path::Path;
use std::path::PathBuf;

use crate::entry::Entry;
use crate::CmdArgs;

#[derive(Debug)]
pub struct ClassPath {
    boot_classpath: Entry,
    ext_classpath: Entry,
    user_classpath: Entry,
}

impl ClassPath {
    fn get_jre_dir(args: &CmdArgs) -> PathBuf {
        if let Some(dir) = &args.x_jre_option {
            PathBuf::from(dir)
        } else if env::current_dir().unwrap().join("jre").exists() {
            env::current_dir().unwrap().join("jre")
        } else if let Some(dir) = env::var_os("JAVA_HOME") {
            let dir = dir.to_str().unwrap().to_owned();
            PathBuf::from(dir)
        } else {
            panic!("Can not find jre folder!")
        }
    }


    pub fn parse(args: &CmdArgs) -> ClassPath {
        let jre_dir = ClassPath::get_jre_dir(args);
        let jre_lib_path = jre_dir.join("lib/*");

        let boot_classpath = Entry::from(jre_lib_path.to_str().unwrap());

        let jre_ext_path = jre_dir.join("lib/ext/*");
        let ext_classpath = Entry::from(jre_ext_path.to_str().unwrap());


        let user_classpath = Entry::from(&args.classpath);

        ClassPath {
            boot_classpath,
            ext_classpath,
            user_classpath,
        }
    }
    pub fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        let class_name = class_name.to_string() + ".class";
        if let Ok(data) = self.boot_classpath.read_class(&class_name) {
            Ok(data)
        } else if let Ok(data) = self.ext_classpath.read_class(&class_name) {
            Ok(data)
        } else {
            self.user_classpath.read_class(&class_name)
        }
    }
}
