use std::{collections::HashMap, rc::Rc};

use crate::{constant_pool::{NameAndType, Entry}, value::Value, errorcodes::Opcode, flags, reference::{self, Reference}};

use super::*;

pub struct CustomClass {
    class_file: Rc<classfile::ClassFile>,
    static_fields: HashMap<NameAndType, Rc<Value<dyn Class, dyn Object>>>, 
}

impl<'a> Class for CustomClass {
    // We could use a different type than NameAndType for the &Strings, but this is simpler and terribly slow.
    fn new(jvm: &mut JVM, file: classfile::ClassFile) -> Result<Self, Error> where Self : Sized {
        let mut static_fields = HashMap::new();
        // Fill in static fields.
        for field in &file.fields {
            if (field.access_flags.flags & flags::field::ACC_STATIC) > 0 {
                let static_field_name = file.constant_pool[field.name_index as usize - 1].as_utf8()?.clone();
                let static_field_desc = file.constant_pool[field.descriptor_index as usize - 1].as_utf8()?.clone();
                let name_and_type = NameAndType {
                    name: static_field_name,
                    descriptor: static_field_desc,
                };
                let value = {
                    if field.constant_value.is_some() {
                        match file.constant_pool[field.constant_value.unwrap() as usize - 1] {
                            Entry::Integer(i) => match &name_and_type.descriptor[0..1] {
                                "B" | "Z" => Value::Byte(i),
                                "C" => Value::Char(i),
                                "I" => Value::Int(i),
                                "S" => Value::Short(i),
                                _ => return Err(Error::IllegalCastToInt),
                            },
                            Entry::Float(f) => Value::Float(f),
                            Entry::Long(l) => Value::Long(l),
                            Entry::Double(d) => Value::Double(d),
                            Entry::String(s_index) => {
                                // This should make a new instance of String, but for now we will just give a null ref.
                                Value::Reference(Rc::new(Reference::Null))
                            },
                            _ => return Err(Error::IllegalConstantLoad(Opcode::ClassLoad)),
                        }
                    }
                    else {
                        match &name_and_type.descriptor[0..1] {
                            "B" | "Z" => Value::Byte(0),
                            "C" => Value::Char(0),
                            "I" => Value::Int(0),
                            "S" => Value::Short(0),
                            "F" => Value::Float(0.0),
                            "J" => Value::Long(0),
                            "D" => Value::Double(0.0),
                            "[" | "L" => {
                                Value::Reference(Rc::new(Reference::Null))
                            },
                            _ => return Err(Error::IllegalDescriptor),
                        }
                    }
                };
                static_fields.insert(name_and_type, Rc::new(value));
            }
        }
        Ok(CustomClass { class_file: Rc::new(file), static_fields })
    }
    fn get_static(&self, name: &String, descriptor: &String, jvm: &mut JVM) -> Result<Rc<Value<dyn Class, dyn Object>>, Error> {
        let name_and_type = NameAndType { name: String::from(name), descriptor: String::from(descriptor) };
        let mut class;
        let mut current_class = &jvm.resolve_class_reference(self.class_file.name())?;
        while current_class.has_super() {
            // First, check the current class's fields.
            if let Some(v) = current_class.m_static_fields.get(&name_and_type) {
                return Ok(v.clone());
            }
            // Next, we have to check the interfaces

            for interface_index in &current_class.m_interfaces {
                let interface_name_index = current_class.cp_entry(*interface_index)?.as_class()?;
                let interface_name = current_class.cp_entry(*interface_name_index)?.as_utf8()?;
                let mut interface;
                let mut current_interface = &jvm.resolve_class_reference(interface_name)?;
                // Now we loop over the superclasses of the interface.
                while current_interface.has_super() {
                    if let Some(v) = current_interface.m_static_fields.get(&name_and_type) {
                        return Ok(v.clone());
                    }
                    interface = jvm.resolve_class_reference(current_interface.super_name().unwrap())?;
                    current_interface = &interface;
                }
            }
            // Finally, we propogate to the superclasses of the current class.
            class = jvm.resolve_class_reference(current_class.super_name().unwrap())?;
            current_class = &class;
        }
        Err(Error::NoSuchFieldError(Opcode::GETSTATIC))
    }
    fn put_static(&mut self, name: &String, descriptor: &String, value: Value<dyn Class, dyn Object>, jvm: &mut JVM) -> Result<(), Error> {
        let name_and_type = NameAndType { name: String::from(name), descriptor: String::from(descriptor) };
        let mut current_class = jvm.resolve_class_reference(self.class_file.name())?;
        while current_class.has_super() {
            // First, check the current class's fields.
            if let Some(v) = unsafe { Rc::get_mut_unchecked(&mut current_class).m_static_fields.get_mut(&name_and_type) } {
                unsafe {*Rc::get_mut_unchecked(v) = value; }
                return Ok(());
            }
            // We actually don't need to check the interfaces, because interface variables are final and cannot be modified.
            // That just means we need to get the next super class.
            current_class = jvm.resolve_class_reference(current_class.super_name().unwrap())?;
        }
        Err(Error::NoSuchFieldError(Opcode::PUTSTATIC))
    }
    fn exec_method<C: Class>(&mut self, current_method_class: Rc<C>, jvm: &mut JVM, method: &MethodInfo) -> Result<bool, Error> {
        
    }
    fn get_class_file(&self) -> Rc<ClassFile> {
        self.class_file.clone()
    }
}


