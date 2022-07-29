use std::{
    fs,
    io::{Error, Read},
    path::{Path, PathBuf},
};

use glob::glob;

#[derive(Debug)]
pub enum Entry {
    DirEntry(PathBuf),
    ZipEntry(PathBuf),
    CompositeEntry(Vec<Box<Entry>>),
    WildcardEntry(Vec<Box<Entry>>),
}

impl Entry {
    pub fn from(path: &str) -> Entry {
        if path.contains(';') {
            let mut entries: Vec<Box<Entry>> = Vec::new();
            for i in path.split(';') {
                let entry = Entry::from(i);
                entries.push(Box::new(entry));
            }
            Entry::CompositeEntry(entries)
        } else if path.ends_with(".jar") || path.ends_with(".zip") {
            Entry::ZipEntry(PathBuf::from(path))
        } else if path.ends_with("*") {
            let data = glob(path)
                .unwrap()
                .map(|x| Box::new(Entry::from(x.unwrap().to_str().unwrap())))
                .collect();
            Entry::WildcardEntry(data)
        } else {
            Entry::DirEntry(PathBuf::from(path))
        }
    }
    pub fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        match self {
            Entry::DirEntry(dir) => fs::read(dir.join(class_name)).map_err(|x| x.to_string()),
            Entry::ZipEntry(file) => {
                let file = fs::File::open(file).unwrap();
                let mut archive = zip::ZipArchive::new(file).unwrap();
                let mut clz = archive.by_name(class_name).map_err(|x| x.to_string())?;
                let mut data: Vec<u8> = Vec::new();
                clz.read_to_end(&mut data).map_err(|x| x.to_string())?;
                Ok(data)
            }
            Entry::CompositeEntry(comp) | Entry::WildcardEntry(comp) => {
                for i in comp {
                    match i.read_class(class_name) {
                        Ok(data) => return Ok(data),
                        _ => (),
                    }
                }
                Err(format!("class {} not found", class_name))
            }
            _ => todo!(),
        }
    }
}
