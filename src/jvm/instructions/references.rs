use crate::{flags, reference::{array::Array, object}};

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GetStatic {
    index: u16,
}
impl Instruction for GetStatic {
    fn name(&self) -> &'static str {
        "getstatic"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(GetStatic { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let current_class = Rc::clone(&frame.rt_const_pool);
        let current_class_file = current_class.get_class_file();
        let field = current_class_file.cp_entry(self.index)?.as_field_ref()?;
        let class_index = current_class_file.cp_entry(field.class_index)?.as_class()?;
        let class_name = current_class_file.cp_entry(*class_index)?.as_utf8()?;
        let class = jvm.resolve_class_reference(class_name)?;
        let name_and_type = current_class_file.cp_entry(field.name_and_type_index)?.as_name_and_type()?;
        let name = current_class_file.cp_entry(name_and_type.name_index)?.as_utf8()?;
        let descriptor = current_class_file.cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
        let field = class.get_static(name, descriptor, jvm)?;
        let new_value = field.clone();
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(new_value);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PutStatic {
    index: u16,
}
impl Instruction for PutStatic {
    fn name(&self) -> &'static str {
        "putstatic"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(PutStatic { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let current_class = Rc::clone(&frame.rt_const_pool);
        let current_class_file = current_class.get_class_file();
        let field = current_class_file.cp_entry(self.index)?.as_field_ref()?;
        let class_index = current_class_file.cp_entry(field.class_index)?.as_class()?;
        let class_name = current_class_file.cp_entry(*class_index)?.as_utf8()?;
        let mut class = jvm.resolve_class_reference(class_name)?;
        let name_and_type = current_class_file.cp_entry(field.name_and_type_index)?.as_name_and_type()?;
        let name = current_class_file.cp_entry(name_and_type.name_index)?.as_utf8()?;
        let descriptor = current_class_file.cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let field = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::PUTSTATIC)),
        };
        //println!("{class_name}");
        //todo!();
        unsafe {Rc::get_mut_unchecked(&mut class).put_static(name, descriptor, field, jvm)?; }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GetField {
    index: u16,
}
impl Instruction for GetField {
    fn name(&self) -> &'static str {
        "getfield"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(GetField { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let object_ref = match frame.op_stack.pop() {
            Some(mut v) => v.as_reference_mut()?,
            None => return Err(Error::StackUnderflow(Opcode::GETFIELD)),
        };
        let current_class = frame.rt_const_pool.clone();
        let val = {
            let object = match object_ref {
                Reference::Object(o, _) => o,
                _ => return Err(Error::IncorrectReferenceType(Opcode::GETFIELD)),
            };
            object.get_field(current_class, self.index, jvm)?
        };
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(val);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PutField {
    index: u16,
}
impl Instruction for PutField {
    fn name(&self) -> &'static str {
        "putfield"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(PutField { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::PUTFIELD)),
        };
        let object_ref = match frame.op_stack.pop() {
            Some(mut v) => v.as_reference_mut()?,
            None => return Err(Error::StackUnderflow(Opcode::PUTFIELD)),
        };
        let mut object = match object_ref {
            Reference::Object(o, _) => o,
            _ => return Err(Error::IncorrectReferenceType(Opcode::PUTFIELD)),
        };
        unsafe {Rc::get_mut_unchecked(&mut object)}
        .put_field(frame.rt_const_pool.clone(), self.index, jvm, val)?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvokeVirtual {
    index: u16,
}
impl Instruction for InvokeVirtual {
    fn name(&self) -> &'static str {
        "invokevirtual"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(InvokeVirtual { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-6.html#jvms-6.5.invokevirtual
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let current_class = Rc::clone(&frame.rt_const_pool);
        let current_class_file = current_class.get_class_file();
        let (mut c, name, descriptor) = {  
            let method_ref = current_class_file.cp_entry(self.index)?.as_method_ref()?;
            let name_and_type = current_class_file.cp_entry(method_ref.name_and_type_index)?.as_name_and_type()?;
            let name = current_class_file.cp_entry(name_and_type.name_index)?.as_utf8()?;
            let descriptor = current_class_file.cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
            let c_info = current_class_file.cp_entry(method_ref.class_index)?.as_class()?;
            let c_name = current_class_file.cp_entry(*c_info)?.as_utf8()?;
            (jvm.resolve_class_reference(c_name.clone().as_str())?, name, descriptor)
        };
        let c_file = c.get_class_file();
        //println!("Got {}.{}{}", c_file.name(), name, descriptor);
        let mut method_to_call = None; 
        // Resolve method
        {
            let mut found = false;
            'super_loop: while !found {
                for method in c_file.methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = c_file.cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = c_file.cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        method_to_call = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found {
                    if !c.get_class_file().has_super() { break 'super_loop; }
                    c = jvm.resolve_class_reference(c.get_class_file().super_name().unwrap())?;
                }
                
            }
            // TODO: Search Superinterfaces of c.
            if !found {
                return Err(Error::NoSuchMethodError(Opcode::INVOKEVIRTUAL));
            }
        }
        
        let resolved_method = method_to_call.unwrap();
        /*
        {
            // search the methods of the object to see if we can override. 
            if (resolved_method.access_flags.flags & flags::method::ACC_PRIVATE) == 0 {
                let mut obj_c = object.class().clone();
                let mut obj_c_file = obj_c.get_class_file();
                let mut found = false;
                while obj_c.get_class_file().has_super() && !found {
                    for method in obj_c.get_class_file().methods() {
                        // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                        // We still need to check for signature polymorphic functions.
                        let method_descriptor = obj_c_file.cp_entry(method.descriptor_index)?.as_utf8()?;
                        if method_descriptor != descriptor {
                            continue;
                        }
                        let method_name = obj_c_file.cp_entry(method.name_index)?.as_utf8()?;
                        if method_name != name {
                            continue;
                        }
                        // we *should* check access control, but we don't
                        resolved_method = method.clone();
                        found = true;
                        break;
                    }   
                    // Recurse up the inheritance tree.
                    if !found {
                        obj_c = self.resolve_class_reference(obj_c.get_class_file().super_name().unwrap())?;
                        obj_c_file = obj_c.get_class_file();
                    }
                    
                }
                // TODO: Search Superinterfaces of c.
            }
        };
        */
        if (resolved_method.access_flags.flags & flags::method::ACC_STATIC) > 0 {
            return Err(Error::IncompatibleClassChangeError(Opcode::INVOKEVIRTUAL));
        } 
        if (resolved_method.access_flags.flags & flags::method::ACC_ABSTRACT) > 0 {
            return Err(Error::AbstractMethodError(Opcode::INVOKEVIRTUAL));
        } 
        if (resolved_method.access_flags.flags & flags::method::ACC_SYNCHRONIZED) > 0 {
            // TODO: Enter monitors on Classes.
        } 
        jvm.execute_on_object(&resolved_method, c)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvokeSpecial {
    index: u16,
}
impl Instruction for InvokeSpecial {
    fn name(&self) -> &'static str {
        "invokespecial"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(InvokeSpecial { index })
        }
    }
    //https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-6.html#jvms-6.5.invokespecial
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let rt_file = frame.rt_const_pool.get_class_file();
        let entry = rt_file.cp_entry(self.index)?;
        let (method_ref, _) = match entry {
            Entry::MethodRef(refinfo) => (refinfo, false),
            Entry::InterfaceMethodRef(refinfo) => (refinfo, true),
            _ => return Err(Error::IllegalConstantLoad(Opcode::INVOKESPECIAL)),
        };
        let current_class = Rc::clone(&frame.rt_const_pool);
        let current_class_file = current_class.get_class_file();
        let name_and_type = current_class_file.cp_entry(method_ref.name_and_type_index)?.as_name_and_type()?;
        let name = current_class_file.cp_entry(name_and_type.name_index)?.as_utf8()?;
        let descriptor = current_class_file.cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
        let c_info = current_class_file.cp_entry(method_ref.class_index)?.as_class()?;
        let c_name = current_class_file.cp_entry(*c_info)?.as_utf8()?;
        let mut c = jvm.resolve_class_reference(c_name)?;
        let mut c_file = c.get_class_file();
        if (c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) > 0 {
            return Err(Error::IncompatibleClassChangeError(Opcode::INVOKESPECIAL));
        } 


        // Resolve method
        let mut resolved_method_wrapped = None; 
        {
            let mut found = false;
            while !found {
                for method in c_file.methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = c_file.cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = c_file.cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        resolved_method_wrapped = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found {
                    if !c_file.has_super() { break; }

                    c = jvm.resolve_class_reference(c_file.super_name().unwrap())?;
                    c_file = c.get_class_file();
                }
                
            }
            // TODO: Search Superinterfaces of c.
            if !found {
                return Err(Error::NoSuchMethodError(Opcode::MethodInvoke));
            }
        }
        let resolved_method = resolved_method_wrapped.unwrap();
        let resolved_method_class = Rc::clone(&c); // Save the originating class of resolved_method so we can index it.
        let res_method_file = resolved_method_class.get_class_file();
        let resolved_name = res_method_file.cp_entry(resolved_method.name_index)?.as_utf8()?;
        let resolved_desc = res_method_file.cp_entry(resolved_method.descriptor_index)?.as_utf8()?;
        // Next, possibly change C
        
        if (c.get_class_file().cp_entry(resolved_method.name_index)?.as_utf8()? != "<init>") && current_class.get_class_file().has_super() && ({
            // Figure out if c is a superclass of current_class
            let mut temp = Rc::clone(&current_class);
            let mut temp_file = temp.get_class_file();
            let mut res = false;
            while temp_file.has_super() {
                let super_name = temp_file.super_name().unwrap();
                if super_name == c.get_class_file().name() {
                    res = true;
                    break;
                }
                temp = jvm.resolve_class_reference(super_name)?;
                temp_file = temp.get_class_file();
            }
            res
        })
        && ((c.get_class_file().access_flags().flags & flags::class::ACC_SUPER) > 0 ) {
            c = jvm.resolve_class_reference(current_class.get_class_file().super_name().unwrap())?;
        }

        let mut actual_method_wrapped = None;
        // Now, select *actual* method
        // All methods have to be instance methods. 
        // First, check methods of c for a method with the same name and desc.
        // and Check superclasses, if c is a class
        if actual_method_wrapped.is_none() && ((c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) == 0) {
            let mut c_super = Rc::clone(&c);
            let mut found = false;
            while !found {
                for method in c.get_class_file().methods() {
                    if (method.access_flags.flags & flags::method::ACC_STATIC) > 0 {
                        continue;
                    }
                    if (c_super.get_class_file().cp_entry(method.name_index)?.as_utf8()? == resolved_name) && 
                        (c_super.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()? == resolved_desc) {
                        actual_method_wrapped = Some(method.clone());
                        c = Rc::clone(&c_super);
                        found = true;
                        break;
                    }
                }
                if !found {
                    if !c_super.get_class_file().has_super() { break; }

                    c_super = jvm.resolve_class_reference(c_super.get_class_file().super_name().unwrap())?;
                }
            }
        }
        // Check object, if c in an interface.
        if actual_method_wrapped.is_none() && ((c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) > 0) {
            let obj_class = jvm.resolve_class_reference("java/lang/Object")?;
            for method in obj_class.get_class_file().methods() {
                if (method.access_flags.flags & flags::method::ACC_STATIC) > 0 {
                    continue;
                }
                if (obj_class.get_class_file().cp_entry(method.name_index)?.as_utf8()? == resolved_name) && 
                   (obj_class.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()? == resolved_desc) {
                    actual_method_wrapped = Some(method.clone());
                    c = Rc::clone(&obj_class);
                    break;
                }
            }
        }
        // TODO: Check maximally specific methods in superinterfaces.

        // I *think* the way this works is that if we don't find a method, we use the one we resolved.
        let actual_method = actual_method_wrapped.unwrap();


        if (actual_method.access_flags.flags & flags::method::ACC_STATIC) > 0 {
            return Err(Error::IncompatibleClassChangeError(Opcode::INVOKESPECIAL));
        } 
        if (actual_method.access_flags.flags & flags::method::ACC_ABSTRACT) > 0 {
            return Err(Error::AbstractMethodError(Opcode::INVOKESPECIAL));
        } 
        if (actual_method.access_flags.flags & flags::method::ACC_SYNCHRONIZED) > 0 {
            let current_thread_number = jvm.m_thread_index;
            let thread = access_macros::current_thread_mut!(jvm);
            let frame = access_macros::current_frame_mut!(thread);
            // FIXME: This code is completely wrong, and should be addressed in either execute_on_object or the object's method call function.
            let object = match frame.op_stack.pop() {
                Some(o) => o.as_reference()?,
                None => return Err(Error::StackUnderflow(Opcode::INVOKESPECIAL)),
            };
            match object {
                Reference::Null => return Err(Error::NullPointerException(Opcode::MONITORENTER)),
                Reference::Array(_, mut mrc) | Reference::Interface(_, mut mrc) | Reference::Object(_, mut mrc) => {
                    let m = unsafe { Rc::get_mut_unchecked(&mut mrc)};
                    let success = m.try_enter(current_thread_number);
                    if !success {
                        thread.current_monitor = Some(mrc.clone());
                        return Ok(()); // We have to block
                    }
                },
            }
        } 
        // TODO: Refactor jvm.setup_method_call() to handle native methods and synchronized ones (and rename it).
        jvm.execute_on_object(&actual_method, c)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvokeStatic {
    index: u16,
}
impl Instruction for InvokeStatic {
    fn name(&self) -> &'static str {
        "invokestatic"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(InvokeStatic { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let rt_file = frame.rt_const_pool.get_class_file();   
        let entry = rt_file.cp_entry(self.index)?;
        let (method_ref, is_interface) = match entry {
            Entry::MethodRef(refinfo) => (refinfo, false),
            Entry::InterfaceMethodRef(refinfo) => (refinfo, true),
            _ => return Err(Error::IllegalConstantLoad(Opcode::INVOKESTATIC)),
        };
        let current_class = Rc::clone(&frame.rt_const_pool);
        let current_class_file = current_class.get_class_file();
        let name_and_type = current_class_file.cp_entry(method_ref.name_and_type_index)?.as_name_and_type()?;
        let name = current_class_file.cp_entry(name_and_type.name_index)?.as_utf8()?;
        let descriptor = current_class_file.cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
        let c_info = current_class_file.cp_entry(method_ref.class_index)?.as_class()?;
        let c_name = current_class_file.cp_entry(*c_info)?.as_utf8()?;
        let mut c = jvm.resolve_class_reference(c_name)?;
        let mut c_file = c.get_class_file();
        if is_interface {
            if (c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) == 0 {
                return Err(Error::IncompatibleMethodRefAndClass(Opcode::INVOKESTATIC));
            }
        }
        else if (c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) > 0 {
                return Err(Error::IncompatibleMethodRefAndClass(Opcode::INVOKESTATIC));
        }
        let mut method_to_call = None; 
        // Resolve method
        {
            let mut found = false;
            while c_file.has_super() && !found {
                for method in c_file.methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = c_file.cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = c_file.cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        method_to_call = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found {
                    c = jvm.resolve_class_reference(c.get_class_file().super_name().unwrap())?;
                    c_file = c.get_class_file();
                }
                
            }
            // TODO: Search Superinterfaces of c.
            if !found {
                return Err(Error::NoSuchMethodError(Opcode::MethodInvoke));
            }
        }
        let method = method_to_call.unwrap();
        if (method.access_flags.flags & flags::method::ACC_STATIC) == 0 {
            return Err(Error::IllegalMethodType(Opcode::INVOKESTATIC));
        } 
        if (method.access_flags.flags & flags::method::ACC_ABSTRACT) > 0 {
            return Err(Error::IllegalMethodType(Opcode::INVOKESTATIC));
        } 
        if (method.access_flags.flags & flags::method::ACC_SYNCHRONIZED) > 0 {
            // TODO: Enter monitors on Classes.
        } 
        let _was_native = c.exec_method(jvm, &method)?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvokeInterface {
    _index: u16,
}
impl Instruction for InvokeInterface {
    fn name(&self) -> &'static str {
        "invokeinterface"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(InvokeInterface { _index: index })
        }
    }
    fn execute(&mut self, _jvm : &mut JVM) -> Result<(), Error> {
        Err(Error::Todo(Opcode::INVOKEINTERFACE))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvokeDynamic {
    index: u16,
}
impl Instruction for InvokeDynamic {
    fn name(&self) -> &'static str {
        "invokedynamic"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            v.remove(0); v.remove(0); // Skip required zeros
            Ok(InvokeDynamic { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let rt_file = frame.rt_const_pool.get_class_file();
        // Resolve the index into an instance of java.lang.invoke.CallSite.
        // This procedure also occurs in ldc, so this should be a subroutine.
        let invoke_dynamic_info = rt_file.cp_entry(self.index)?.as_invoke_dynamic()?;
        let bootstrap_index = invoke_dynamic_info.bootstrap_method_attr_index;
        let bootstrap_methods = match &rt_file.bootstrap_methods {
            Some(m) => m,
            None => return Err(Error::MissingBootstrapTable(Opcode::INVOKEDYNAMIC)),
        };
        let bootstrap_method = &bootstrap_methods[bootstrap_index as usize];
        let _method_handle = frame.rt_const_pool.get_class_file().cp_entry(bootstrap_method.bootstrap_method_ref)?.as_method_handle()?;
        // Resolve though https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.6 and 
        // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.5, a reference to an instance of 
        // java.lang.invoke.CallSite.
        /* 
        match method_handle.ref_kind {
            ReferenceKind::RefGetField => ,
            ReferenceKind::RefGetStatic => ,
            ReferenceKind::RefPutField => ,
            ReferenceKind::RefPutStatic => ,
            ReferenceKind::RefInvokeVirtual => ,
            ReferenceKind::RefInvokeStatic => ,
            ReferenceKind::RefInvokeSpecial => ,
            ReferenceKind::RefNewInvokeSpecial => ,
            ReferenceKind::RefInvokeInterface => ,
        }
        */
        // Then, use the process in invokevirtual to invoke a method where
        // The class in which the method is to be found is java.lang.invoke.MethodHandle,
        // The name of the method is invokeExact,
        // The descriptor is the descriptor in the reference,
        // and the stack looks like: reference to the method handle in the invoke.CallSite [nargs].
        Err(Error::Todo(Opcode::INVOKEDYNAMIC))

    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct New {
    index: u16,
}
impl Instruction for New {
    fn name(&self) -> &'static str {
        "new"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(New { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let current_class = frame.rt_const_pool.clone();      
        let objectref = Reference::new_object(current_class, self.index, jvm)?;
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Reference(objectref));
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NewArray {
    atype: u8,
}
impl Instruction for NewArray {
    fn name(&self) -> &'static str {
        "newarray"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let atype = v[0];
            v.remove(0);
            Ok(NewArray { atype })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let count = match frame.op_stack.pop() {
            Some(v) => *v.as_int()?,
            None => return Err(Error::StackUnderflow(Opcode::NEWARRAY)),
        };
        if count < 0 {
            return Err(Error::NegativeArraySizeException(Opcode::NEWARRAY));
        }
        let arrayref = Reference::new_array(count as usize, self.atype);
        let arrayval = Value::Reference(arrayref);
        frame.op_stack.push(arrayval);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ANewArray {
    index: u16,
}
impl Instruction for ANewArray {
    fn name(&self) -> &'static str {
        "anewarray"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(ANewArray { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let rt_file = frame.rt_const_pool.get_class_file();
        let count = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::PUTFIELD)),
        };
        let descriptor = {
            let class_ref = rt_file.cp_entry(self.index)?.as_class()?;
            let class_name = rt_file.cp_entry(*class_ref)?.as_utf8()?;
            // We actually just need the descriptor.
            // The Array descriptor represents the descriptor for the array, not for the component type, and so we need to add an extra '['.
            format!("[{}", class_name.clone())          
        };
        if *count.as_int()? < 0 {
            return Err(Error::NegativeArraySizeException(Opcode::ANEWARRAY));
        }
        let array = Reference::Array(Rc::new(Array::new_ref(*count.as_int()? as usize, descriptor)), 
        Rc::new(Monitor::new()));
        frame.op_stack.push(Value::Reference(array));
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayLength {}
impl Instruction for ArrayLength {
    fn name(&self) -> &'static str {
        "arraylength"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(ArrayLength {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let array_val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ARRAYLENGTH)),
        };
        let array_ref = array_val.as_reference()?;
        let array = match array_ref {
            Reference::Array(a, _) => a,
            Reference::Null => return Err(Error::NullPointerException(Opcode::ARRAYLENGTH)),
            _ => return Err(Error::IncorrectReferenceType(Opcode::ARRAYLENGTH)),
        };
        let size = array.len();
        let size_val = Value::Int(size as i32);
        frame.op_stack.push(size_val);
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AThrow {}
impl Instruction for AThrow {
    fn name(&self) -> &'static str {
        "athrow"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(AThrow {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let (should_init, null_class) = {
            let thread = access_macros::current_thread_mut!(jvm);
            let frame = access_macros::current_frame_mut!(thread);
            let len = frame.op_stack.len();
            if len == 0 {
                return Err(Error::StackUnderflow(Opcode::ATHROW));
            }
            let objectref = match &frame.op_stack[len - 1] {
                Value::Reference(r) => r.clone(),
                _ => return Err(Error::UnexpectedTypeOnStack(Opcode::ATHROW)),
            };
            
                        
            match objectref {
                Reference::Array(_, _) | Reference::Interface(_, _) => return Err(Error::IncorrectReferenceType(Opcode::ATHROW)),
                Reference::Null => {
                    // We could pop the other exception here, but we already clear it in self.handle_exception, so there's not really a point.
                    let exception = object::new_object_with_name("java/Lang/NullPointerException", jvm)?;
                    let exception_class = exception.class().clone();
                    let exception_ref = Reference::Object(exception, Rc::new(Monitor::new()));
                    let exception_val = Value::Reference(exception_ref);
                    let thread = access_macros::current_thread_mut!(jvm);
                    let frame = access_macros::current_frame_mut!(thread);
                    frame.op_stack.push(exception_val);
                    (true, Some(exception_class))
                },
                Reference::Object(_, _) => (false, None),
            }
        };
        if should_init {
            jvm.setup_method_call_from_name("<init>", "()V", null_class.unwrap(), false)?;
            jvm.run_until_method_exit();
        }
        Err(Error::Exception)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CheckCast {
    index: u16,
}
impl Instruction for CheckCast {
    fn name(&self) -> &'static str {
        "checkcast"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(CheckCast { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let op_stack_len = frame.op_stack.len();
        let objectref = &frame.op_stack[op_stack_len - 1].as_reference()?;
        let name: String;
        let object_desc = match objectref {
            Reference::Array(a, _) => a.descriptor(),
            Reference::Interface(i, _) => {
                name = format!("L{}", i.get_class_file().name());
                name.as_str()
            },
            Reference::Object(o, _) => {
                let c = o.class().clone();
                name = format!("L{}", c.get_class_file().name());
                name.as_str()
            }
            Reference::Null => return Ok(()),
        };
        let class = Rc::clone(&frame.rt_const_pool);
        let class_file = class.get_class_file();
        let class_reference = *class_file.cp_entry(self.index)?.as_class()?;
        let class_desc = class_file.cp_entry(class_reference)?.as_utf8()?.as_str();
        
        
        if jvm.check_class(object_desc, class_desc)? {
            return Ok(());
        }
        Err(Error::ClassCastException(Opcode::CHECKCAST))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstanceOf {
    index: u16,
}
impl Instruction for InstanceOf {
    fn name(&self) -> &'static str {
        "instanceof"
    }
    fn new(v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            let index = unsafe {
                u16::from_be_bytes(std::slice::from_raw_parts(v.as_ptr(), 2).try_into().unwrap()) 
            };
            v.remove(0); v.remove(0);
            Ok(InstanceOf { index })
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let objectref = match frame.op_stack.pop() {
            Some(v) => v.as_reference()?,
            None => return Err(Error::StackUnderflow(Opcode::INSTANCEOF)),
        };
        let name: String;
        let possible_array_class;
        let object_desc = match objectref {
            Reference::Array(a, _) => {
                possible_array_class = Some(a);
                possible_array_class.as_ref().unwrap().descriptor()
            },
            Reference::Interface(i, _) => {
                name = format!("L{}", i.get_class_file().name());
                name.as_str()
            },
            Reference::Object(o, _) => {
                let c = o.class().clone();
                name = format!("L{}", c.get_class_file().name());
                name.as_str()
            }
            Reference::Null => {
                frame.op_stack.push(Value::Int(0));
                return Ok(());
            },
        };
        let class = Rc::clone(&frame.rt_const_pool);
        let class_file = class.get_class_file();
        let class_reference = *class_file.cp_entry(self.index)?.as_class()?;
        let class_desc = class_file.cp_entry(class_reference)?.as_utf8()?.as_str();
        if jvm.check_class(object_desc, class_desc)? {
            let thread = access_macros::current_thread_mut!(jvm);
            let frame = access_macros::current_frame_mut!(thread);
            frame.op_stack.push(Value::Int(0));
            return Ok(());
        }
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(1));
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MonitorEnter {}
impl Instruction for MonitorEnter {
    fn name(&self) -> &'static str {
        "monitorenter"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(MonitorEnter {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let current_thread_number = jvm.m_thread_index;
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let object = match frame.op_stack.pop() {
            Some(o) => o.as_reference()?,
            None => return Err(Error::StackUnderflow(Opcode::MONITORENTER)),
        };
        match object {
            Reference::Null => return Err(Error::NullPointerException(Opcode::MONITORENTER)),
            Reference::Array(_, mut mrc) | Reference::Interface(_, mut mrc) | Reference::Object(_, mut mrc) => {
                let m = match Rc::get_mut(&mut mrc) {
                    Some(m) => m,
                    None => return Err(Error::DoubleMutableReferenceToMonitor(Opcode::MONITORENTER)),
                };
                let success = m.try_enter(current_thread_number);
                if !success {
                    thread.current_monitor = Some(mrc.clone());
                }
            },
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MonitorExit {}
impl Instruction for MonitorExit {
    fn name(&self) -> &'static str {
        "monitorexit"
    }
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized {
        if was_wide {
            Err(Error::IllegalWide)
        } else {
            Ok(MonitorExit {})
        }
    }
    fn execute(&mut self, jvm : &mut JVM) -> Result<(), Error> {
        let current_thread_number = jvm.m_thread_index;
        let thread = access_macros::current_thread_mut!(jvm);
        let frame = access_macros::current_frame_mut!(thread);
        let object = match frame.op_stack.pop() {
            Some(o) => o.as_reference()?,
            None => return Err(Error::StackUnderflow(Opcode::MONITOREXIT)),
        };
        match object {
            Reference::Null => return Err(Error::NullPointerException(Opcode::MONITOREXIT)),
            Reference::Array(_, mut mrc) | Reference::Interface(_, mut mrc) | Reference::Object(_, mut mrc) => {
                let m = match Rc::get_mut(&mut mrc) {
                    Some(m) => m,
                    None => return Err(Error::DoubleMutableReferenceToMonitor(Opcode::MONITORENTER)),
                };
                let success = m.try_exit(current_thread_number);
                if !success {
                    return Err(Error::IllegalMonitorStateException(Opcode::MONITOREXIT));
                }
            },
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eq(&self, other: &dyn Instruction) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }
}