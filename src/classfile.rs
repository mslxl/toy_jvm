use std::cell::{RefCell, RefMut};
use std::fs::read;

pub struct ClassReader {
    data: Vec<u8>,
    pos: usize,
}

impl ClassReader {
    pub fn from(data: Vec<u8>) -> ClassReader {
        ClassReader {
            data,
            pos: 0,
        }
    }
    pub fn read_u8(&mut self) -> u8 {
        let v = self.data[self.pos];
        self.pos += 1;
        v
    }
    pub fn read_u16(&mut self) -> u16 {
        let v = u16::from_be_bytes(self.data[self.pos..self.pos + 2].try_into().unwrap());
        self.pos += 2;
        v
    }

    pub fn read_u32(&mut self) -> u32 {
        let v = u32::from_be_bytes(self.data[self.pos..self.pos + 4].try_into().unwrap());
        self.pos += 4;
        v
    }

    pub fn read_u64(&mut self) -> u64 {
        let v = u64::from_be_bytes(self.data[self.pos..self.pos + 8].try_into().unwrap());
        self.pos += 8;
        v
    }
    pub fn read_u16s(&mut self) -> Vec<u16> {
        let size = self.read_u16();
        let mut v: Vec<u16> = Vec::with_capacity(size as usize);
        for i in 0..size {
            v[i as usize] = self.read_u16();
        }
        v
    }

    pub fn read_bytes(&mut self, size: usize) -> &[u8] {
        let v = &self.data[self.pos..self.pos + size];
        self.pos += size;
        v
    }
}

#[derive(Debug)]
pub struct ConstantPool(Vec<Option<ConstantInfo>>);

impl ConstantPool {
    fn get_constant_info(&self, index: u16) -> &ConstantInfo {
        let info = self.0.get(index as usize).unwrap();
        match info {
            Some(i) => i,
            None => panic!("Invalid constant pool index!")
        }
    }

    fn get_utf8(&self, index: u16) -> &str {
        if let ConstantInfo::Utf8(text) = self.get_constant_info(index) {
            text
        } else {
            panic!("constant info at {} is not a UTF8", index)
        }
    }

    fn get_class_name(&self, index: u16) -> &str {
        if let ConstantInfo::Class { name_index } = self.get_constant_info(index) {
            self.get_utf8(*name_index)
        } else {
            panic!("constant info at {} is not a Class", index)
        }
    }


    fn get_name_and_type(&self, index: u16) -> (&str, &str) {
        if let ConstantInfo::NameAndType { name_index, descriptor_index } = self.get_constant_info(index) {
            (self.get_utf8(*name_index), self.get_utf8(*descriptor_index))
        } else {
            panic!("constant info at {} is not a NameAndType", index)
        }
    }
}


#[derive(Debug)]
pub enum ConstantInfo {
    Class {
        name_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    String(u16),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8(String),
    MethodHandle,
    MethodType,
    InvokeDynamic,
}

#[derive(Debug)]
pub struct MemberInfo {
    constant_pool: ConstantPool,
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct AttributeInfo {}


#[derive(Debug)]
pub struct ClassFile {
    // magic:u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<MemberInfo>,
    methods: Vec<MemberInfo>,
    attributes: Vec<AttributeInfo>,
}

struct ClassReaderHelper(RefCell<ClassReader>);

impl ClassReaderHelper {
    fn new(mut reader: ClassReader) -> Self {
        ClassReaderHelper(RefCell::new(reader))
    }

    fn raw(&self) -> RefMut<ClassReader> {
        self.0.borrow_mut()
    }

    fn read_and_check_magic(&self) {
        let magic = self.raw().read_u32();
        if magic != 0xCAFEBABE {
            panic!("java.lang.ClassFormatError: magic!") // :)
        }
    }

    fn read_and_check_version(&self) -> (u16, u16) {
        let mut reader = self.raw();
        let minor_version = reader.read_u16();
        let major_version = reader.read_u16();
        match major_version {
            45 => (),
            46..=52 => {
                if minor_version == 0 {
                    ()
                } else {
                    panic!("java.lang.UnsupportedClassVersionError!")
                }
            }
            _ => panic!("java.lang.UnsupportedClassVersionError!")
        };
        (minor_version, major_version)
    }
    fn read_constant_info(&self) -> ConstantInfo {
        let tag = self.raw().read_u8();
        match tag {
            7 => {
                // Class
                let name_index = self.raw().read_u16();
                ConstantInfo::Class {
                    name_index
                }
            }
            9 | 10 | 11 => {
                // 9:  FieldRef
                // 10: MethodRef
                // 11: InterfaceMethodRef
                let mut reader = self.raw();
                let class_index = reader.read_u16();
                let name_and_type_index = reader.read_u16();
                match tag {
                    9 => ConstantInfo::FieldRef { class_index, name_and_type_index },
                    10 => ConstantInfo::MethodRef { class_index, name_and_type_index },
                    11 => ConstantInfo::InterfaceMethodRef { class_index, name_and_type_index },
                    _ => panic!()
                }
            }
            8 => {
                // String
                ConstantInfo::String(self.raw().read_u16())
            }
            3 => {
                // Integer
                let value: i32 = self.raw().read_u32() as i32;
                ConstantInfo::Integer(value)
            }
            4 => {
                // Float
                let value: u32 = self.raw().read_u32();
                let value = f32::from_bits(value);
                ConstantInfo::Float(value)
            }
            5 => {
                // Long
                let value = self.raw().read_u64() as i64;
                ConstantInfo::Long(value)
            }
            6 => {
                // Double
                let value = self.raw().read_u64();
                let value = f64::from_bits(value);
                ConstantInfo::Double(value)
            }
            12 => {
                // NameAndType
                let mut reader = self.raw();
                let name_index = reader.read_u16();
                let descriptor_index = reader.read_u16();
                ConstantInfo::NameAndType {
                    name_index,
                    descriptor_index,
                }
            }
            1 => {
                // Utf8
                let mut reader = self.raw();
                let len = reader.read_u16();
                let bytes = reader.read_bytes(len as usize);
                let s = String::from_utf8(mutf8::mutf8_to_utf8(bytes).unwrap().to_vec()).unwrap();
                ConstantInfo::Utf8(s)
            }
            15 => {
                //Method Handle
                todo!()
            }
            16 => {
                // MethodType
                todo!()
            }
            18 => {
                // InvokeDynamic
                todo!()
            }
            _ => panic!("")
        }
    }

    fn read_constant_pool(&self) -> ConstantPool {
        let cnt = self.raw().read_u16() as usize;
        let mut item: Vec<Option<ConstantInfo>> = Vec::with_capacity(cnt);
        for _ in 0..cnt {
            item.push(None);
        }
        let mut idx = 1;
        while idx < cnt {
            item[idx] = Some(self.read_constant_info());
            match item[idx].as_ref().unwrap() {
                ConstantInfo::Long(_) | ConstantInfo::Double(_) => {
                    idx += 2;
                }
                _ => {
                    idx += 1;
                }
            }
        }
        ConstantPool(item)
    }
}


impl ClassFile {
    fn read_member(&mut self, reader: &mut ClassReader) {
        todo!()
    }


    pub fn from(data: Vec<u8>) -> Self {
        let reader = ClassReaderHelper::new(ClassReader::from(data));
        reader.read_and_check_magic();
        let (minor_version, major_version) = reader.read_and_check_version();
        let constant_pool = reader.read_constant_pool();
        let access_flags = reader.raw().read_u16();
        let this_class = reader.raw().read_u16();
        let super_class = reader.raw().read_u16();
        ClassFile {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
        }
    }
}