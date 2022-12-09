use std::collections::HashMap;
use std::vec::Vec;
use std::option::Option;
use std::rc::Rc;
use std::result::Result;
use std::string::String;

use crate::attributes::annotations::{Annotation, TypeAnnotation, ElementValue};
use crate::attributes::code::stack_map_table::{StackMapFrame, VerificationTypeInfo};
use crate::attributes::code::{Code, Exception, LineNumber, LocalVariable, LocalVariableType};
use crate::attributes::module::{Module, Require, Export, Open, Provide};
use crate::attributes::{InnerClass, EnclosingMethod, BootstrapMethod, RecordComponentInfo, MethodParameter};
use crate::constant_pool::{Entry, NameAndTypeInfo, RefInfo, MethodHandleInfo, ReferenceKind, DynamicInfo};
use crate::data_access::*;
use crate::errorcodes::{Error, Opcode};
use crate::flags;
use crate::jvm::JVM;
use crate::jvm::instructions::{Instruction, self};


#[derive(Clone, Debug, PartialEq)]
pub struct FieldInfo {
    pub access_flags: flags::field::AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    // Attributes:
    pub constant_value: Option<u16>,
    pub synthetic: bool,
    pub deprecated: bool,
    pub signature: Option<u16>,
    pub rt_vis_annotations: Option<Vec<Annotation>>,
    pub rt_invis_annotations: Option<Vec<Annotation>>,
    pub rt_vis_type_annotations: Option<Vec<TypeAnnotation>>,
    pub rt_invis_type_annotations: Option<Vec<TypeAnnotation>>,
    
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodInfo {
    pub access_flags: flags::method::AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    // Attributes:
    pub code: Option<Code>,
    pub exceptions: Option<Vec<u16>>,
    pub rt_vis_param_annotations: Option<Vec<Vec<Annotation>>>,
    pub rt_invis_param_annotations: Option<Vec<Vec<Annotation>>>,
    pub annotation_default: Option<ElementValue>,
    pub method_parameters: Option<Vec<MethodParameter>>,
    pub synthetic: bool,
    pub deprecated: bool,
    pub signature: Option<u16>,
    pub rt_vis_annotations: Option<Vec<Annotation>>,
    pub rt_invis_annotations: Option<Vec<Annotation>>,
    pub rt_vis_type_annotations: Option<Vec<TypeAnnotation>>,
    pub rt_invis_type_annotations: Option<Vec<TypeAnnotation>>,
}

impl MethodInfo {
    pub fn code(& self) -> Result<&Vec<Box<dyn Instruction>>, Error> {
        if let Some(code) = &self.code {
            return Ok(&code.code);
        }
        if self.access_flags.flags & flags::method::ACC_ABSTRACT > 0 {
            return Err(Error::AbstractMethodCodeAccess);
        }
        Err(Error::NativeMethodCodeAccess)
    }
    pub fn code_at(&self, index: usize) -> Result<&Box<dyn Instruction>, Error> {
        if let Some(code) = &self.code {
            return Ok(&code.code[index]);
        }
        if self.access_flags.flags & flags::method::ACC_ABSTRACT > 0 {
            return Err(Error::AbstractMethodCodeAccess);
        }
        Err(Error::NativeMethodCodeAccess)
    }
    pub fn code_at_mut(&mut self, index: usize) -> Result<&mut Box<dyn Instruction>, Error> {
        if let Some(code) = &mut self.code {
            return Ok(&mut code.code[index]);
        }
        if self.access_flags.flags & flags::method::ACC_ABSTRACT > 0 {
            return Err(Error::AbstractMethodCodeAccess);
        }
        Err(Error::NativeMethodCodeAccess)
    }

    pub fn returns_int(&self, current_class: &Rc<ClassFile>) -> Result<bool, Error> {
        let descriptor = current_class.cp_entry(self.descriptor_index)?.as_utf8()?;
        let return_start = match descriptor.find(')') {
            Some(loc) => loc + 1,
            None => return Err(Error::IncompleteMethodDescriptor(Opcode::IRETURN)),
        };
        Ok(matches!(descriptor.as_bytes()[return_start], b'B' | b'C' | b'I' | b'S' | b'Z')) 
    }
    pub fn return_char(&self, current_class: &Rc<ClassFile>) -> Result<char, Error>{
        let descriptor = current_class.cp_entry(self.descriptor_index)?.as_utf8()?;
        let return_start = match descriptor.find(')') {
            Some(loc) => loc + 1,
            None => return Err(Error::IncompleteMethodDescriptor(Opcode::IRETURN)),
        };
        Ok(descriptor.as_bytes()[return_start] as char) 
    }
    pub fn returns_long(&self, current_class: &Rc<ClassFile>) -> Result<bool, Error> {
        let descriptor = current_class.cp_entry(self.descriptor_index)?.as_utf8()?;
        let return_start = match descriptor.find(')') {
            Some(loc) => loc + 1,
            None => return Err(Error::IncompleteMethodDescriptor(Opcode::LRETURN)),
        };
        Ok(descriptor.as_bytes()[return_start] == b'J') 
    }
    pub fn returns_float(&self, current_class: &Rc<ClassFile>) -> Result<bool, Error> {
        let descriptor = current_class.cp_entry(self.descriptor_index)?.as_utf8()?;
        let return_start = match descriptor.find(')') {
            Some(loc) => loc + 1,
            None => return Err(Error::IncompleteMethodDescriptor(Opcode::FRETURN)),
        };
        Ok(descriptor.as_bytes()[return_start] == b'F') 
    }
    pub fn returns_double(&self, current_class: &Rc<ClassFile>) -> Result<bool, Error> {
        let descriptor = current_class.cp_entry(self.descriptor_index)?.as_utf8()?;
        let return_start = match descriptor.find(')') {
            Some(loc) => loc + 1,
            None => return Err(Error::IncompleteMethodDescriptor(Opcode::DRETURN)),
        };
        Ok(descriptor.as_bytes()[return_start] == b'D') 
    }
    pub fn returns_reference(&self, current_class: &Rc<ClassFile>) -> Result<bool, Error> {
        let descriptor = current_class.cp_entry(self.descriptor_index)?.as_utf8()?;
        let return_start = match descriptor.find(')') {
            Some(loc) => loc + 1,
            None => return Err(Error::IncompleteMethodDescriptor(Opcode::ARETURN)),
        };
        Ok((descriptor.as_bytes()[return_start] == b'L') | (descriptor.as_bytes()[return_start] == b'[')) 
    }
    pub fn returns_void(&self, current_class: &Rc<ClassFile>) -> Result<bool, Error> {
        let descriptor = current_class.cp_entry(self.descriptor_index)?.as_utf8()?;
        let return_start = match descriptor.find(')') {
            Some(loc) => loc + 1,
            None => return Err(Error::IncompleteMethodDescriptor(Opcode::RETURN)),
        };
        Ok(descriptor.as_bytes()[return_start] == b'V') 
    }
    pub fn return_descriptor<'a>(&self, current_class: &'a Rc< ClassFile>) -> Result<&'a str, Error> {
        let descriptor = (*current_class).cp_entry(self.descriptor_index)?.as_utf8()?;
        let return_start = match descriptor.find(')') {
            Some(loc) => loc + 1,
            None => return Err(Error::IncompleteMethodDescriptor(Opcode::ARETURN)),
        };
        Ok(&descriptor[return_start..])
    }
    pub fn num_args(&self, current_class: &Rc<ClassFile>) -> Result<usize, Error> {
        let desc = current_class.cp_entry(self.descriptor_index)?.as_utf8()?.as_str();
        // This is slow but will work for now
        Ok(JVM::parse_descriptor(desc)?.0.len())
    }

}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Entry>, 
    pub access_flags: flags::class::AccessFlags,
    pub this_class_index: u16,
    pub super_class_index: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,  
    // Attributes:
    pub source_file: Option<u16>,
    pub inner_classes: Option<Vec<InnerClass>>,
    pub enclosing_method: Option<EnclosingMethod>,
    pub source_debug_extension: Option<Vec<u8>>,
    pub bootstrap_methods: Option<Vec<BootstrapMethod>>,
    pub module: Option<Module>,
    pub module_packages: Option<Vec<u16>>,
    pub module_main_class: Option<u16>,
    // Important: There cannot be both a NestHost attribute and a NestMembers attribute in the same class.
    // We could implement this as an enum variant, but I think this will work for now.
    pub nest_host: Option<u16>,
    pub nest_members: Option<Vec<u16>>,
    pub record: Option<Vec<RecordComponentInfo>>,
    // There cannot be a PermittedSubclasses attribute in an ACC_FINAL class.
    pub permitted_subclasses: Option<Vec<u16>>,
    pub synthetic: bool, // It's really only a flag.
    pub deprecated: bool, // Same here.
    pub signature: Option<u16>,
    pub rt_vis_annotations: Option<Vec<Annotation>>,
    pub rt_invis_annotations: Option<Vec<Annotation>>,
    pub rt_vis_type_annotations: Option<Vec<TypeAnnotation>>,
    pub rt_invis_type_annotations: Option<Vec<TypeAnnotation>>,
}

impl ClassFile {
    pub fn access_flags(&self) -> flags::class::AccessFlags {
        self.access_flags
    }
    pub fn fields(&self) -> &Vec<FieldInfo> {
        &self.fields
    }
    pub fn methods(&self) -> &Vec<MethodInfo> {
        &self.methods
    }
    pub fn cp_entry(&self, index: u16) -> Result<&Entry, Error> {
        if index == 0 || index > self.constant_pool.len() as u16{
            return Err(Error::IllegalConstantPoolIndex);
        }
        Ok(&self.constant_pool[(index - 1) as usize])
    }
    pub fn cp_entries(&self) -> &Vec<Entry> {
        &self.constant_pool
    }
    pub fn cpool_size(&self) -> usize {
        self.constant_pool.len()
    }
    pub fn interfaces(&self) -> &Vec<u16> {
        &self.interfaces
    }
    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }
    pub fn major_version(&self) -> u16 {
        self.major_version
    }
    pub fn this_index(&self) -> u16 {
        self.this_class_index
    }
    pub fn super_index(&self) -> Option<u16> {
        match self.super_class_index {
            0 => None,
            _ => Some(self.super_class_index)
        }
    }
    pub fn name(&self) -> &str {
        // This code doesn't test the value of the option, and just uses unwrap, because cases like constant pool corruption should be tested in verify_state().
        (self.cp_entry(*self.cp_entry(self.this_class_index).unwrap().as_class().unwrap())).unwrap().as_utf8().unwrap()
        
    }
    pub fn super_name(&self) -> Option<&str> {
        // This code doesn't test the value of the option, and just uses unwrap, because cases like constant pool corruption should be tested in verify_state()
        if self.super_class_index != 0 {
            return Some((self.cp_entry(*self.cp_entry(self.super_class_index).unwrap().as_class().unwrap())).unwrap().as_utf8().unwrap());
        }
        None
    }
    #[inline] pub fn has_super(&self) -> bool {
        self.super_class_index != 0
    }
    pub fn is_interface(&self) -> bool {
        (self.access_flags().flags & flags::class::ACC_INTERFACE) > 0
    }
}

macro_rules! illegal_duplicate {
    ($attrib:ident) => {
        if $attrib.is_some() {
            return Err(Error::IllegalDuplicateAttribute);
        }
    };
}

impl ClassFile {
    // TODO: This function should use ptr.add() instead of ptr.offset().
    /// # Safety
    /// 
    /// This function is unsafe because the caller has to guarantee that `data` is aligned as a `&[u64]`
    /// The Caller also has to make sure to call 'init_code(jvm)' at some point before use.
    pub unsafe fn new(data: &[u8]) -> Result<(ClassFile, Vec<Vec<u8>>), Error> {
        // This function uses unsafe raw pointers because it makes the casting easier. 
        // This is something we should do for all functions at some point.
        // It's safe as long as we align the vector because all we're doing is reinterpreting memory, we know the memory is valid and how ownership works with it.
        // IMPORTANT: The data struct should be aligned as a &[u64], and then cast to a &[u8].

        // We define some nice reading functions here just to make code more readable.
        // These functions should check the length of the vec, but for now, we don't.
        let mut location = 0;
        let mut code_bytes= Vec::new();
        let data_ptr = data.as_ptr();
        let magic = read_u32(data_ptr, &mut location);
        if magic != 0xCAFEBABE {
            return Err(Error::IllegalMagicNumber(magic));
        }
        let minor_version = read_u16(data_ptr, &mut location);
        let major_version = read_u16(data_ptr, &mut location);
        // TODO CHECK VERSION  
        let cpool_count = read_u16(data_ptr, &mut location) - 1;
        let mut cpool: Vec<Entry> = Vec::with_capacity(cpool_count as usize);
        while cpool.capacity() > cpool.len() {
            let (entry, is8byte) = match read_u8(data_ptr, &mut location) {
                1 => {
                    let length = read_u16(data_ptr, &mut location);
                    // The Java spec says that this string should be valid utf8, but we check just in case.
                    let string = match String::from_utf8(std::slice::from_raw_parts(data_ptr.offset(location), length as usize).to_vec()) {
                        Ok(s) => s,
                        Err(_) => {
                            return Err(Error::InvalidUtf8);
                        },
                    };
                    location += length as isize;
                    (Entry::Utf8(string), false)
                },
                3 => {
                    let value = read_i32(data_ptr, &mut location);
                    (Entry::Integer(value), false)
                },
                4 => {
                    let value = f32::from_bits(read_u32(data_ptr, &mut location));
                    (Entry::Float(value), false)
                },
                5 => {
                    let value = read_i64(data_ptr, &mut location);
                    (Entry::Long(value), true)
                },
                6 => {
                    let value = f64::from_bits(read_u64(data_ptr, &mut location));
                    (Entry::Double(value), true)
                },
                7 => {
                    let index = read_u16(data_ptr, &mut location);
                    (Entry::Class(index), false)
                },
                8 => {
                    let index = read_u16(data_ptr, &mut location);
                    (Entry::String(index), false)
                },
                9 => {
                    let class_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::FieldRef(RefInfo{class_index, name_and_type_index}), false)
                },
                10 => {
                    let class_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::MethodRef(RefInfo{class_index, name_and_type_index}), false)
                },
                11 => {
                    let class_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::InterfaceMethodRef(RefInfo{class_index, name_and_type_index}), false)
                },
                12 => {
                    let name_index = read_u16(data_ptr, &mut location);
                    let descriptor_index = read_u16(data_ptr, &mut location);
                    (Entry::NameAndType(NameAndTypeInfo{name_index, descriptor_index}), false)
                },
                15 => {
                    let reference_kind = match read_u8(data_ptr, &mut location) {
                        1 => ReferenceKind::RefGetField,
                        2 => ReferenceKind::RefGetStatic,
                        3 => ReferenceKind::RefPutField,
                        4 => ReferenceKind::RefPutStatic,
                        5 => ReferenceKind::RefInvokeVirtual,
                        6 => ReferenceKind::RefInvokeStatic,
                        7 => ReferenceKind::RefInvokeSpecial,
                        8 => ReferenceKind::RefNewInvokeSpecial,
                        9 => ReferenceKind::RefInvokeInterface,
                        _ => return Err(Error::IllegalReferenceKind),
                    };
                    let reference_index = read_u16(data_ptr, &mut location);
                    (Entry::MethodHandle(MethodHandleInfo{ref_kind: reference_kind, ref_index: reference_index}), false)
                },
                16 => {
                    let descriptor_index = read_u16(data_ptr, &mut location);
                    (Entry::MethodType(descriptor_index), false)
                },
                17 => {
                    let bootstrap_method_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::Dynamic(DynamicInfo{bootstrap_method_attr_index: bootstrap_method_index, name_and_type_index}), false)
                },
                18 => {
                    let bootstrap_method_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::InvokeDynamic(DynamicInfo{bootstrap_method_attr_index: bootstrap_method_index, name_and_type_index}), false)
                },
                19 => {
                    let index = read_u16(data_ptr, &mut location);
                    (Entry::Module(index), false)
                },
                20 => {
                    let index = read_u16(data_ptr, &mut location);
                    (Entry::Package(index), false)
                },
                x => {
                    return Err(Error::IllegalConstantPoolTag(x))
                },
            };  
            cpool.push(entry);
            if is8byte {
                cpool.push(Entry::Unusable);
            }
        }
        let access_flags = read_u16(data_ptr, &mut location);
        let this_class = read_u16(data_ptr, &mut location);
        let super_class = read_u16(data_ptr, &mut location);
        let interfaces_count = read_u16(data_ptr, &mut location);
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        while interfaces.capacity() > interfaces.len() {
            interfaces.push(read_u16(data_ptr, &mut location));
        }
        let fields_count = read_u16(data_ptr, &mut location);
        let mut fields = Vec::with_capacity(fields_count as usize);
        while fields.capacity() > fields.len() {
            let field_flags = read_u16(data_ptr, &mut location);
            let field_name_index = read_u16(data_ptr, &mut location);
            let descriptor_index = read_u16(data_ptr, &mut location);
            let mut constant_value = None;
            let mut synthetic = false;
            let mut deprecated = false;
            let mut signature = None;
            let mut rt_vis_annotations = None;
            let mut rt_invis_annotations = None;
            let mut rt_vis_type_annotations = None;
            let mut rt_invis_type_annotations = None;
            let attributes_count = read_u16(data_ptr, &mut location);
            for _ in 0..attributes_count {
                let name_index = read_u16(data_ptr, &mut location);
                if name_index == 0 {
                    return Err(Error::IllegalConstantPoolIndex);
                }
                let name: &str = cpool[name_index as usize - 1].as_utf8()?;
                let length = read_u32(data_ptr, &mut location);
                match name {
                    "ConstantValue" => {
                        illegal_duplicate!(constant_value);
                        let index = read_u16(data_ptr, &mut location);
                        constant_value = Some(index);
                    },
                    "Synthetic" => synthetic = true,
                    "Deprecated" => deprecated = true,
                    "Signature" => {
                        illegal_duplicate!(signature);
                        let index = read_u16(data_ptr, &mut location);
                        signature = Some(index);
                    },
                    "RuntimeVisibleAnnotations" => {
                        illegal_duplicate!(rt_vis_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(Annotation::new(data_ptr, &mut location)?);
                        }
                        rt_vis_annotations = Some(annotations);
                    },
                    "RuntimeInvisibleAnnotations" => {
                        illegal_duplicate!(rt_invis_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(Annotation::new(data_ptr, &mut location)?);
                        }
                        rt_invis_annotations = Some(annotations);
                    },
                    "RuntimeVisibleTypeAnnotations" => {
                        illegal_duplicate!(rt_vis_type_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                        }
                        rt_vis_type_annotations = Some(annotations);
                    },
                    "RuntimeInvisibleTypeAnnotations" => {
                        illegal_duplicate!(rt_invis_type_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                        }
                        rt_invis_type_annotations = Some(annotations);
                    },
                    _ => location += length as isize, // Ignore custom attributes.
                }
            }
            fields.push(FieldInfo {
                access_flags: flags::field::AccessFlags { flags: field_flags },
                name_index: field_name_index,
                descriptor_index,
                constant_value,
                synthetic,
                deprecated,
                signature,
                rt_vis_annotations,
                rt_invis_annotations,
                rt_vis_type_annotations,
                rt_invis_type_annotations,
            })
        }
        let methods_count = read_u16(data_ptr, &mut location);
        let mut methods = Vec::with_capacity(methods_count as usize);
        while methods.capacity() > methods.len() {
            let method_flags = read_u16(data_ptr, &mut location);
            let method_name_index = read_u16(data_ptr, &mut location);
            let descriptor_index = read_u16(data_ptr, &mut location);
            let mut code = None;
            let mut exceptions = None;
            let mut rt_vis_param_annotations = None;
            let mut rt_invis_param_annotations = None;
            let mut annotation_default = None;
            let mut method_parameters = None;
            let mut synthetic = false;
            let mut deprecated = false;
            let mut signature = None;
            let mut rt_vis_annotations = None;
            let mut rt_invis_annotations = None;
            let mut rt_vis_type_annotations = None;
            let mut rt_invis_type_annotations = None;
            let attributes_count = read_u16(data_ptr, &mut location);
            for _ in 0..attributes_count {
                let name_index = read_u16(data_ptr, &mut location);
                if name_index == 0 {
                    return Err(Error::IllegalConstantPoolIndex);
                }
                let name: &str = cpool[name_index as usize - 1].as_utf8()?;
                let length = read_u32(data_ptr, &mut location);
                let starting_location = location;
                match name {
                    "Code" => {
                        illegal_duplicate!(code);
                        let max_stack = read_u16(data_ptr, &mut location);
                        let max_locals = read_u16(data_ptr, &mut location);
                        let code_length = read_u32(data_ptr, &mut location);
                        let code_data = Vec::with_capacity(code_length as usize);
                        // This is inefficient, but it can be improved later
                        code_bytes.push(unsafe {
                            std::slice::from_raw_parts(data_ptr.offset(location), code_length as usize).to_vec()
                        });
                        location += code_length as isize;

                        // TODO: None of this works, it needs to be in init_code().
                        // The instructions have to be initialized after the classfile is created.
                        let exception_table_length = read_u16(data_ptr, &mut location);
                        let mut exception_table = Vec::with_capacity(exception_table_length as usize);
                        while exception_table.capacity() > exception_table.len() {
                            exception_table.push(Exception{
                                start_pc: read_u16(data_ptr, &mut location),
                                end_pc: read_u16(data_ptr, &mut location),
                                handler_pc: read_u16(data_ptr, &mut location),
                                catch_type: read_u16(data_ptr, &mut location),
                            });
                        }
                        let mut line_number_table = Vec::new();
                        let mut local_variable_table = Vec::new();
                        let mut local_variable_type_table = Vec::new();
                        let mut stack_map_table = None;
                        let mut code_rt_vis_type_annotations = None;
                        let mut code_rt_invis_type_annotations = None;
                        let code_attribs_count = read_u16(data_ptr, &mut location);
                        for _ in 0..code_attribs_count {
                            let name_index_code = read_u16(data_ptr, &mut location);
                            if name_index_code == 0 {
                                return Err(Error::IllegalConstantPoolIndex);
                            }
                            let name_code: &str = cpool[name_index_code as usize - 1].as_utf8()?;
                            let length_code = read_u32(data_ptr, &mut location);
                            let starting_location_code = location;
                            match name_code {
                                "LineNumberTable" => {
                                    let table_length = read_u16(data_ptr, &mut location);
                                    let mut table = Vec::with_capacity(table_length as usize);
                                    while table.capacity() > table.len() {
                                        table.push(LineNumber {
                                            start_pc: read_u16(data_ptr, &mut location),
                                            line_number: read_u16(data_ptr, &mut location),
                                        });
                                    }
                                    line_number_table.append(&mut table);
                                },
                                "LocalVariableTable" => {
                                    let table_length = read_u16(data_ptr, &mut location);
                                    let mut table = Vec::with_capacity(table_length as usize);
                                    while table.capacity() > table.len() {
                                        table.push(LocalVariable {
                                            start_pc: read_u16(data_ptr, &mut location),
                                            length: read_u16(data_ptr, &mut location),
                                            name_index: read_u16(data_ptr, &mut location),
                                            descriptor_index: read_u16(data_ptr, &mut location),
                                            index: read_u16(data_ptr, &mut location),
                                        });
                                    }
                                    local_variable_table.push(table);
                                },
                                "LocalVariableTypeTable" => {
                                    let table_length = read_u16(data_ptr, &mut location);
                                    let mut table = Vec::with_capacity(table_length as usize);
                                    while table.capacity() > table.len() {
                                        table.push(LocalVariableType {
                                            start_pc: read_u16(data_ptr, &mut location),
                                            length: read_u16(data_ptr, &mut location),
                                            name_index: read_u16(data_ptr, &mut location),
                                            signature_index: read_u16(data_ptr, &mut location),
                                            index: read_u16(data_ptr, &mut location),
                                        });
                                    }
                                    local_variable_type_table.push(table);
                                },
                                "StackMapTable" => {
                                    illegal_duplicate!(stack_map_table);
                                    let table_length = read_u16(data_ptr, &mut location);
                                    let mut table = Vec::with_capacity(table_length as usize);
                                    while table.capacity() > table.len() {
                                        let frame = {
                                            let tag = read_u8(data_ptr, &mut location);
                                            match tag {
                                                0..=63 => StackMapFrame::SameFrame(tag),
                                                64..=127 => StackMapFrame::SameLocals1StackItem(tag, VerificationTypeInfo::new(data_ptr, &mut location)?),
                                                128..=246 => return Err(Error::IllegalFrameType),
                                                247 => StackMapFrame::SameLocals1StackItemExtended(read_u16(data_ptr, &mut location), VerificationTypeInfo::new(data_ptr, &mut location)?),
                                                248..=250 => StackMapFrame::ChopFrame(tag, read_u16(data_ptr, &mut location)),
                                                251 => StackMapFrame::SameFrameExtended(read_u16(data_ptr, &mut location)),
                                                252..=254 => {
                                                    let offset_delta = read_u16(data_ptr, &mut location);
                                                    let mut locals = Vec::with_capacity(tag as usize - 251);
                                                    while locals.capacity() > locals.len() {
                                                        locals.push(VerificationTypeInfo::new(data_ptr, &mut location)?);
                                                    }
                                                    StackMapFrame::AppendFrame(tag, offset_delta, locals)
                                                }
                                                255 => {
                                                    let offset_delta = read_u16(data_ptr, &mut location);
                                                    let num_locals = read_u16(data_ptr, &mut location);
                                                    let mut locals = Vec::with_capacity(num_locals as usize);
                                                    while locals.capacity() > locals.len() {
                                                        locals.push(VerificationTypeInfo::new(data_ptr, &mut location)?);
                                                    }
                                                    let num_stack = read_u16(data_ptr, &mut location);
                                                    let mut stack = Vec::with_capacity(num_stack as usize);
                                                    while stack.capacity() > stack.len() {
                                                        stack.push(VerificationTypeInfo::new(data_ptr, &mut location)?);
                                                    }
                                                    StackMapFrame::FullFrame(offset_delta, locals, stack)
                                                }
                                            }
                                        };
                                        table.push(frame);
                                    }
                                    stack_map_table = Some(table);
                                },
                                "RuntimeVisibleTypeAnnotations" => {
                                    illegal_duplicate!(code_rt_vis_type_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                                    }
                                    code_rt_vis_type_annotations = Some(annotations);
                                },
                                "RuntimeInvisibleTypeAnnotations" => {

                                    illegal_duplicate!(code_rt_invis_type_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                                    }
                                    code_rt_invis_type_annotations = Some(annotations);
                                },
                                _ => location += length_code as isize, // Ignore custom attributes.
                            }
                            assert!(starting_location_code + length_code as isize == location);
                        }
                        code = Some(Code {
                            max_stack,
                            max_locals,
                            code: code_data,
                            exception_table,
                            line_number_table,
                            local_variable_table,
                            local_variable_type_table,
                            stack_map_table,
                            rt_vis_type_annotations: code_rt_vis_type_annotations,
                            rt_invis_type_annotations: code_rt_invis_type_annotations,
                        });
                    },
                    "Exceptions" => {
                        illegal_duplicate!(exceptions);
                        let num_exceptions = read_u16(data_ptr, &mut location);
                        let mut exception_table = Vec::with_capacity(num_exceptions as usize);
                        while exception_table.capacity() > exception_table.len() {
                            exception_table.push(read_u16(data_ptr, &mut location));
                        }
                        exceptions = Some(exception_table);
                    },
                    "RuntimeVisibleParameterAnnotations" => {
                        illegal_duplicate!(rt_vis_param_annotations);
                        let num_param_annotations = read_u16(data_ptr, &mut location);
                        let mut param_annotations = Vec::with_capacity(num_param_annotations as usize);
                        while param_annotations.capacity() > param_annotations.len() {
                            let num_annotations = read_u16(data_ptr, &mut location);
                            let mut annotations = Vec::with_capacity(num_annotations as usize);
                            while annotations.capacity() > annotations.len() {                            
                                annotations.push(Annotation::new(data_ptr, &mut location)?);
                            }
                            param_annotations.push(annotations);
                        }
                        rt_vis_param_annotations = Some(param_annotations);
                    },
                    "RuntimeInvisibleParameterAnnotations" => {
                        illegal_duplicate!(rt_invis_param_annotations);
                        let num_param_annotations = read_u16(data_ptr, &mut location);
                        let mut param_annotations = Vec::with_capacity(num_param_annotations as usize);
                        while param_annotations.capacity() > param_annotations.len() {
                            let num_annotations = read_u16(data_ptr, &mut location);
                            let mut annotations = Vec::with_capacity(num_annotations as usize);
                            while annotations.capacity() > annotations.len() {                            
                                annotations.push(Annotation::new(data_ptr, &mut location)?);
                            }
                            param_annotations.push(annotations);
                        }
                        rt_invis_param_annotations = Some(param_annotations);
                    },
                    "AnnotationDefault" => {
                        illegal_duplicate!(annotation_default);
                        annotation_default = Some(ElementValue::new(data_ptr, &mut location)?);
                    },
                    "MethodParameters" => {
                        illegal_duplicate!(method_parameters);
                        let num_params = read_u8(data_ptr, &mut location);
                        let mut parameters = Vec::with_capacity(num_params as usize);
                        while parameters.capacity() > parameters.len() {
                            parameters.push(MethodParameter {
                                name_index: read_u16(data_ptr, &mut location),
                                access_flags: read_u16(data_ptr, &mut location),
                            })
                        }
                        method_parameters = Some(parameters);
                    },
                    "Synthetic" => synthetic = true,
                    "Deprecated" => deprecated = true,
                    "Signature" => {
                        illegal_duplicate!(signature);
                        let index = read_u16(data_ptr, &mut location);
                        signature = Some(index);
                    },
                    "RuntimeVisibleAnnotations" => {
                        illegal_duplicate!(rt_vis_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(Annotation::new(data_ptr, &mut location)?);
                        }
                        rt_vis_annotations = Some(annotations);
                    },
                    "RuntimeInvisibleAnnotations" => {
                        illegal_duplicate!(rt_invis_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(Annotation::new(data_ptr, &mut location)?);
                        }
                        rt_invis_annotations = Some(annotations);
                    },
                    "RuntimeVisibleTypeAnnotations" => {
                        illegal_duplicate!(rt_vis_type_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                        }
                        rt_vis_type_annotations = Some(annotations);
                    },
                    "RuntimeInvisibleTypeAnnotations" => {
                        illegal_duplicate!(rt_invis_type_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                        }
                        rt_invis_type_annotations = Some(annotations);
                    },
                    _ => location += length as isize, // Ignore custom attributes.
                }
                assert!(starting_location + length as isize == location);
            }
            let method = MethodInfo {
                access_flags: flags::method::AccessFlags{ flags: method_flags},
                name_index: method_name_index,
                descriptor_index,
                code,
                exceptions,
                rt_vis_param_annotations,
                rt_invis_param_annotations,
                annotation_default,
                method_parameters,
                synthetic,
                deprecated,
                signature,
                rt_vis_annotations,
                rt_invis_annotations,
                rt_vis_type_annotations,
                rt_invis_type_annotations,
            };
            methods.push(method)
        }
        let attributes_count = read_u16(data_ptr, &mut location);
        let mut source_file = None;
        let mut inner_classes = None;
        let mut enclosing_method = None;
        let mut source_debug_extension = None;
        let mut bootstrap_methods = None;
        let mut module = None;
        let mut module_packages = None;
        let mut module_main_class = None;
        let mut nest_host = None;
        let mut nest_members = None;
        let mut record = None;
        let mut permitted_subclasses = None;
        let mut synthetic = false;
        let mut deprecated = false;
        let mut signature = None;
        let mut rt_vis_annotations = None;
        let mut rt_invis_annotations = None;
        let mut rt_vis_type_annotations = None;
        let mut rt_invis_type_annotations = None;
        for _ in 0..attributes_count {
            let name_index = read_u16(data_ptr, &mut location);
            if name_index == 0 {
                return Err(Error::IllegalConstantPoolIndex);
            }
            let name: &str = cpool[name_index as usize - 1].as_utf8()?;
            let length = read_u32(data_ptr, &mut location);
            match name {
                "SourceFile" => {
                    illegal_duplicate!(source_file);
                    source_file = Some(read_u16(data_ptr, &mut location));
                },
                "InnerClasses" => {
                    illegal_duplicate!(inner_classes);
                    let num_classes = read_u16(data_ptr, &mut location);
                    let mut classes = Vec::with_capacity(num_classes as usize);
                    while classes.capacity() > classes.len() {
                        classes.push(InnerClass {
                            inner_class_info_index: read_u16(data_ptr, &mut location),
                            outer_class_info_index: read_u16(data_ptr, &mut location),
                            inner_name_index: read_u16(data_ptr, &mut location),
                            inner_class_access_flags: read_u16(data_ptr, &mut location),
                        });
                    }
                    inner_classes = Some(classes);
                }
                "EnclosingMethod" => {
                    illegal_duplicate!(enclosing_method);
                    enclosing_method = Some(EnclosingMethod {
                        class_index: read_u16(data_ptr, &mut location),
                        method_index: read_u16(data_ptr, &mut location),
                    });
                },
                "SourceDebugExtension" => {
                    illegal_duplicate!(source_debug_extension);
                    let mut extension = Vec::with_capacity(length as usize);
                    while extension.capacity() > extension.len() {
                        extension.push(read_u8(data_ptr, &mut location));
                    }
                    source_debug_extension = Some(extension);
                }
                "BootstrapMethods" => {
                    illegal_duplicate!(bootstrap_methods);
                    let numethods = read_u16(data_ptr, &mut location);
                    let mut methods = Vec::with_capacity(numethods as usize);
                    while methods.capacity() > methods.len() {
                        let method_ref = read_u16(data_ptr, &mut location);
                        let num_args = read_u16(data_ptr, &mut location);
                        let mut args = Vec::with_capacity(num_args as usize);
                        while args.capacity() > args.len() {
                            args.push(read_u16(data_ptr, &mut location));
                        }
                        methods.push(BootstrapMethod {
                            bootstrap_method_ref: method_ref,
                            bootstrap_arguments: args
                        });
                    }
                    bootstrap_methods = Some(methods);
                },
                "Module" => {
                    illegal_duplicate!(module);
                    let name_index = read_u16(data_ptr, &mut location);
                    let flags = read_u16(data_ptr, &mut location);
                    let version_index = read_u16(data_ptr, &mut location);
                    let requires_count = read_u16(data_ptr, &mut location);
                    let mut requires = Vec::with_capacity(requires_count as usize);
                    while requires.capacity() > requires.len() {
                        requires.push(Require {
                            requires_index: read_u16(data_ptr, &mut location),
                            requires_flags: read_u16(data_ptr, &mut location),
                            requires_version_count: read_u16(data_ptr, &mut location),
                        })
                    }
                    let exports_count = read_u16(data_ptr, &mut location);
                    let mut exports = Vec::with_capacity(exports_count as usize);
                    while exports.capacity() > exports.len() {
                        let exports_index = read_u16(data_ptr, &mut location);
                        let exports_flags = read_u16(data_ptr, &mut location);
                        let exports_to_count = read_u16(data_ptr, &mut location);
                        let mut exports_to_index = Vec::with_capacity(exports_to_count as usize);
                        while exports_to_index.capacity() > exports_to_index.len() {
                            exports_to_index.push(read_u16(data_ptr, &mut location));
                        }
                        exports.push(Export {
                            exports_index,
                            exports_flags,
                            exports_to_index,
                        })
                    }
                    let opens_count = read_u16(data_ptr, &mut location);
                    let mut opens = Vec::with_capacity(opens_count as usize);
                    while opens.capacity() > opens.len() {
                        let opens_index = read_u16(data_ptr, &mut location);
                        let opens_flags = read_u16(data_ptr, &mut location);
                        let opens_to_count = read_u16(data_ptr, &mut location);
                        let mut opens_to_index = Vec::with_capacity(opens_to_count as usize);
                        while opens_to_index.capacity() > opens_to_index.len() {
                            opens_to_index.push(read_u16(data_ptr, &mut location));
                        }
                        opens.push(Open {
                            opens_index,
                            opens_flags,
                            opens_to_index,
                        })
                    }
                    let uses_count = read_u16(data_ptr, &mut location);
                    let mut uses = Vec::with_capacity(uses_count as usize);
                    while uses.capacity() > uses.len() {
                        uses.push(read_u16(data_ptr, &mut location));
                    }
                    let provides_count = read_u16(data_ptr, &mut location);
                    let mut provides = Vec::with_capacity(provides_count as usize);
                    while provides.capacity() > provides.len() {
                        let provides_index = read_u16(data_ptr, &mut location);
                        let provides_with_count = read_u16(data_ptr, &mut location);
                        let mut provides_with_index = Vec::with_capacity(provides_with_count as usize);
                        while provides_with_index.capacity() > provides_with_index.len() {
                            provides_with_index.push(read_u16(data_ptr, &mut location));
                        }
                        provides.push(Provide {
                            provides_index,
                            provides_with_index,
                        })
                    }
                    module = Some(Module {
                        module_name_index: name_index,
                        module_flags: flags,
                        module_version_index: version_index,
                        requires,
                        exports,
                        opens,
                        uses,
                        provides,
                    })
                }
                "ModulePackages" => {
                    illegal_duplicate!(module_packages);
                    let package_count = read_u16(data_ptr, &mut location);
                    let mut packages = Vec::with_capacity(package_count as usize);
                    while packages.capacity() > packages.len() {
                        packages.push(read_u16(data_ptr, &mut location));
                    }
                    module_packages = Some(packages);
                }
                "ModuleMainClass" => {
                    illegal_duplicate!(module_main_class);
                    module_main_class = Some(read_u16(data_ptr, &mut location));
                }
                "NestHost" => {
                    illegal_duplicate!(nest_host);
                    nest_host = Some(read_u16(data_ptr, &mut location));
                }
                "NestMembers" => {
                    illegal_duplicate!(nest_members);
                    let num_members = read_u16(data_ptr, &mut location);
                    let mut members = Vec::with_capacity(num_members as usize);
                    while members.capacity() > members.len() {
                        members.push(read_u16(data_ptr, &mut location));
                    }
                    nest_members = Some(members);
                }
                "Record" => {
                    illegal_duplicate!(record);
                    let num_components = read_u16(data_ptr, &mut location);
                    let mut record_components = Vec::with_capacity(num_components as usize);
                    while record_components.capacity() > record_components.len() {
                        let record_name_index = read_u16(data_ptr, &mut location);
                        let descriptor_index = read_u16(data_ptr, &mut location);
                        let nurecord_attributes = read_u16(data_ptr, &mut location);
                        let mut record_signature = None;
                        let mut record_rt_vis_annotations = None;
                        let mut record_rt_invis_annotations = None;
                        let mut record_rt_vis_type_annotations = None;
                        let mut record_rt_invis_type_annotations = None;
                        for _ in 0..nurecord_attributes {
                            let name_index_record = read_u16(data_ptr, &mut location);
                            if name_index_record == 0 {
                                return Err(Error::IllegalConstantPoolIndex);
                            }
                            let name_record: &str = cpool[name_index_record as usize - 1].as_utf8()?;
                            let length_record = read_u32(data_ptr, &mut location);
                            match name_record {
                                "Signature" => {
                                    illegal_duplicate!(record_signature);
                                    let index = read_u16(data_ptr, &mut location);
                                    record_signature = Some(index);
                                },
                                "RuntimeVisibleAnnotations" => {
                                    illegal_duplicate!(record_rt_vis_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(Annotation::new(data_ptr, &mut location)?);
                                    }
                                    record_rt_vis_annotations = Some(annotations);
                                },
                                "RuntimeInvisibleAnnotations" => {
                                    illegal_duplicate!(record_rt_invis_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(Annotation::new(data_ptr, &mut location)?);
                                    }
                                    record_rt_invis_annotations = Some(annotations);
                                },
                                "RuntimeVisibleTypeAnnotations" => {
                                    illegal_duplicate!(record_rt_vis_type_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                                    }
                                    record_rt_vis_type_annotations = Some(annotations);
                                },
                                "RuntimeInvisibleTypeAnnotations" => {
                                    illegal_duplicate!(record_rt_invis_type_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                                    }
                                    record_rt_invis_type_annotations = Some(annotations);
                                },
                                _ => location += length_record as isize,
                            }
                        }
                        record_components.push(RecordComponentInfo { name_index: record_name_index, descriptor_index, signature: record_signature, rt_vis_annotations: record_rt_vis_annotations, 
                            rt_invis_annotations: record_rt_invis_annotations, rt_vis_type_annotations: record_rt_vis_type_annotations, rt_invis_type_annotations: record_rt_invis_type_annotations })
                    }
                    record = Some(record_components);
                }
                "PermittedSubclasses" => {
                    illegal_duplicate!(permitted_subclasses);
                    let num_classes = read_u16(data_ptr, &mut location);
                    let mut classes = Vec::with_capacity(num_classes as usize);
                    while classes.capacity() > classes.len() {
                        classes.push(read_u16(data_ptr, &mut location));
                    }
                    permitted_subclasses = Some(classes);
                }
                "Synthetic" => synthetic = true,
                "Deprecated" => deprecated = true,
                "Signature" => {
                    illegal_duplicate!(signature);
                    let index = read_u16(data_ptr, &mut location);
                    signature = Some(index);
                },
                "RuntimeVisibleAnnotations" => {
                    illegal_duplicate!(rt_vis_annotations);
                    let num_annotations = read_u16(data_ptr, &mut location);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.capacity() > annotations.len() {
                        annotations.push(Annotation::new(data_ptr, &mut location)?);
                    }
                    rt_vis_annotations = Some(annotations);
                },
                "RuntimeInvisibleAnnotations" => {
                    illegal_duplicate!(rt_invis_annotations);
                    let num_annotations = read_u16(data_ptr, &mut location);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.capacity() > annotations.len() {
                        annotations.push(Annotation::new(data_ptr, &mut location)?);
                    }
                    rt_invis_annotations = Some(annotations);
                },
                "RuntimeVisibleTypeAnnotations" => {
                    illegal_duplicate!(rt_vis_type_annotations);
                    let num_annotations = read_u16(data_ptr, &mut location);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.capacity() > annotations.len() {
                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                    }
                    rt_vis_type_annotations = Some(annotations);
                },
                "RuntimeInvisibleTypeAnnotations" => {
                    illegal_duplicate!(rt_invis_type_annotations);
                    let num_annotations = read_u16(data_ptr, &mut location);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.capacity() > annotations.len() {
                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                    }
                    rt_invis_type_annotations = Some(annotations);
                },
                _ => location += length as isize, // Ignore custom attributes.
            }
        }
        Ok( (ClassFile {
            minor_version,
            major_version,
            constant_pool: cpool,
            access_flags: flags::class::AccessFlags{ flags: access_flags },
            this_class_index: this_class,
            super_class_index: super_class,
            interfaces,
            fields,
            methods,
            source_file,
            inner_classes,
            enclosing_method,
            source_debug_extension,
            bootstrap_methods,
            module,
            module_packages,
            module_main_class,
            nest_host,
            nest_members,
            record,
            permitted_subclasses,
            synthetic,
            deprecated,
            signature,
            rt_vis_annotations,
            rt_invis_annotations,
            rt_vis_type_annotations,
            rt_invis_type_annotations,
        }, code_bytes))
    }
    /// This function *can* be called multiple times, but *should* only be called once on a classfile generated from 'new()'
    pub fn init_code(&mut self, op_bytes_vec: Vec<Vec<u8>>, jvm: &mut JVM) -> Result<(), Error> {
        //let is_main_class = self.name() == jvm.m_main_class_name;

        let mut op_bytes_vec = op_bytes_vec.into_iter();
        let self_ptr = self as *const ClassFile;

        for (code, name, desc) in 
            self.methods.iter_mut().filter_map(|m| Some((m.code.as_mut()?, m.name_index, m.descriptor_index))) {
            let mut op_bytes = op_bytes_vec.next().unwrap();
            let mut true_pc = 0;
            let mut was_wide = false;
            let mut addr_hmap = HashMap::with_capacity(op_bytes.len());
            let mut true_pcs = Vec::new();
            let mut base_compress = 0;
            while op_bytes.len() > 0 {
                let old_len = op_bytes.len();
                match instructions::new_instruction(&mut op_bytes, &self.constant_pool, jvm, was_wide, true_pc) {
                    Err(Error::Wide) => was_wide = true,
                    Err(e) => {
                        println!("Currently resolved code in method {}.{}{}: ",
                            unsafe { (*self_ptr).name() }, 
                            unsafe {(*self_ptr).cp_entry(name)?.as_utf8()?},
                            unsafe {(*self_ptr).cp_entry(desc)?.as_utf8()?}
                        );
                        for op in &code.code {
                            println!("{}", op.name());
                        }
                        panic!("Hit error: {:?}", e);
                        return Err(e);
                    },
                    Ok(instruction) => {
                        addr_hmap.insert(true_pc, true_pc - base_compress);
                        true_pcs.push(true_pc);
                        
                        // println!("found instruction {instruction}");

                        code.code.push(instruction);
                        true_pc += old_len - op_bytes.len();
                        base_compress += (old_len - 1) - op_bytes.len();
                        was_wide = false;
                    },
                }
            }
            // println!("Compressing ranges");
            let mut true_pcs = true_pcs.into_iter();
            for instruction in &mut code.code {
                instruction.compress_range(true_pcs.next().unwrap(), &addr_hmap);
            }
        }    
        Ok(())
    }
    /// # Safety
    /// 
    /// This function is unsafe because the caller has to guarantee that `data` is aligned as a `&[u64]`
    pub unsafe fn new_with_jvm(data: &[u8], jvm: &mut JVM) -> Result<ClassFile, Error> {
        // This function uses unsafe raw pointers because it makes the casting easier. 
        // This is something we should do for all functions at some point.
        // It's safe as long as we align the vector because all we're doing is reinterpreting memory, we know the memory is valid and how ownership works with it.
        // IMPORTANT: The data struct should be aligned as a &[u64], and then cast to a &[u8].

        // We define some nice reading functions here just to make code more readable.
        // These functions should check the length of the vec, but for now, we don't.
        let mut location = 0;
        let data_ptr = data.as_ptr();
        let magic = read_u32(data_ptr, &mut location);
        if magic != 0xCAFEBABE {
            return Err(Error::IllegalMagicNumber(magic));
        }
        let minor_version = read_u16(data_ptr, &mut location);
        let major_version = read_u16(data_ptr, &mut location);
        // TODO CHECK VERSION  
        let cpool_count = read_u16(data_ptr, &mut location) - 1;
        let mut cpool: Vec<Entry> = Vec::with_capacity(cpool_count as usize);
        while cpool.capacity() > cpool.len() {
            let (entry, is8byte) = match read_u8(data_ptr, &mut location) {
                1 => {
                    let length = read_u16(data_ptr, &mut location);
                    // The Java spec says that this string should be valid utf8, but we check just in case.
                    let string = match String::from_utf8(std::slice::from_raw_parts(data_ptr.offset(location), length as usize).to_vec()) {
                        Ok(s) => s,
                        Err(_) => {
                            return Err(Error::InvalidUtf8);
                        },
                    };
                    location += length as isize;
                    (Entry::Utf8(string), false)
                },
                3 => {
                    let value = read_i32(data_ptr, &mut location);
                    (Entry::Integer(value), false)
                },
                4 => {
                    let value = f32::from_bits(read_u32(data_ptr, &mut location));
                    (Entry::Float(value), false)
                },
                5 => {
                    let value = read_i64(data_ptr, &mut location);
                    (Entry::Long(value), true)
                },
                6 => {
                    let value = f64::from_bits(read_u64(data_ptr, &mut location));
                    (Entry::Double(value), true)
                },
                7 => {
                    let index = read_u16(data_ptr, &mut location);
                    (Entry::Class(index), false)
                },
                8 => {
                    let index = read_u16(data_ptr, &mut location);
                    (Entry::String(index), false)
                },
                9 => {
                    let class_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::FieldRef(RefInfo{class_index, name_and_type_index}), false)
                },
                10 => {
                    let class_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::MethodRef(RefInfo{class_index, name_and_type_index}), false)
                },
                11 => {
                    let class_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::InterfaceMethodRef(RefInfo{class_index, name_and_type_index}), false)
                },
                12 => {
                    let name_index = read_u16(data_ptr, &mut location);
                    let descriptor_index = read_u16(data_ptr, &mut location);
                    (Entry::NameAndType(NameAndTypeInfo{name_index, descriptor_index}), false)
                },
                15 => {
                    let reference_kind = match read_u8(data_ptr, &mut location) {
                        1 => ReferenceKind::RefGetField,
                        2 => ReferenceKind::RefGetStatic,
                        3 => ReferenceKind::RefPutField,
                        4 => ReferenceKind::RefPutStatic,
                        5 => ReferenceKind::RefInvokeVirtual,
                        6 => ReferenceKind::RefInvokeStatic,
                        7 => ReferenceKind::RefInvokeSpecial,
                        8 => ReferenceKind::RefNewInvokeSpecial,
                        9 => ReferenceKind::RefInvokeInterface,
                        _ => return Err(Error::IllegalReferenceKind),
                    };
                    let reference_index = read_u16(data_ptr, &mut location);
                    (Entry::MethodHandle(MethodHandleInfo{ref_kind: reference_kind, ref_index: reference_index}), false)
                },
                16 => {
                    let descriptor_index = read_u16(data_ptr, &mut location);
                    (Entry::MethodType(descriptor_index), false)
                },
                17 => {
                    let bootstrap_method_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::Dynamic(DynamicInfo{bootstrap_method_attr_index: bootstrap_method_index, name_and_type_index}), false)
                },
                18 => {
                    let bootstrap_method_index = read_u16(data_ptr, &mut location);
                    let name_and_type_index = read_u16(data_ptr, &mut location);
                    (Entry::InvokeDynamic(DynamicInfo{bootstrap_method_attr_index: bootstrap_method_index, name_and_type_index}), false)
                },
                19 => {
                    let index = read_u16(data_ptr, &mut location);
                    (Entry::Module(index), false)
                },
                20 => {
                    let index = read_u16(data_ptr, &mut location);
                    (Entry::Package(index), false)
                },
                x => {
                    return Err(Error::IllegalConstantPoolTag(x))
                },
            };  
            cpool.push(entry);
            if is8byte {
                cpool.push(Entry::Unusable);
            }
        }
        let access_flags = read_u16(data_ptr, &mut location);
        let this_class = read_u16(data_ptr, &mut location);
        let super_class = read_u16(data_ptr, &mut location);
        let interfaces_count = read_u16(data_ptr, &mut location);
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        while interfaces.capacity() > interfaces.len() {
            interfaces.push(read_u16(data_ptr, &mut location));
        }
        let fields_count = read_u16(data_ptr, &mut location);
        let mut fields = Vec::with_capacity(fields_count as usize);
        while fields.capacity() > fields.len() {
            let field_flags = read_u16(data_ptr, &mut location);
            let field_name_index = read_u16(data_ptr, &mut location);
            let descriptor_index = read_u16(data_ptr, &mut location);
            let mut constant_value = None;
            let mut synthetic = false;
            let mut deprecated = false;
            let mut signature = None;
            let mut rt_vis_annotations = None;
            let mut rt_invis_annotations = None;
            let mut rt_vis_type_annotations = None;
            let mut rt_invis_type_annotations = None;
            let attributes_count = read_u16(data_ptr, &mut location);
            for _ in 0..attributes_count {
                let name_index = read_u16(data_ptr, &mut location);
                if name_index == 0 {
                    return Err(Error::IllegalConstantPoolIndex);
                }
                let name: &str = cpool[name_index as usize - 1].as_utf8()?;
                let length = read_u32(data_ptr, &mut location);
                match name {
                    "ConstantValue" => {
                        illegal_duplicate!(constant_value);
                        let index = read_u16(data_ptr, &mut location);
                        constant_value = Some(index);
                    },
                    "Synthetic" => synthetic = true,
                    "Deprecated" => deprecated = true,
                    "Signature" => {
                        illegal_duplicate!(signature);
                        let index = read_u16(data_ptr, &mut location);
                        signature = Some(index);
                    },
                    "RuntimeVisibleAnnotations" => {
                        illegal_duplicate!(rt_vis_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(Annotation::new(data_ptr, &mut location)?);
                        }
                        rt_vis_annotations = Some(annotations);
                    },
                    "RuntimeInvisibleAnnotations" => {
                        illegal_duplicate!(rt_invis_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(Annotation::new(data_ptr, &mut location)?);
                        }
                        rt_invis_annotations = Some(annotations);
                    },
                    "RuntimeVisibleTypeAnnotations" => {
                        illegal_duplicate!(rt_vis_type_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                        }
                        rt_vis_type_annotations = Some(annotations);
                    },
                    "RuntimeInvisibleTypeAnnotations" => {
                        illegal_duplicate!(rt_invis_type_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                        }
                        rt_invis_type_annotations = Some(annotations);
                    },
                    _ => location += length as isize, // Ignore custom attributes.
                }
            }
            fields.push(FieldInfo {
                access_flags: flags::field::AccessFlags { flags: field_flags },
                name_index: field_name_index,
                descriptor_index,
                constant_value,
                synthetic,
                deprecated,
                signature,
                rt_vis_annotations,
                rt_invis_annotations,
                rt_vis_type_annotations,
                rt_invis_type_annotations,
            })
        }
        let methods_count = read_u16(data_ptr, &mut location);
        let mut methods = Vec::with_capacity(methods_count as usize);
        while methods.capacity() > methods.len() {
            let method_flags = read_u16(data_ptr, &mut location);
            let method_name_index = read_u16(data_ptr, &mut location);
            let descriptor_index = read_u16(data_ptr, &mut location);
            let mut code = None;
            let mut exceptions = None;
            let mut rt_vis_param_annotations = None;
            let mut rt_invis_param_annotations = None;
            let mut annotation_default = None;
            let mut method_parameters = None;
            let mut synthetic = false;
            let mut deprecated = false;
            let mut signature = None;
            let mut rt_vis_annotations = None;
            let mut rt_invis_annotations = None;
            let mut rt_vis_type_annotations = None;
            let mut rt_invis_type_annotations = None;
            let attributes_count = read_u16(data_ptr, &mut location);
            for _ in 0..attributes_count {
                let name_index = read_u16(data_ptr, &mut location);
                if name_index == 0 {
                    return Err(Error::IllegalConstantPoolIndex);
                }
                let name: &str = cpool[name_index as usize - 1].as_utf8()?;
                let length = read_u32(data_ptr, &mut location);
                let starting_location = location;
                match name {
                    "Code" => {
                        illegal_duplicate!(code);
                        let max_stack = read_u16(data_ptr, &mut location);
                        let max_locals = read_u16(data_ptr, &mut location);
                        let code_length = read_u32(data_ptr, &mut location);
                        let mut code_data = Vec::with_capacity(code_length as usize);
                        // This is inefficient, but it can be improved later
                        let mut op_bytes = unsafe {
                            std::slice::from_raw_parts(data_ptr.offset(location), code_length as usize).to_vec()
                        };
                        location += code_length as isize;
                        let mut true_pc = 0;
                        let mut was_wide = false;
                        let mut addr_hmap = HashMap::with_capacity(op_bytes.len());
                        let mut base_compress = 0;
                        while op_bytes.len() > 0 {
                            let old_len = op_bytes.len();
                            match instructions::new_instruction(&mut op_bytes, &cpool, jvm, was_wide, true_pc) {
                                Err(Error::Wide) => was_wide = true,
                                Err(e) => return Err(e),
                                Ok(mut instruction) => {
                                    addr_hmap.insert(true_pc, true_pc - base_compress);
                                    instruction.compress_range(true_pc, &addr_hmap);
                                    code_data.push(instruction);
                                    true_pc += old_len - op_bytes.len();
                                    base_compress += (old_len - 1) - op_bytes.len();
                                    was_wide = false;
                                },
                            }
                        }

                        // TODO: Remap other parts of Code, like the exception table. 

                        let exception_table_length = read_u16(data_ptr, &mut location);
                        let mut exception_table = Vec::with_capacity(exception_table_length as usize);
                        while exception_table.capacity() > exception_table.len() {
                            exception_table.push(Exception{
                                start_pc: read_u16(data_ptr, &mut location),
                                end_pc: read_u16(data_ptr, &mut location),
                                handler_pc: read_u16(data_ptr, &mut location),
                                catch_type: read_u16(data_ptr, &mut location),
                            });
                        }
                        let mut line_number_table = Vec::new();
                        let mut local_variable_table = Vec::new();
                        let mut local_variable_type_table = Vec::new();
                        let mut stack_map_table = None;
                        let mut code_rt_vis_type_annotations = None;
                        let mut code_rt_invis_type_annotations = None;
                        let code_attribs_count = read_u16(data_ptr, &mut location);
                        for _ in 0..code_attribs_count {
                            let name_index_code = read_u16(data_ptr, &mut location);
                            if name_index_code == 0 {
                                return Err(Error::IllegalConstantPoolIndex);
                            }
                            let name_code: &str = cpool[name_index_code as usize - 1].as_utf8()?;
                            let length_code = read_u32(data_ptr, &mut location);
                            let starting_location_code = location;
                            match name_code {
                                "LineNumberTable" => {
                                    let table_length = read_u16(data_ptr, &mut location);
                                    let mut table = Vec::with_capacity(table_length as usize);
                                    while table.capacity() > table.len() {
                                        table.push(LineNumber {
                                            start_pc: read_u16(data_ptr, &mut location),
                                            line_number: read_u16(data_ptr, &mut location),
                                        });
                                    }
                                    line_number_table.append(&mut table);
                                },
                                "LocalVariableTable" => {
                                    let table_length = read_u16(data_ptr, &mut location);
                                    let mut table = Vec::with_capacity(table_length as usize);
                                    while table.capacity() > table.len() {
                                        table.push(LocalVariable {
                                            start_pc: read_u16(data_ptr, &mut location),
                                            length: read_u16(data_ptr, &mut location),
                                            name_index: read_u16(data_ptr, &mut location),
                                            descriptor_index: read_u16(data_ptr, &mut location),
                                            index: read_u16(data_ptr, &mut location),
                                        });
                                    }
                                    local_variable_table.push(table);
                                },
                                "LocalVariableTypeTable" => {
                                    let table_length = read_u16(data_ptr, &mut location);
                                    let mut table = Vec::with_capacity(table_length as usize);
                                    while table.capacity() > table.len() {
                                        table.push(LocalVariableType {
                                            start_pc: read_u16(data_ptr, &mut location),
                                            length: read_u16(data_ptr, &mut location),
                                            name_index: read_u16(data_ptr, &mut location),
                                            signature_index: read_u16(data_ptr, &mut location),
                                            index: read_u16(data_ptr, &mut location),
                                        });
                                    }
                                    local_variable_type_table.push(table);
                                },
                                "StackMapTable" => {
                                    illegal_duplicate!(stack_map_table);
                                    let table_length = read_u16(data_ptr, &mut location);
                                    let mut table = Vec::with_capacity(table_length as usize);
                                    while table.capacity() > table.len() {
                                        let frame = {
                                            let tag = read_u8(data_ptr, &mut location);
                                            match tag {
                                                0..=63 => StackMapFrame::SameFrame(tag),
                                                64..=127 => StackMapFrame::SameLocals1StackItem(tag, VerificationTypeInfo::new(data_ptr, &mut location)?),
                                                128..=246 => return Err(Error::IllegalFrameType),
                                                247 => StackMapFrame::SameLocals1StackItemExtended(read_u16(data_ptr, &mut location), VerificationTypeInfo::new(data_ptr, &mut location)?),
                                                248..=250 => StackMapFrame::ChopFrame(tag, read_u16(data_ptr, &mut location)),
                                                251 => StackMapFrame::SameFrameExtended(read_u16(data_ptr, &mut location)),
                                                252..=254 => {
                                                    let offset_delta = read_u16(data_ptr, &mut location);
                                                    let mut locals = Vec::with_capacity(tag as usize - 251);
                                                    while locals.capacity() > locals.len() {
                                                        locals.push(VerificationTypeInfo::new(data_ptr, &mut location)?);
                                                    }
                                                    StackMapFrame::AppendFrame(tag, offset_delta, locals)
                                                }
                                                255 => {
                                                    let offset_delta = read_u16(data_ptr, &mut location);
                                                    let num_locals = read_u16(data_ptr, &mut location);
                                                    let mut locals = Vec::with_capacity(num_locals as usize);
                                                    while locals.capacity() > locals.len() {
                                                        locals.push(VerificationTypeInfo::new(data_ptr, &mut location)?);
                                                    }
                                                    let num_stack = read_u16(data_ptr, &mut location);
                                                    let mut stack = Vec::with_capacity(num_stack as usize);
                                                    while stack.capacity() > stack.len() {
                                                        stack.push(VerificationTypeInfo::new(data_ptr, &mut location)?);
                                                    }
                                                    StackMapFrame::FullFrame(offset_delta, locals, stack)
                                                }
                                            }
                                        };
                                        table.push(frame);
                                    }
                                    stack_map_table = Some(table);
                                },
                                "RuntimeVisibleTypeAnnotations" => {
                                    illegal_duplicate!(code_rt_vis_type_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                                    }
                                    code_rt_vis_type_annotations = Some(annotations);
                                },
                                "RuntimeInvisibleTypeAnnotations" => {

                                    illegal_duplicate!(code_rt_invis_type_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                                    }
                                    code_rt_invis_type_annotations = Some(annotations);
                                },
                                _ => location += length_code as isize, // Ignore custom attributes.
                            }
                            assert!(starting_location_code + length_code as isize == location);
                        }
                        code = Some(Code {
                            max_stack,
                            max_locals,
                            code: code_data,
                            exception_table,
                            line_number_table,
                            local_variable_table,
                            local_variable_type_table,
                            stack_map_table,
                            rt_vis_type_annotations: code_rt_vis_type_annotations,
                            rt_invis_type_annotations: code_rt_invis_type_annotations,
                        });
                    },
                    "Exceptions" => {
                        illegal_duplicate!(exceptions);
                        let num_exceptions = read_u16(data_ptr, &mut location);
                        let mut exception_table = Vec::with_capacity(num_exceptions as usize);
                        while exception_table.capacity() > exception_table.len() {
                            exception_table.push(read_u16(data_ptr, &mut location));
                        }
                        exceptions = Some(exception_table);
                    },
                    "RuntimeVisibleParameterAnnotations" => {
                        illegal_duplicate!(rt_vis_param_annotations);
                        let num_param_annotations = read_u16(data_ptr, &mut location);
                        let mut param_annotations = Vec::with_capacity(num_param_annotations as usize);
                        while param_annotations.capacity() > param_annotations.len() {
                            let num_annotations = read_u16(data_ptr, &mut location);
                            let mut annotations = Vec::with_capacity(num_annotations as usize);
                            while annotations.capacity() > annotations.len() {                            
                                annotations.push(Annotation::new(data_ptr, &mut location)?);
                            }
                            param_annotations.push(annotations);
                        }
                        rt_vis_param_annotations = Some(param_annotations);
                    },
                    "RuntimeInvisibleParameterAnnotations" => {
                        illegal_duplicate!(rt_invis_param_annotations);
                        let num_param_annotations = read_u16(data_ptr, &mut location);
                        let mut param_annotations = Vec::with_capacity(num_param_annotations as usize);
                        while param_annotations.capacity() > param_annotations.len() {
                            let num_annotations = read_u16(data_ptr, &mut location);
                            let mut annotations = Vec::with_capacity(num_annotations as usize);
                            while annotations.capacity() > annotations.len() {                            
                                annotations.push(Annotation::new(data_ptr, &mut location)?);
                            }
                            param_annotations.push(annotations);
                        }
                        rt_invis_param_annotations = Some(param_annotations);
                    },
                    "AnnotationDefault" => {
                        illegal_duplicate!(annotation_default);
                        annotation_default = Some(ElementValue::new(data_ptr, &mut location)?);
                    },
                    "MethodParameters" => {
                        illegal_duplicate!(method_parameters);
                        let num_params = read_u8(data_ptr, &mut location);
                        let mut parameters = Vec::with_capacity(num_params as usize);
                        while parameters.capacity() > parameters.len() {
                            parameters.push(MethodParameter {
                                name_index: read_u16(data_ptr, &mut location),
                                access_flags: read_u16(data_ptr, &mut location),
                            })
                        }
                        method_parameters = Some(parameters);
                    },
                    "Synthetic" => synthetic = true,
                    "Deprecated" => deprecated = true,
                    "Signature" => {
                        illegal_duplicate!(signature);
                        let index = read_u16(data_ptr, &mut location);
                        signature = Some(index);
                    },
                    "RuntimeVisibleAnnotations" => {
                        illegal_duplicate!(rt_vis_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(Annotation::new(data_ptr, &mut location)?);
                        }
                        rt_vis_annotations = Some(annotations);
                    },
                    "RuntimeInvisibleAnnotations" => {
                        illegal_duplicate!(rt_invis_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(Annotation::new(data_ptr, &mut location)?);
                        }
                        rt_invis_annotations = Some(annotations);
                    },
                    "RuntimeVisibleTypeAnnotations" => {
                        illegal_duplicate!(rt_vis_type_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                        }
                        rt_vis_type_annotations = Some(annotations);
                    },
                    "RuntimeInvisibleTypeAnnotations" => {
                        illegal_duplicate!(rt_invis_type_annotations);
                        let num_annotations = read_u16(data_ptr, &mut location);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        while annotations.capacity() > annotations.len() {
                            annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                        }
                        rt_invis_type_annotations = Some(annotations);
                    },
                    _ => location += length as isize, // Ignore custom attributes.
                }
                assert!(starting_location + length as isize == location);
            }
            let method = MethodInfo {
                access_flags: flags::method::AccessFlags{ flags: method_flags},
                name_index: method_name_index,
                descriptor_index,
                code,
                exceptions,
                rt_vis_param_annotations,
                rt_invis_param_annotations,
                annotation_default,
                method_parameters,
                synthetic,
                deprecated,
                signature,
                rt_vis_annotations,
                rt_invis_annotations,
                rt_vis_type_annotations,
                rt_invis_type_annotations,
            };
            methods.push(method)
        }
        let attributes_count = read_u16(data_ptr, &mut location);
        let mut source_file = None;
        let mut inner_classes = None;
        let mut enclosing_method = None;
        let mut source_debug_extension = None;
        let mut bootstrap_methods = None;
        let mut module = None;
        let mut module_packages = None;
        let mut module_main_class = None;
        let mut nest_host = None;
        let mut nest_members = None;
        let mut record = None;
        let mut permitted_subclasses = None;
        let mut synthetic = false;
        let mut deprecated = false;
        let mut signature = None;
        let mut rt_vis_annotations = None;
        let mut rt_invis_annotations = None;
        let mut rt_vis_type_annotations = None;
        let mut rt_invis_type_annotations = None;
        for _ in 0..attributes_count {
            let name_index = read_u16(data_ptr, &mut location);
            if name_index == 0 {
                return Err(Error::IllegalConstantPoolIndex);
            }
            let name: &str = cpool[name_index as usize - 1].as_utf8()?;
            let length = read_u32(data_ptr, &mut location);
            match name {
                "SourceFile" => {
                    illegal_duplicate!(source_file);
                    source_file = Some(read_u16(data_ptr, &mut location));
                },
                "InnerClasses" => {
                    illegal_duplicate!(inner_classes);
                    let num_classes = read_u16(data_ptr, &mut location);
                    let mut classes = Vec::with_capacity(num_classes as usize);
                    while classes.capacity() > classes.len() {
                        classes.push(InnerClass {
                            inner_class_info_index: read_u16(data_ptr, &mut location),
                            outer_class_info_index: read_u16(data_ptr, &mut location),
                            inner_name_index: read_u16(data_ptr, &mut location),
                            inner_class_access_flags: read_u16(data_ptr, &mut location),
                        });
                    }
                    inner_classes = Some(classes);
                }
                "EnclosingMethod" => {
                    illegal_duplicate!(enclosing_method);
                    enclosing_method = Some(EnclosingMethod {
                        class_index: read_u16(data_ptr, &mut location),
                        method_index: read_u16(data_ptr, &mut location),
                    });
                },
                "SourceDebugExtension" => {
                    illegal_duplicate!(source_debug_extension);
                    let mut extension = Vec::with_capacity(length as usize);
                    while extension.capacity() > extension.len() {
                        extension.push(read_u8(data_ptr, &mut location));
                    }
                    source_debug_extension = Some(extension);
                }
                "BootstrapMethods" => {
                    illegal_duplicate!(bootstrap_methods);
                    let numethods = read_u16(data_ptr, &mut location);
                    let mut methods = Vec::with_capacity(numethods as usize);
                    while methods.capacity() > methods.len() {
                        let method_ref = read_u16(data_ptr, &mut location);
                        let num_args = read_u16(data_ptr, &mut location);
                        let mut args = Vec::with_capacity(num_args as usize);
                        while args.capacity() > args.len() {
                            args.push(read_u16(data_ptr, &mut location));
                        }
                        methods.push(BootstrapMethod {
                            bootstrap_method_ref: method_ref,
                            bootstrap_arguments: args
                        });
                    }
                    bootstrap_methods = Some(methods);
                },
                "Module" => {
                    illegal_duplicate!(module);
                    let name_index = read_u16(data_ptr, &mut location);
                    let flags = read_u16(data_ptr, &mut location);
                    let version_index = read_u16(data_ptr, &mut location);
                    let requires_count = read_u16(data_ptr, &mut location);
                    let mut requires = Vec::with_capacity(requires_count as usize);
                    while requires.capacity() > requires.len() {
                        requires.push(Require {
                            requires_index: read_u16(data_ptr, &mut location),
                            requires_flags: read_u16(data_ptr, &mut location),
                            requires_version_count: read_u16(data_ptr, &mut location),
                        })
                    }
                    let exports_count = read_u16(data_ptr, &mut location);
                    let mut exports = Vec::with_capacity(exports_count as usize);
                    while exports.capacity() > exports.len() {
                        let exports_index = read_u16(data_ptr, &mut location);
                        let exports_flags = read_u16(data_ptr, &mut location);
                        let exports_to_count = read_u16(data_ptr, &mut location);
                        let mut exports_to_index = Vec::with_capacity(exports_to_count as usize);
                        while exports_to_index.capacity() > exports_to_index.len() {
                            exports_to_index.push(read_u16(data_ptr, &mut location));
                        }
                        exports.push(Export {
                            exports_index,
                            exports_flags,
                            exports_to_index,
                        })
                    }
                    let opens_count = read_u16(data_ptr, &mut location);
                    let mut opens = Vec::with_capacity(opens_count as usize);
                    while opens.capacity() > opens.len() {
                        let opens_index = read_u16(data_ptr, &mut location);
                        let opens_flags = read_u16(data_ptr, &mut location);
                        let opens_to_count = read_u16(data_ptr, &mut location);
                        let mut opens_to_index = Vec::with_capacity(opens_to_count as usize);
                        while opens_to_index.capacity() > opens_to_index.len() {
                            opens_to_index.push(read_u16(data_ptr, &mut location));
                        }
                        opens.push(Open {
                            opens_index,
                            opens_flags,
                            opens_to_index,
                        })
                    }
                    let uses_count = read_u16(data_ptr, &mut location);
                    let mut uses = Vec::with_capacity(uses_count as usize);
                    while uses.capacity() > uses.len() {
                        uses.push(read_u16(data_ptr, &mut location));
                    }
                    let provides_count = read_u16(data_ptr, &mut location);
                    let mut provides = Vec::with_capacity(provides_count as usize);
                    while provides.capacity() > provides.len() {
                        let provides_index = read_u16(data_ptr, &mut location);
                        let provides_with_count = read_u16(data_ptr, &mut location);
                        let mut provides_with_index = Vec::with_capacity(provides_with_count as usize);
                        while provides_with_index.capacity() > provides_with_index.len() {
                            provides_with_index.push(read_u16(data_ptr, &mut location));
                        }
                        provides.push(Provide {
                            provides_index,
                            provides_with_index,
                        })
                    }
                    module = Some(Module {
                        module_name_index: name_index,
                        module_flags: flags,
                        module_version_index: version_index,
                        requires,
                        exports,
                        opens,
                        uses,
                        provides,
                    })
                }
                "ModulePackages" => {
                    illegal_duplicate!(module_packages);
                    let package_count = read_u16(data_ptr, &mut location);
                    let mut packages = Vec::with_capacity(package_count as usize);
                    while packages.capacity() > packages.len() {
                        packages.push(read_u16(data_ptr, &mut location));
                    }
                    module_packages = Some(packages);
                }
                "ModuleMainClass" => {
                    illegal_duplicate!(module_main_class);
                    module_main_class = Some(read_u16(data_ptr, &mut location));
                }
                "NestHost" => {
                    illegal_duplicate!(nest_host);
                    nest_host = Some(read_u16(data_ptr, &mut location));
                }
                "NestMembers" => {
                    illegal_duplicate!(nest_members);
                    let num_members = read_u16(data_ptr, &mut location);
                    let mut members = Vec::with_capacity(num_members as usize);
                    while members.capacity() > members.len() {
                        members.push(read_u16(data_ptr, &mut location));
                    }
                    nest_members = Some(members);
                }
                "Record" => {
                    illegal_duplicate!(record);
                    let num_components = read_u16(data_ptr, &mut location);
                    let mut record_components = Vec::with_capacity(num_components as usize);
                    while record_components.capacity() > record_components.len() {
                        let record_name_index = read_u16(data_ptr, &mut location);
                        let descriptor_index = read_u16(data_ptr, &mut location);
                        let nurecord_attributes = read_u16(data_ptr, &mut location);
                        let mut record_signature = None;
                        let mut record_rt_vis_annotations = None;
                        let mut record_rt_invis_annotations = None;
                        let mut record_rt_vis_type_annotations = None;
                        let mut record_rt_invis_type_annotations = None;
                        for _ in 0..nurecord_attributes {
                            let name_index_record = read_u16(data_ptr, &mut location);
                            if name_index_record == 0 {
                                return Err(Error::IllegalConstantPoolIndex);
                            }
                            let name_record: &str = cpool[name_index_record as usize - 1].as_utf8()?;
                            let length_record = read_u32(data_ptr, &mut location);
                            match name_record {
                                "Signature" => {
                                    illegal_duplicate!(record_signature);
                                    let index = read_u16(data_ptr, &mut location);
                                    record_signature = Some(index);
                                },
                                "RuntimeVisibleAnnotations" => {
                                    illegal_duplicate!(record_rt_vis_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(Annotation::new(data_ptr, &mut location)?);
                                    }
                                    record_rt_vis_annotations = Some(annotations);
                                },
                                "RuntimeInvisibleAnnotations" => {
                                    illegal_duplicate!(record_rt_invis_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(Annotation::new(data_ptr, &mut location)?);
                                    }
                                    record_rt_invis_annotations = Some(annotations);
                                },
                                "RuntimeVisibleTypeAnnotations" => {
                                    illegal_duplicate!(record_rt_vis_type_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                                    }
                                    record_rt_vis_type_annotations = Some(annotations);
                                },
                                "RuntimeInvisibleTypeAnnotations" => {
                                    illegal_duplicate!(record_rt_invis_type_annotations);
                                    let num_annotations = read_u16(data_ptr, &mut location);
                                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                                    while annotations.capacity() > annotations.len() {
                                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                                    }
                                    record_rt_invis_type_annotations = Some(annotations);
                                },
                                _ => location += length_record as isize,
                            }
                        }
                        record_components.push(RecordComponentInfo { name_index: record_name_index, descriptor_index, signature: record_signature, rt_vis_annotations: record_rt_vis_annotations, 
                            rt_invis_annotations: record_rt_invis_annotations, rt_vis_type_annotations: record_rt_vis_type_annotations, rt_invis_type_annotations: record_rt_invis_type_annotations })
                    }
                    record = Some(record_components);
                }
                "PermittedSubclasses" => {
                    illegal_duplicate!(permitted_subclasses);
                    let num_classes = read_u16(data_ptr, &mut location);
                    let mut classes = Vec::with_capacity(num_classes as usize);
                    while classes.capacity() > classes.len() {
                        classes.push(read_u16(data_ptr, &mut location));
                    }
                    permitted_subclasses = Some(classes);
                }
                "Synthetic" => synthetic = true,
                "Deprecated" => deprecated = true,
                "Signature" => {
                    illegal_duplicate!(signature);
                    let index = read_u16(data_ptr, &mut location);
                    signature = Some(index);
                },
                "RuntimeVisibleAnnotations" => {
                    illegal_duplicate!(rt_vis_annotations);
                    let num_annotations = read_u16(data_ptr, &mut location);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.capacity() > annotations.len() {
                        annotations.push(Annotation::new(data_ptr, &mut location)?);
                    }
                    rt_vis_annotations = Some(annotations);
                },
                "RuntimeInvisibleAnnotations" => {
                    illegal_duplicate!(rt_invis_annotations);
                    let num_annotations = read_u16(data_ptr, &mut location);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.capacity() > annotations.len() {
                        annotations.push(Annotation::new(data_ptr, &mut location)?);
                    }
                    rt_invis_annotations = Some(annotations);
                },
                "RuntimeVisibleTypeAnnotations" => {
                    illegal_duplicate!(rt_vis_type_annotations);
                    let num_annotations = read_u16(data_ptr, &mut location);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.capacity() > annotations.len() {
                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                    }
                    rt_vis_type_annotations = Some(annotations);
                },
                "RuntimeInvisibleTypeAnnotations" => {
                    illegal_duplicate!(rt_invis_type_annotations);
                    let num_annotations = read_u16(data_ptr, &mut location);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    while annotations.capacity() > annotations.len() {
                        annotations.push(TypeAnnotation::new(data_ptr, &mut location)?);
                    }
                    rt_invis_type_annotations = Some(annotations);
                },
                _ => location += length as isize, // Ignore custom attributes.
            }
        }
        Ok( ClassFile {
            minor_version,
            major_version,
            constant_pool: cpool,
            access_flags: flags::class::AccessFlags{ flags: access_flags },
            this_class_index: this_class,
            super_class_index: super_class,
            interfaces,
            fields,
            methods,
            source_file,
            inner_classes,
            enclosing_method,
            source_debug_extension,
            bootstrap_methods,
            module,
            module_packages,
            module_main_class,
            nest_host,
            nest_members,
            record,
            permitted_subclasses,
            synthetic,
            deprecated,
            signature,
            rt_vis_annotations,
            rt_invis_annotations,
            rt_vis_type_annotations,
            rt_invis_type_annotations,
        })
    }
}

impl ClassFile {
    pub fn verify_state(&self) -> bool {
        true
    }
}


use std::fmt::{self, Debug};

impl fmt::Display for  ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Version: {}.{}", self.major_version, self.minor_version)?;
        writeln!(f, "Size of Constant Pool: {}", self.constant_pool.len())?;
        writeln!(f, "Constant Pool:")?;
        for entry in &self.constant_pool {
            write!(f, "{entry}")?;
        }
        writeln!(f, "Access flags: {}", self.access_flags)?;
        writeln!(f, "This class index: {}", self.this_class_index)?;
        writeln!(f, "This class entry:")?;
        let this_index = self.cp_entry(self.this_class_index).unwrap();
        writeln!(f, "{this_index}")?;
        writeln!(f, "This class name:")?;
        writeln!(f, "{}", self.cp_entry(*this_index.as_class().unwrap()).unwrap())?;
        if self.has_super() {
            writeln!(f, "Super class index: {}", self.super_class_index)?;
            writeln!(f, "Super class entry:")?;
            let super_index = self.cp_entry(self.super_class_index).unwrap();
            writeln!(f, "{super_index}")?;
            writeln!(f, "Super class name:")?;
            writeln!(f, "{}", self.cp_entry(*super_index.as_class().unwrap()).unwrap())?;
        }
        else {
            writeln!(f, "This class has no super class")?;
        }
        writeln!(f, "Number of interfaces: {}", self.interfaces.len())?;
        writeln!(f, "Interfaces:")?;
        for interface in &self.interfaces {
            writeln!(f, "Interface index: {interface}")?;
            writeln!(f, "Interface: ")?;
            write!(f, "{}", self.cp_entry(*interface).unwrap())?;
        }
        writeln!(f, "Number of fields: {}", self.fields.len())?;
        writeln!(f, "Fields:")?;
        for field in &self.fields {
            write!(f, "{field:#?}")?;
        }
        writeln!(f, "Number of methods: {}", self.methods.len())?;
        writeln!(f, "Methods:")?;
        for method in &self.methods {
            write!(f, "{method:#?}")?;
        }
        if let Some(file) = self.source_file {
            writeln!(f, "Source file index: {file}")?;
            writeln!(f, "Source file: {}", self.cp_entry(file).unwrap())?;
        }
        else {
            writeln!(f, "This class has no SourceFile attribute")?;
        }
        if let Some(classes) = &self.inner_classes {
            writeln!(f, "Number of inner classes: {}", classes.len())?;
            writeln!(f, "Inner classes:")?;
            for class in classes {
                write!(f, "{class:#?}")?;
            }
        }
        else {
            writeln!(f, "This class has no InnerClasses attribute")?;
        }
        if let Some(method) = &self.enclosing_method {
            writeln!(f, "Enclosing method: {method:#?}")?;
        }
        else {
            writeln!(f, "This class has no EnclosingMethod attribute")?;
        }
        if let Some(extension) = &self.source_debug_extension {
            writeln!(f, "Source debug extension: {extension:#?}")?;
        }
        else {
            writeln!(f, "This class has no SourceDebugExtension attribute")?;
        }
        if let Some(bootstraps) = &self.bootstrap_methods {
            writeln!(f, "Number of bootstrap methods; {}", bootstraps.len())?;
            writeln!(f, "Bootstrap methods:")?;
            for method in bootstraps {
                write!(f, "{method:#?}")?;
            }
        }
        else {
            writeln!(f, "This class has no BootstrapMethods attribute")?;
        }
        if let Some(module) = &self.module {
            writeln!(f, "Module: {module:#?}")?;
        }
        else {
            writeln!(f, "This class has no Module attribute")?;
        }
        if let Some(packages) = &self.module_packages {
            writeln!(f, "Number of module packages: {}", packages.len())?;
            writeln!(f, "Module packages:")?;
            for package_index in packages {
                write!(f, "{}", self.cp_entry(*package_index).unwrap())?;
            }
        }
        else {
            writeln!(f, "This class has no ModulePackages attribute")?;
        }
        if let Some(main_class) = self.module_main_class {
            writeln!(f, "Module main class index: {main_class}")?;
            writeln!(f, "Module main class: {}", self.cp_entry(main_class).unwrap())?;
        }
        else {
            writeln!(f, "This class has no ModuleMainClass attribute")?;
        }
        if let Some(host) = self.nest_host {
            writeln!(f, "Host class: {}", self.cp_entry(host).unwrap())?;
        }
        else {
            writeln!(f, "This class has no NestHost attribute")?;
        }
        if let Some(members) = &self.nest_members {
            writeln!(f, "Number of nest members: {}", members.len())?;
            writeln!(f, "Nest members:")?;
            for member in members {
                write!(f, "{}", self.cp_entry(*member).unwrap())?;
            }
        }
        else {
            writeln!(f, "This class has no NestMembers attribute")?;
        }
        if let Some(record) = &self.record {
            writeln!(f, "Number of record components: {}", record.len())?;
            writeln!(f, "Record components:")?;
            for component in record {
                write!(f, "{component:#?}")?;
            }
        }
        else {
            writeln!(f, "This class has no Record attribute")?;
        }
        if let Some(subclasses) = &self.permitted_subclasses {
            writeln!(f, "Number of permitted subclasses: {}", subclasses.len())?;
            writeln!(f, "Permitted subclasses:")?;
            for class in subclasses {
                write!(f, "{}", self.cp_entry(*class).unwrap())?;
            } 
        }
        else {
            writeln!(f, "This class has no PermittedSubclasses attribute")?;
        }
        if self.synthetic {
            writeln!(f, "This class is synthetic")?;
        }
        if self.deprecated {
            writeln!(f, "WARNING: This class is deprecated, and should not be used")?;
        }
        if let Some(signature) = self.signature {
            writeln!(f, "Class signature: {}", self.cp_entry(signature).unwrap())?;
        }
        else {
            writeln!(f, "This class has no Signature attribute")?;
        }
        if let Some(annotations) = &self.rt_vis_annotations {
            writeln!(f, "Number of visible annotations: {}", annotations.len())?;
            writeln!(f, "Visible annotations:")?;
            for annotation in annotations {
                write!(f, "{annotation:#?}")?;
            }
        }
        else {
            writeln!(f, "This class has no RuntimeVisibleAnnotations attribute")?;
        } 
        if let Some(annotations) = &self.rt_invis_annotations {
            writeln!(f, "Number of invisible annotations: {}", annotations.len())?;
            writeln!(f, "Invisible annotations:")?;
            for annotation in annotations {
                write!(f, "{annotation:#?}")?;
            }
        }
        else {
            writeln!(f, "This class has no RuntimeInvisibleAnnotations attribute")?;
        } 
        if let Some(annotations) = &self.rt_vis_type_annotations {
            writeln!(f, "Number of visible type annotations: {}", annotations.len())?;
            writeln!(f, "Visible type annotations:")?;
            for annotation in annotations {
                write!(f, "{annotation:#?}")?;
            }
        }
        else {
            writeln!(f, "This class has no RuntimeVisibleAnnotations attribute")?;
        } 
        if let Some(annotations) = &self.rt_invis_type_annotations {
            writeln!(f, "Number of invisible type annotations: {}", annotations.len())?;
            writeln!(f, "Invisible type annotations:")?;
            for annotation in annotations {
                write!(f, "{annotation:#?}")?;
            }
            Ok(())
        }
        else {
            write!(f, "This class has no RuntimeInvisibleAnnotations attribute")
        } 
    }
}