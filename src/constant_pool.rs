use std::fmt;
use std::result::Result;
use std::string::String;

use crate::errorcodes::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReferenceKind {
    RefGetField,
    RefGetStatic,
    RefPutField,
    RefPutStatic,
    RefInvokeVirtual,
    RefInvokeStatic,
    RefInvokeSpecial,
    RefNewInvokeSpecial,
    RefInvokeInterface,
}

impl fmt::Display for ReferenceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReferenceKind::RefGetField => writeln!(f, "ReferenceKind::RefGetField"),
            ReferenceKind::RefGetStatic => writeln!(f, "ReferenceKind::RefGetStatic"),
            ReferenceKind::RefPutField => writeln!(f, "ReferenceKind::RefPutField"),
            ReferenceKind::RefPutStatic => writeln!(f, "ReferenceKind::RefPutStatic"),
            ReferenceKind::RefInvokeVirtual => writeln!(f, "ReferenceKind::RefInvokeVirtual"),
            ReferenceKind::RefInvokeStatic => writeln!(f, "ReferenceKind::RefInvokeStatic"),
            ReferenceKind::RefInvokeSpecial => writeln!(f, "ReferenceKind::RefInvokeSpecial"),
            ReferenceKind::RefNewInvokeSpecial => writeln!(f, "ReferenceKind::RefNewInvokeSpecial"),
            ReferenceKind::RefInvokeInterface => writeln!(f, "ReferenceKind::RefInvokeInterface"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DynamicInfo {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MethodHandleInfo {
    pub ref_kind: ReferenceKind,
    pub ref_index: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NameAndTypeInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RefInfo {
    pub class_index: u16,
    pub name_and_type_index: u16,
}


#[derive(Clone, Debug, PartialEq)]
pub enum Entry {
    Utf8(String),
    Integer(i32), 
    Float(f32), 
    Long(i64), 
    Double(f64), 
    Class(u16), 
    String(u16),
    FieldRef(RefInfo), 
    MethodRef(RefInfo), 
    InterfaceMethodRef(RefInfo), 
    NameAndType(NameAndTypeInfo), 
    MethodHandle(MethodHandleInfo), 
    MethodType(u16), 
    Dynamic(DynamicInfo), 
    InvokeDynamic(DynamicInfo), 
    Module(u16), 
    Package(u16),
    Unusable, // This is a custom kind introduced to account for the fact that long and double are mandated to occupy 2 entries, even though they only use one.
    // It's a very weird decision, but this implementation follows it for now.
}

// This code uses runtime assertions (with Option) to check the validity of reads, but I can't think of any way to do it at compile time.

// TODO: This code should panic immediatly if the cast is invalid.
impl Entry {
    pub fn as_utf8(&self) -> Result<&String, Error> {
        match self {
            Entry::Utf8(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToUtf8),
        }
    }
    pub fn as_integer(&self) ->  Result<&i32, Error> {
        match self {
            Entry::Integer(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToInteger),
        }
    }
    pub fn as_float(&self) ->  Result<&f32, Error> {
        match self {
            Entry::Float(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToFloat),
        }
    }
    pub fn as_long(&self) ->  Result<&i64, Error> {
        match self {
            Entry::Long(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToLong),
        }
    }
    pub fn as_double(&self) -> Result<&f64, Error> {
        match self {
            Entry::Double(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToDouble),
        }
    }
    pub fn as_class(&self) -> Result<&u16, Error> {
        match self {
            Entry::Class(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToClass),
        }
    }
    pub fn as_string(&self) -> Result<&u16, Error> {
        match self {
            Entry::String(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToString),
        }
    }
    pub fn as_field_ref(&self) -> Result<&RefInfo, Error> {
        match self {
            Entry::FieldRef(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToFieldRef),
        }
    }
    pub fn as_method_ref(&self) -> Result<&RefInfo, Error> {
        match self {
            Entry::MethodRef(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToMethodRef),
        }
    }
    pub fn as_interface_method_ref(&self) -> Result<&RefInfo, Error> {
        match self {
            Entry::InterfaceMethodRef(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToInterfaceMethodRef),
        }
    }
    pub fn as_name_and_type(&self) -> Result<&NameAndTypeInfo, Error> {
        match self {
            Entry::NameAndType(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToNameAndType),
        }
    }
    pub fn as_method_handle(&self) -> Result<&MethodHandleInfo, Error> {
        match self {
            Entry::MethodHandle(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToMethodHandle),
        }
    }
    pub fn as_method_type(&self) -> Result<&u16, Error> {
        match self {
            Entry::MethodType(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToMethodType),
        }
    }
    pub fn as_dynamic(&self) -> Result<&DynamicInfo, Error> {
        match self {
            Entry::Dynamic(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToDynamic),
        }
    }
    pub fn as_invoke_dynamic(&self) -> Result<&DynamicInfo, Error> {
        match self {
            Entry::InvokeDynamic(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToInvokeDynamic),
        }
    }
    pub fn as_module(&self) -> Result<&u16, Error> {
        match self {
            Entry::Module(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToModule),
        }
    }
    pub fn as_package(&self) -> Result<&u16, Error> {
        match self {
            Entry::Package(val) => Ok(val),
            _ => Err(Error::IllegalEntryCastToPackage),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entry::Utf8(s) => write!(f, "Entry::Utf8 {{
    length: {},
    bytes: {},\n}}", s.len(), s),
            Entry::Integer(i) => write!(f, "Entry::Integer {{
    bytes: {},\n}}", i),
            Entry::Float(float) => write!(f, "Entry::Float {{
    bytes: {},\n}}", float),
            Entry::Long(l) => write!(f, "Entry::Long {{
    bytes: {},\n}}", l),
            Entry::Double(d) => write!(f, "Entry::Double {{
    bytes: {},\n}}", d),
            Entry::Class(c) => write!(f, "Entry::Class {{
    name_index: {},\n}}", c),
            Entry::String(index) => write!(f, "Entry::String {{
    string_index: {},\n}}", index),
            Entry::FieldRef(info) => write!(f, "Entry::FieldRef {{
    class_index: {},
    name_and_type_index: {},\n}}", info.class_index, info.name_and_type_index),
            Entry::MethodRef(info) => write!(f, "Entry::MethodRef {{
    class_index: {},
    name_and_type_index: {},\n}}", info.class_index, info.name_and_type_index),
            Entry::InterfaceMethodRef(info) => write!(f, "Entry::InterfaceMethodRef {{
    class_index: {},
    name_and_type_index: {},\n}}", info.class_index, info.name_and_type_index),
            Entry::NameAndType(nandt) => write!(f, "Entry::NameAndType {{
    name_index: {},
    descriptor_index: {},\n}}", nandt.name_index, nandt.descriptor_index),
            Entry::MethodHandle(handle) => write!(f, "Entry::MethodHandle {{
    reference_kind: {},
    reference_index: {},\n}}", handle.ref_kind, handle.ref_index),
            Entry::MethodType(t) => write!(f, "Entry::MethodType {{
    descriptor_index: {},\n}}", t),
            Entry::Dynamic(d) => write!(f, "Entry::Dynamic {{
    bootstrap_method_attr_index: {},
    name_and_type_index: {},\n}}", d.bootstrap_method_attr_index, d.name_and_type_index),
            Entry::InvokeDynamic(d) => write!(f, "Entry::InvokeDynamic {{
    bootstrap_method_attr_index: {},
    name_and_type_index: {},\n}}", d.bootstrap_method_attr_index, d.name_and_type_index),
            Entry::Module(m) => write!(f, "Entry::Module {{
    name_index: {},\n}}", m),
            Entry::Package(p) => write!(f, "Entry::Package {{
    name_index: {},\n}}", p),
            Entry::Unusable => write!(f, "Entry::Unusable\n"),
        }
    }
}