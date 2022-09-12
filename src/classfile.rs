use std::borrow::Cow;
use std::cell::{RefCell, RefMut};
use std::collections::HashSet;
use std::fs::read;
use std::rc::Rc;

pub struct ClassReader {
    data: Rc<Vec<u8>>,
    pos: usize,
}

impl ClassReader {
    pub fn from(data: Rc<Vec<u8>>) -> ClassReader {
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
        for _ in 0..size {
            v.push(self.read_u16());
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
    pub fn get_constant_info(&self, index: u16) -> Option<&ConstantInfo> {
        self.0.get(index as usize).unwrap().as_ref()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_utf8(&self, index: u16) -> &str {
        if let ConstantInfo::Utf8(text) = self.get_constant_info(index).expect("Invalid constant idx when get utf8") {
            text
        } else {
            panic!("constant info at {} is not a UTF8", index)
        }
    }

    pub fn get_class_name(&self, index: u16) -> Option<&str> {
        if index == 0 {
            None
        } else if let ConstantInfo::Class { name_index } = self.get_constant_info(index).expect("Invalid constant idx when get classname") {
            Some(self.get_utf8(*name_index))
        } else {
            panic!("constant info at {} is not a Class", index)
        }
    }


    pub fn get_name_and_type(&self, index: u16) -> (&str, &str) {
        if let ConstantInfo::NameAndType { name_index, descriptor_index } = self.get_constant_info(index).expect("Invalid constant idx when get name and type") {
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
    pub access_flags: u16,
    pub name_index: u16,
    pub name: String,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug)]
pub enum AttributeInfo {
    Deprecated,
    Synthetic,
    SourceFile {
        sourcefile_index: u16,
        sourcefile: String,
    },
    ConstantValue {
        constantvalue_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTable>,
        attributes: Vec<AttributeInfo>,
    },
    Exceptions,
    LineNumberTable(Vec<LineNumberTableEntry>),
    LocalVariableTable,
    UnparsedAttribute {
        attribute_name_index: u16,
        attribute_name: String,
        attribute_length: u32,
        info: Vec<u8>,
    },
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}


#[derive(Debug)]
pub struct ClassFile {
    // magic:u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<MemberInfo>,
    pub methods: Vec<MemberInfo>,
    pub attributes: Vec<AttributeInfo>,
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

    fn read_interfaces(&self) -> Vec<u16> {
        self.raw().read_u16s()
    }

    fn read_member_info(&self, constant_pool: &ConstantPool) -> MemberInfo {
        let access_flags = self.raw().read_u16();
        let name_index = self.raw().read_u16();
        let descriptor_index = self.raw().read_u16();
        let attributes = self.read_attributes(constant_pool);
        let name = constant_pool.get_utf8(name_index).to_string();
        MemberInfo {
            access_flags,
            name_index,
            name,
            descriptor_index,
            attributes,
        }
    }

    fn read_members(&self, constant_pool: &ConstantPool) -> Vec<MemberInfo> {
        let cnt = self.raw().read_u16();
        let mut items: Vec<MemberInfo> = Vec::new();
        for _ in 0..cnt {
            items.push(self.read_member_info(constant_pool));
        }
        items
    }

    fn read_attribute_info(&self, constant_pool: &ConstantPool) -> AttributeInfo {
        let attribute_name_index = self.raw().read_u16();
        let attribute_name = constant_pool.get_utf8(attribute_name_index);
        let attribute_length = self.raw().read_u32();
        match attribute_name {
            "Deprecated" => AttributeInfo::Deprecated,
            "Synthetic" => AttributeInfo::Synthetic,
            "SourceFile" => {
                assert_eq!(attribute_length, 2);
                let sourcefile_index = self.raw().read_u16();
                let sourcefile = constant_pool.get_utf8(sourcefile_index).to_string();
                AttributeInfo::SourceFile { sourcefile_index, sourcefile }
            }
            "ConstantValue" => {
                assert_eq!(attribute_length, 2);
                let constantvalue_index = self.raw().read_u16();
                AttributeInfo::ConstantValue { constantvalue_index }
            }
            "Code" => {
                let max_stack = self.raw().read_u16();
                let max_locals = self.raw().read_u16();
                let code_length = self.raw().read_u32();
                let mut code: Vec<u8> = Vec::new();
                for _ in 0..code_length {
                    code.push(self.raw().read_u8());
                }
                let exception_table = self.read_exception_table_attr(constant_pool);
                let attributes = self.read_attributes(constant_pool);
                AttributeInfo::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                }
            }
            "LineNumberTable" => {
                AttributeInfo::LineNumberTable(self.read_line_number_table_attr())
            }
            _ => {
                let mut info: Vec<u8> = Vec::new();
                for _ in 0..attribute_length {
                    info.push(self.raw().read_u8())
                }
                AttributeInfo::UnparsedAttribute { attribute_name_index, attribute_name: attribute_name.to_string(), attribute_length, info }
            }
        }
    }

    fn read_line_number_table_attr(&self) -> Vec<LineNumberTableEntry> {
        let len = self.raw().read_u16();
        let mut entries = Vec::new();
        for _ in 0..len {
            entries.push(self.read_line_number_table_entry());
        }
        entries
    }

    fn read_line_number_table_entry(&self) -> LineNumberTableEntry {
        let start_pc = self.raw().read_u16();
        let line_number = self.raw().read_u16();
        LineNumberTableEntry {
            start_pc,
            line_number,
        }
    }

    fn read_exception_table_attr(&self, constant_pool: &ConstantPool) -> Vec<ExceptionTable> {
        let exception_table_length = self.raw().read_u16();
        let mut entries = Vec::new();
        for _ in 0..exception_table_length {
            entries.push(self.read_exception_table_entry(constant_pool));
        }
        entries
    }

    fn read_exception_table_entry(&self, constant_pool: &ConstantPool) -> ExceptionTable {
        let start_pc = self.raw().read_u16();
        let end_pc = self.raw().read_u16();
        let handler_pc = self.raw().read_u16();
        let catch_type = self.raw().read_u16();
        ExceptionTable {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }

    fn read_attributes(&self, constant_pool: &ConstantPool) -> Vec<AttributeInfo> {
        let cnt = self.raw().read_u16();
        let mut attributes = Vec::new();
        for _ in 0..cnt {
            attributes.push(self.read_attribute_info(constant_pool));
        }
        attributes
    }
}


impl ClassFile {
    pub fn from(data: Rc<Vec<u8>>) -> Self {
        let reader = ClassReaderHelper::new(ClassReader::from(data));
        reader.read_and_check_magic();
        let (minor_version, major_version) = reader.read_and_check_version();
        let constant_pool = reader.read_constant_pool();
        let access_flags = reader.raw().read_u16();
        let this_class = reader.raw().read_u16();
        let super_class = reader.raw().read_u16();
        let interfaces = reader.read_interfaces();
        let fields = reader.read_members(&constant_pool);
        let methods = reader.read_members(&constant_pool);
        let attributes = reader.read_attributes(&constant_pool);
        ClassFile {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }
}