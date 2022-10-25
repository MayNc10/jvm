use std::cell::Ref;
use std::result::Result;
use std::rc::Rc;

use crate::class::{Class, self};
use crate::constant_pool::{Entry, ReferenceKind};
use crate::errorcodes::{Error, Opcode};
use crate::reference::{Reference, Monitor, object};
use crate::reference::array::Array;
use crate::reference::object::Object;
use crate::value::{Value, VarValue};

use super::JVM;
use crate::{access_macros, flags};

// TODO: Rework actual JVM Exceptions to push an object instead of just returning an error.
// TODO: Use op_stack.last() instead of indexing. 

impl JVM {
    // Constants
    pub fn nop(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        Ok(())
    }
    pub fn aconst_null(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Reference(Reference::Null));
        Ok(())
    }
    pub fn iconst_m1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Int(-1));
        Ok(())
    }
    pub fn iconst_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Int(0));
        Ok(())
    }
    pub fn iconst_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Int(1));
        Ok(())
    }
    pub fn iconst_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Int(2));
        Ok(())
    }
    pub fn iconst_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Int(3));
        Ok(())
    }
    pub fn iconst_4(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Int(4));
        Ok(())
    }    
    pub fn iconst_5(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Int(5));
        Ok(())
    }
    pub fn lconst_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Long(0));
        Ok(())
    }
    pub fn lconst_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Long(1));
        Ok(())
    }
    pub fn fconst_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Float(0.0));
        Ok(())
    }
    pub fn fconst_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Float(1.0));
        Ok(())
    }
    pub fn fconst_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Float(2.0));
        Ok(())
    }
    pub fn dconst_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Double(0.0));
        Ok(())
    }
    pub fn dconst_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);

        frame.op_stack.push(Value::Double(1.0));
        Ok(())
    }
    pub fn bipush(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(2)?;
        let frame = access_macros::current_frame_mut!(thread);
        let byte = Value::Byte(frame.current_method.code()?[pc] as i32);
        frame.op_stack.push(byte);
        Ok(())
    }
    pub fn sipush(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);

        let short = Value::Short((frame.current_method.code()?[pc] as i32) << 8 + 
        frame.current_method.code()?[pc + 1] as i32);
        frame.op_stack.push(short);
        Ok(())
    }
    pub fn ldc(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(2)?;
        let frame = access_macros::current_frame_mut!(thread);
        // First, get the constant pool entry at that index.
        let entry = frame.rt_const_pool.get_class_file().cp_entry(frame.current_method.code()?[pc] as u16)?;
        match entry {
            Entry::Integer(i) => frame.op_stack.push(Value::Int(*i)),
            Entry::Float(f) => frame.op_stack.push(Value::Float(*f)),
            Entry::String(s) => {
                // We have to resolve a reference to the internal java class string.
                // The Java spec says that if we have already created an instance of String with the same base string, we cant just get a reference to it.
                // We aren't going to do this right now, but we should implement it in the future.
                // Instead, we are taking the path of creating a new String instance.
                // First, we create the new String.
                let current_class = Rc::clone(&frame.rt_const_pool);
                let s_raw = current_class.get_class_file().cp_entry(*s)?.as_utf8()?; 
                let s_obj = object::new_object("java/lang/String", self)?;
                let string_class = Rc::clone(&s_obj.class());
                let s_ref = Reference::Object(s_obj.clone(), Rc::new(Monitor::new()));             
                let carray = Array::Char(s_raw.encode_utf16().collect());
                let carray_len = carray.len();
                let carray = Rc::new(carray);
                let carray_asref = Reference::Array(carray.clone(), Rc::new(Monitor::new()));
                let thread = access_macros::current_thread_mut!(self);
                let frame = access_macros::current_frame_mut!(thread);
                frame.op_stack.push(Value::Reference(s_ref));
                frame.op_stack.push(Value::Reference(s_ref)); // duplicate the value, so we still have one afterwards.
                frame.op_stack.push(Value::Reference(carray_asref));
                frame.op_stack.push(Value::Int(0));
                frame.op_stack.push(Value::Int(carray_len as i32));
                self.setup_method_call_from_name("<init>", "([CII)V", Rc::clone(&string_class) , false)?;
                self.run_until_method_exit();
                // Lastly, we have to call .intern() on it.
                // String <init> calls this already, so I don't think we have to call it.
                //self.execute_native_from_name("intern", "()Ljava/lang/String;", string_class)?;
            },
            Entry::Class(c) => {
                // The spec says we have to return a reference to the class or interface itself. 
                // I think it means that we have to create a java.lang.Class object and return a reference.
                // For the same reasons as string above, we are not implementing this right now.
                return Err(Error::Todo(Opcode::LDC));
            },
            // For these next 2, see https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.5
            Entry::MethodType(m) => {
                // This one is a reference to java.lang.invoke.MethodType.
                // See above.
                return Err(Error::Todo(Opcode::LDC));
            },
            Entry::MethodHandle(m) => {
                // This one is incredibly complicated, but should result in a java.lang.invoke.MethodHandle.
                return Err(Error::Todo(Opcode::LDC));
            },
            Entry::Dynamic(dynamic) => {
                // This Dynamic cannot reference a field with discriptor Long or Double.

                // For more information about the process see: https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.6
                // First, we have to find a 'bootstrap method' to call to produce the value.
                // This is done by indexing into the BootstrapMethods Attribute of the current class.
                // This gives us a methodhandle info and a list of static arguments (loadable entries in the constant pool).
                // The method handle is resolved in the same way as above, except that the first parameter of the method must be java.lang.invoke.MethodHandles.Lookup.
                // If it's not, we fail with a BootstrapMethodError.
                // We alse get a field descriptor, from which we create a java.lang.Class object from it.
                // We then resolve every static argument. This process can be recursive, so we should make it a function.
                // Second, we have to call the bootstrap method.
                // To do this, we first create an Array of Object with length n + 3, where n is the number of static arguments.
                // The 0[] index is a reference an instance of java.lang.invoke.MethodHandles.Lookup for the current class.
                // The 1[] index is an reference to an instance of java.lang.String denoting the unqualified name from the name and type info.
                // The 2[] index is the reference to and instance of Class obtained earlier.
                // The rest of the array is filled with the static arguments.
                // The method handle is invoked with BMH.invokeWithArguments(args).
                // Finally, we have to validate the reference produced by the invocation.
                // the reference o is converted as if by invoking MH.invoke(o), 
                // where MH is a method handler produced from invoking the identity(class Object) method of java.lang.invoke.MethodHandles.
                // If this gives a NullPtrException or a ClassCastExeption, we fail with a BootstrapMethodError.
                return Err(Error::Todo(Opcode::LDC));
            },
            Entry::Long(_) | Entry::Double(_) => {
                // Even though these are loadable, they shouldn't appear here
                return Err(Error::IllegalConstantLoad(Opcode::LDCW));
            },
            _ => {
                return Err(Error::IllegalConstantLoad(Opcode::LDCW));
            },


        }
        Ok(())
    }
    pub fn ldc_w(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);

        let entry = frame.rt_const_pool.get_class_file().cp_entry(frame.current_method.code()?[pc] as u16 + 
                                                                frame.current_method.code()?[pc + 1] as u16)?;
        match entry {
            Entry::Integer(i) => frame.op_stack.push(Value::Int(*i)),
            Entry::Float(f) => frame.op_stack.push(Value::Float(*f)),
            Entry::String(s) => {
                // We have to resolve a reference to the internal java class string.
                // The Java spec says that if we have already created an instance of String with the same base string, we cant just get a reference to it.
                // We aren't going to do this right now, but we should implement it in the future.
                // Instead, we are taking the path of creating a new String instance.
                // For right now, we aren't implemting any of it, just because it requires a detour into how String works
                // (e.g. how do we create one from a constant string, how do we call .intern on it, ...)
                return Err(Error::Todo(Opcode::LDCW));
            },
            Entry::Class(c) => {
                // The spec says we have to return a reference to the class or interface itself. 
                // I think it means that we have to create a java.lang.Class object and return a reference.
                // For the same reasons as string above, we are not implementing this right now.
                return Err(Error::Todo(Opcode::LDCW));
            },
            // For these next 2, see https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.5
            Entry::MethodType(m) => {
                // This one is a reference to java.lang.invoke.MethodType.
                // See above.
                return Err(Error::Todo(Opcode::LDCW));
            },
            Entry::MethodHandle(m) => {
                // This one is incredibly complicated, but should result in a java.lang.invoke.MethodHandle.
                return Err(Error::Todo(Opcode::LDCW));
            },
            Entry::Dynamic(dynamic) => {
                // This Dynamic cannot reference a field with discriptor Long or Double.

                // For more information about the process see: https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.6
                // First, we have to find a 'bootstrap method' to call to produce the value.
                // This is done by indexing into the BootstrapMethods Attribute of the current class.
                // This gives us a methodhandle info and a list of static arguments (loadable entries in the constant pool).
                // The method handle is resolved in the same way as above, except that the first parameter of the method must be java.lang.invoke.MethodHandles.Lookup.
                // If it's not, we fail with a BootstrapMethodError.
                // We alse get a field descriptor, from which we create a java.lang.Class object from it.
                // We then resolve every static argument. This process can be recursive, so we should make it a function.
                // Second, we have to call the bootstrap method.
                // To do this, we first create an Array of Object with length n + 3, where n is the number of static arguments.
                // The 0[] index is a reference an instance of java.lang.invoke.MethodHandles.Lookup for the current class.
                // The 1[] index is an reference to an instance of java.lang.String denoting the unqualified name from the name and type info.
                // The 2[] index is the reference to and instance of Class obtained earlier.
                // The rest of the array is filled with the static arguments.
                // The method handle is invoked with BMH.invokeWithArguments(args).
                // Finally, we have to validate the reference produced by the invocation.
                // the reference o is converted as if by invoking MH.invoke(o), 
                // where MH is a method handler produced from invoking the identity(class Object) method of java.lang.invoke.MethodHandles.
                // If this gives a NullPtrException or a ClassCastExeption, we fail with a BootstrapMethodError.
                return Err(Error::Todo(Opcode::LDCW));
            },
            Entry::Long(_) | Entry::Double(_) => {
                // Even though these are loadable, they shouldn't appear here
                return Err(Error::IllegalConstantLoad(Opcode::LDCW));
            },
            _ => {
                return Err(Error::IllegalConstantLoad(Opcode::LDCW));
            },


        }
        Ok(())
    }
    pub fn ldc2_w(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);

        let entry = frame.rt_const_pool.get_class_file().cp_entry(frame.current_method.code()?[pc] as u16 + 
                                                                frame.current_method.code()?[pc + 1] as u16 )?;
        match entry {
            Entry::Double(d) => {
                frame.op_stack.push(Value::Double(*d));
            },
            Entry::Long(l) => {
                frame.op_stack.push(Value::Long(*l));
            },
            Entry::Dynamic(dynamic) => {
                // like ldc_w dynamic, except it can only load references to longs or doubles
                return Err(Error::Todo(Opcode::LDC2W));
            },
            Entry::Integer(_) | Entry::Float(_) | Entry::String(_) | Entry::Class(_) | 
            Entry::MethodHandle(_) | Entry::MethodType(_) => return Err(Error::IllegalConstantLoad(Opcode::LDC2W)),
            _ => return Err(Error::IllegalConstantLoad(Opcode::LDC2W)),
        }
        Ok(())
    }
    // Loads
    pub fn iload(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let offset = if wide {2} else {1};
        thread.inc_pc(1 + offset)?;
        let pc = (thread.pc() as isize - offset) as usize;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => {
                frame.current_method.code()?[pc] as u32
            },
            true => {
                ((frame.current_method.code()?[pc] as u32) << 8) + frame.current_method.code()?[pc + 1] as u32
            },
        };
        let var = &frame.local_variables[index as usize];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
    pub fn lload(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let offset = if wide {2} else {1};
        thread.inc_pc(1 + offset)?;
        let pc = (thread.pc() as isize - offset) as usize;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => {
                frame.current_method.code()?[pc] as u32
            },
            true => {
                ((frame.current_method.code()?[pc] as u32) << 8) + frame.current_method.code()?[pc + 1] as u32
            },
        };
        let var = &frame.local_variables[index as usize];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
    pub fn fload(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let offset = if wide {2} else {1};
        thread.inc_pc(1 + offset)?;
        let pc = (thread.pc() as isize - offset) as usize;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => {
                frame.current_method.code()?[pc] as u32
            },
            true => {
                ((frame.current_method.code()?[pc] as u32) << 8) + frame.current_method.code()?[pc + 1] as u32
            },
        };
        let var = &frame.local_variables[index as usize];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
    pub fn dload(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let offset = if wide {2} else {1};
        thread.inc_pc(1 + offset)?;
        let pc = (thread.pc() as isize - offset) as usize;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => {
                frame.current_method.code()?[pc] as u32
            },
            true => {
                ((frame.current_method.code()?[pc] as u32) << 8) + frame.current_method.code()?[pc + 1] as u32
            },
        };
        let var = &frame.local_variables[index as usize];
        frame.op_stack.push(Value::Double(*var.as_double()?));
        Ok(())
    }
    pub fn aload(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let offset = if wide {2} else {1};
        thread.inc_pc(1 + offset)?;
        let pc = (thread.pc() as isize - offset) as usize;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => {
                frame.current_method.code()?[pc] as u32
            },
            true => {
                ((frame.current_method.code()?[pc] as u32) << 8) + frame.current_method.code()?[pc + 1] as u32
            },
        };
        let var = &frame.local_variables[index as usize];
        frame.op_stack.push(Value::Reference(var.as_reference()?.clone()));
        Ok(())
    }
    pub fn iload_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
    pub fn iload_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
    pub fn iload_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
    pub fn iload_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Int(*var.as_int()?));
        Ok(())
    }
    pub fn lload_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
    pub fn lload_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
    pub fn lload_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
    pub fn lload_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Long(*var.as_long()?));
        Ok(())
    }
    pub fn fload_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
    pub fn fload_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
    pub fn fload_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
    pub fn fload_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Float(*var.as_float()?));
        Ok(())
    }
    pub fn dload_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Double(*var.as_double()?));
        Ok(())
    }
    pub fn dload_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Double(*var.as_double()?));
        Ok(())
    }
    pub fn dload_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Double(*var.as_double()?));
        Ok(())
    }
    pub fn dload_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Double(*var.as_double()?));
        Ok(())
    }
    pub fn aload_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[0];
        frame.op_stack.push(Value::Reference(var.as_reference()?.clone()));
        Ok(())
    }
    pub fn aload_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[1];
        frame.op_stack.push(Value::Reference(var.as_reference()?.clone()));
        Ok(())
    }
    pub fn aload_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[2];
        frame.op_stack.push(Value::Reference(var.as_reference()?.clone()));
        Ok(())
    }
    pub fn aload_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let var = &frame.local_variables[3];
        frame.op_stack.push(Value::Reference(var.as_reference()?));
        Ok(())
    }
    pub fn iaload(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::IALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::IALOAD)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::IALOAD)),
        };
        if !arr.is_iarray() {
            return Err(Error::IncorrectReferenceType(Opcode::IALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
    pub fn laload(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::LALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::LALOAD)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::LALOAD)),
        };
        if !arr.is_larray() {
            return Err(Error::IncorrectReferenceType(Opcode::LALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }    
    pub fn faload(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::FALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::FALOAD)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::FALOAD)),
        };
        if !arr.is_farray() {
            return Err(Error::IncorrectReferenceType(Opcode::FALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
    pub fn daload(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::DALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::DALOAD)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::DALOAD)),
        };
        if !arr.is_darray() {
            return Err(Error::IncorrectReferenceType(Opcode::DALOAD));
        }  
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
    pub fn aaload(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::AALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::AALOAD)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::AALOAD)),
        };
        if !arr.is_refarray() {
            return Err(Error::IncorrectReferenceType(Opcode::AALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
    pub fn baload(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::BALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::BALOAD)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::BALOAD)),
        };
        if !arr.is_barray() | !arr.is_boolarray() {
            return Err(Error::IncorrectReferenceType(Opcode::BALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
    pub fn caload(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::CALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::CALOAD)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::CALOAD)),
        };
        if !arr.is_carray() {
            return Err(Error::IncorrectReferenceType(Opcode::CALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
    pub fn saload(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index_val = match frame.op_stack.pop() {
            Some(val) => val,
            None => return Err(Error::StackUnderflow(Opcode::SALOAD)),
        };
        let index = index_val.as_int()?;
        let mut arrayref = match frame.op_stack.pop() {
            Some(a) => a,
            None => return Err(Error::StackUnderflow(Opcode::SALOAD)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let arr = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::SALOAD)),
        };
        if !arr.is_sarray() {
            return Err(Error::IncorrectReferenceType(Opcode::SALOAD));
        }
        frame.op_stack.push(arr.get(*index as usize));
        Ok(())
    }
    // Stores
    pub fn istore(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(2 + (wide as isize))?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => frame.current_method.code()?[pc] as usize,
            true => {
                ((frame.current_method.code()?[pc] as usize) << 8) | frame.current_method.code()?[pc + 1] as usize
            },
        };
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), index);
        Ok(())
    }
    pub fn lstore(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(2 + (wide as isize))?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => frame.current_method.code()?[pc] as usize,
            true => {
                ((frame.current_method.code()?[pc] as usize) << 8) | frame.current_method.code()?[pc + 1] as usize
            },
        };
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), index);
        Ok(())
    }
    pub fn fstore(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(2 + (wide as isize))?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => frame.current_method.code()?[pc] as usize,
            true => {
                ((frame.current_method.code()?[pc] as usize) << 8) | frame.current_method.code()?[pc + 1] as usize
            },
        };
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), index);
        Ok(())
    }
    pub fn dstore(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(2 + (wide as isize))?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => frame.current_method.code()?[pc] as usize,
            true => {
                ((frame.current_method.code()?[pc] as usize) << 8) | frame.current_method.code()?[pc + 1] as usize
            },
        };
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), index);
        Ok(())
    }
    pub fn astore(&mut self, wide: bool) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(2 + (wide as isize))?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = match wide {
            false => frame.current_method.code()?[pc] as usize,
            true => {
                ((frame.current_method.code()?[pc] as usize) << 8) | frame.current_method.code()?[pc + 1] as usize
            },
        };
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), index);
        Ok(())
    }
    pub fn istore_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE0)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), 0);
        Ok(())
    }
    pub fn istore_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE1)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), 1);
        Ok(())
    }
    pub fn istore_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE2)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), 2);
        Ok(())
    }
    pub fn istore_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISTORE3)),
        };
        frame.insert_local(VarValue::Int(*val.as_int()?), 3);
        Ok(())
    }
    pub fn lstore_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE0)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), 0);
        Ok(())
    }
    pub fn lstore_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE1)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), 1);
        Ok(())
    }
    pub fn lstore_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE2)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), 2);
        Ok(())
    }
    pub fn lstore_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSTORE3)),
        };
        frame.insert_local(VarValue::Long(*val.as_long()?), 3);
        Ok(())
    }
    pub fn fstore_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE0)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), 0);
        Ok(())
    }
    pub fn fstore_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE1)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), 1);
        Ok(())
    }
    pub fn fstore_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE2)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), 2);
        Ok(())
    }
    pub fn fstore_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSTORE3)),
        };
        frame.insert_local(VarValue::Float(*val.as_float()?), 3);
        Ok(())
    }
    pub fn dstore_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE0)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), 0);
        Ok(())
    }
    pub fn dstore_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE1)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), 1);
        Ok(())
    }
    pub fn dstore_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE2)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), 2);
        Ok(())
    }
    pub fn dstore_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSTORE3)),
        };
        frame.insert_local(VarValue::Double(*val.as_double()?), 3);
        Ok(())
    }
    pub fn astore_0(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE0)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), 0);
        Ok(())
    }
    pub fn astore_1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE1)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), 1);
        Ok(())
    }
    pub fn astore_2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE2)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), 2);
        Ok(())
    }
    pub fn astore_3(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ASTORE3)),
        };
        frame.insert_local(VarValue::Reference(val.as_reference()?.clone()), 3);
        Ok(())
    }
    pub fn iastore(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IASTORE)),
        };
        if !val.is_int() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::IASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IASTORE)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::IASTORE)),
        };
        if !array.is_iarray() {
            return Err(Error::IncorrectReferenceType(Opcode::IASTORE));
        }
        array.set(*index.as_int()? as usize, val)?;
        Ok(())
    }
    pub fn lastore(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LASTORE)),
        };
        if !val.is_long() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::LASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LASTORE)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::LASTORE)),
        };
        if !array.is_larray() {
            return Err(Error::IncorrectReferenceType(Opcode::LASTORE));
        }
        array.set(*index.as_int()? as usize, val)?;
        Ok(())
    }    
    pub fn fastore(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FASTORE)),
        };
        if !val.is_float() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::FASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FASTORE)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::FASTORE)),
        };
        if !array.is_farray() {
            return Err(Error::IncorrectReferenceType(Opcode::FASTORE));
        }
        array.set(*index.as_int()? as usize, val)?;
        Ok(())
    }
    pub fn dastore(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DASTORE)),
        };
        if !val.is_double() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::DASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DASTORE)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::DASTORE)),
        };
        if !array.is_darray() {
            return Err(Error::IncorrectReferenceType(Opcode::DASTORE));
        }
        array.set(*index.as_int()? as usize, val)?;
        Ok(())
    }
    // This function needs some work: https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-6.html#jvms-6.5.aastore
    pub fn aastore(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::AASTORE)),
        };
        if !val.is_reference() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::AASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::AASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::AASTORE)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::IncorrectReferenceType(Opcode::AASTORE)),
        };
        if !array.is_refarray() {
            return Err(Error::IncorrectReferenceType(Opcode::AASTORE));
        }
        // Validate reference compatibility. 
        // Currently we can't do that, because we don't know what type of reference array refers to. 
        // FIXME: Store the array descriptor in the array type. 
        match val.as_reference()? {
            Reference::Null => (),
            Reference::Array(a, _) => {
            },
            Reference::Interface(i, _) => {
            },
            Reference::Object(o, _) => {
            },           
        }
        Ok(())
    }
    pub fn bastore(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::BASTORE)),
        };
        if !val.is_int() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::BASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::BASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::BASTORE)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::BASTORE)),
        };
        if !array.is_barray() {
            return Err(Error::IncorrectReferenceType(Opcode::BASTORE));
        }
        array.set(*index.as_int()? as usize, val)?;
        Ok(())
    }
    pub fn castore(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::CASTORE)),
        };
        if !val.is_int() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::CASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::CASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::CASTORE)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::CASTORE)),
        };
        if !array.is_carray() {
            return Err(Error::IncorrectReferenceType(Opcode::CASTORE));
        }
        array.set(*index.as_int()? as usize, val)?;
        Ok(())
    }
    pub fn sastore(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SASTORE)),
        };
        if !val.is_int() {
            return Err(Error::UnexpectedTypeOnStack(Opcode::SASTORE));
        }
        let index = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SASTORE)),
        };
        let mut arrayref = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SASTORE)),
        };
        let mut arrayref_rc = arrayref.as_reference_mut()?;
        let array = match arrayref_rc {
            Reference::Array(arr, _) => arr,
            _ => return Err(Error::UnexpectedTypeOnStack(Opcode::SASTORE)),
        };
        if !array.is_sarray() {
            return Err(Error::IncorrectReferenceType(Opcode::SASTORE));
        }
        array.set(*index.as_int()? as usize, val)?;
        Ok(())
    }
    // Stack

    // TODO: Write clearer error messages that explain the branches taken for the dup operations.
    pub fn pop(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        match frame.op_stack.pop() {
            Some(val) => {
                if val.is_comptype2() {
                    return Err(Error::IncorrectComputationalType(Opcode::POP));
                }
            },
            None => return Err(Error::StackUnderflow(Opcode::POP)),
        }
        Ok(())
    }    
    pub fn pop2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        match frame.op_stack.pop() {
            Some(val) => {
                if val.is_comptype2() {
                    return Ok(());
                }
                else {
                    match frame.op_stack.pop() {
                        Some(val) => {
                            if val.is_comptype2() {
                                return Err(Error::IncorrectComputationalType(Opcode::POP2));
                            }
                        },
                        None => return Err(Error::StackUnderflow(Opcode::POP2)),
                    }
                }
            },
            None => return Err(Error::StackUnderflow(Opcode::POP2)),
        }
        Ok(())
    }
    pub fn dup(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let top_index = frame.op_stack.len();
        if top_index == 0 {
            return Err(Error::StackUnderflow(Opcode::DUP));
        }
        let val = frame.op_stack[top_index - 1].clone();
        if val.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUP));
        }
        frame.op_stack.push(val);
        Ok(())
    }
    pub fn dup_x1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUPX1)),
        };
        if val.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUPX1));
        }
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUPX1)),
        };
        if val2.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUPX1));
        }
        frame.op_stack.push(val.clone());
        frame.op_stack.push(val2);
        frame.op_stack.push(val);
        Ok(())
    }
    pub fn dup_x2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUPX2)),
        };
        if val.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUPX2));
        }
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUPX2)),
        };
        if val2.is_comptype1() {
            let val3 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::DUPX2)),
            };
            if val3.is_comptype2() {
                return Err(Error::IncorrectComputationalType(Opcode::DUPX2));
            }
            frame.op_stack.push(val.clone());
            frame.op_stack.push(val3);
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
        else {
            frame.op_stack.push(val.clone());
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
        Ok(())
    }
    pub fn dup2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let top_index = frame.op_stack.len();
        if top_index == 0 {
            return Err(Error::StackUnderflow(Opcode::DUP2));
        }
        let val = frame.op_stack[top_index - 1].clone();
        if val.is_comptype1() {
            if top_index == 1 {
                return Err(Error::StackUnderflow(Opcode::DUP2));
            }
            let val2 = frame.op_stack[top_index - 2].clone();
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
        else {
            frame.op_stack.push(val);
        }
        Ok(())
    }
    pub fn dup2_x1(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUP2X1)),
        };
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUP2X1)),
        };
        if val2.is_comptype2() {
            return Err(Error::IncorrectComputationalType(Opcode::DUP2X1));
        }
        if val.is_comptype1() {
            let val3 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::IncorrectComputationalType(Opcode::DUP2X1)),
            };
            if val3.is_comptype2() {
                return Err(Error::IncorrectComputationalType(Opcode::DUP2X1));
            }
            frame.op_stack.push(val2.clone());
            frame.op_stack.push(val.clone());
            frame.op_stack.push(val3);
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
        else {
            frame.op_stack.push(val.clone());
            frame.op_stack.push(val2);
            frame.op_stack.push(val);
        }
        Ok(())
    }
    pub fn dup2_x2(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
        };
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
        };
        if val.is_comptype1() {
            if val2.is_comptype2() {
                return Err(Error::IncorrectComputationalType(Opcode::DUP2X2));
            }
            else {
                let val3 = match frame.op_stack.pop() {
                    Some(v) => v,
                    None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
                };
                if val3.is_comptype1() {
                    let val4 = match frame.op_stack.pop() {
                        Some(v) => v,
                        None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
                    };
                    if val4.is_comptype2() {
                        return Err(Error::IncorrectComputationalType(Opcode::DUP2X2));
                    }
                    frame.op_stack.push(val2.clone());
                    frame.op_stack.push(val.clone());
                    frame.op_stack.push(val4);
                    frame.op_stack.push(val3);
                    frame.op_stack.push(val2);
                    frame.op_stack.push(val);
                }
                else {
                    frame.op_stack.push(val2.clone());
                    frame.op_stack.push(val.clone());
                    frame.op_stack.push(val3);
                    frame.op_stack.push(val2);
                    frame.op_stack.push(val); 
                }

            }
        }   
        else {
            if val2.is_comptype1() {
                let val3 = match frame.op_stack.pop() {
                    Some(v) => v,
                    None => return Err(Error::StackUnderflow(Opcode::DUP2X2)),
                };
                if val3.is_comptype2() {
                    return Err(Error::IncorrectComputationalType(Opcode::DUP2X2));
                }
                frame.op_stack.push(val.clone());
                frame.op_stack.push(val3);
                frame.op_stack.push(val2);
                frame.op_stack.push(val);
            }
            else {
                frame.op_stack.push(val.clone());
                frame.op_stack.push(val2);
                frame.op_stack.push(val); 
            }
        }
        Ok(())
    }
    pub fn swap(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SWAP)),
        };
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::SWAP)),
        };
        frame.op_stack.push(val);
        frame.op_stack.push(val2);
        Ok(())
    }
    // Math
    // These should be checked to confirm that rust works the same way that java expects.
    pub fn iadd(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IADD)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IADD)),
        };
        let (result, _) = val1.as_int()?.overflowing_add(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn ladd(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LADD)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LADD)),
        };
        let (result, _) = val1.as_long()?.overflowing_add(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn fadd(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FADD)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FADD)),
        };
        let result = val1.as_float()? + val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    pub fn dadd(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DADD)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DADD)),
        };
        let result = val1.as_double()? + val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
    pub fn isub(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISUB)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISUB)),
        };
        let (result, _) = val1.as_int()?.overflowing_sub(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn lsub(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSUB)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSUB)),
        };
        let (result, _) = val1.as_long()?.overflowing_sub(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn fsub(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSUB)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FSUB)),
        };
        let result = val1.as_float()? - val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    pub fn dsub(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSUB)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DSUB)),
        };
        let result = val1.as_double()? - val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
    pub fn imul(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IMUL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IMUL)),
        };
        let (result, _) = val1.as_int()?.overflowing_mul(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn lmul(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LMUL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LMUL)),
        };
        let (result, _) = val1.as_long()?.overflowing_mul(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn fmul(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FMUL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FMUL)),
        };
        let result = val1.as_float()? * val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    pub fn dmul(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DMUL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DMUL)),
        };
        let result = val1.as_double()? * val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
    pub fn idiv(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IDIV)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IDIV)),
        };
        if *val2.as_int()? == 0 {
            return Err(Error::ArithmeticException(Opcode::IDIV));
        }
        let (result, _) = val1.as_int()?.overflowing_div(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn ldiv(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LDIV)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LDIV)),
        };
        if *val2.as_long()? == 0 {
            return Err(Error::ArithmeticException(Opcode::LDIV));
        }
        let (result, _) = val1.as_long()?.overflowing_div(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn fdiv(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FDIV)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FDIV)),
        };
        let result = val1.as_float()? / val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    pub fn ddiv(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DDIV)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DDIV)),
        };
        let result = val1.as_double()? / val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
    pub fn irem(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IREM)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IREM)),
        };
        if *val2.as_int()? == 0 {
            return Err(Error::ArithmeticException(Opcode::IREM));
        }
        let (result, _) = val1.as_int()?.overflowing_rem(*val2.as_int()?);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn lrem(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LREM)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LREM)),
        };
        if *val2.as_long()? == 0 {
            return Err(Error::ArithmeticException(Opcode::LREM));
        }
        let (result, _) = val1.as_long()?.overflowing_rem(*val2.as_long()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn frem(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FREM)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FREM)),
        };
        let result = val1.as_float()? % val2.as_float()?;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    pub fn drem(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DREM)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DREM)),
        };
        let result = val1.as_double()? % val2.as_double()?;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
    pub fn ineg(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::INEG)),
        };
        let (result, _) = val.as_int()?.overflowing_neg();
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn lneg(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LNEG)),
        };
        let (result, _) = val.as_long()?.overflowing_neg();
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn fneg(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FNEG)),
        };
        let result = val.as_float()? * -1.0;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    pub fn dneg(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DNEG)),
        };
        let result = val.as_double()? * -1.0;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
    pub fn ishl(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISHL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISHL)),
        };
        let result = val1.as_int()? << (val2.as_int()? & 0x1f);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn lshl(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSHL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSHL)),
        };
        let result = val1.as_long()? << (val2.as_long()? & 0x3f);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn ishr(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISHR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::ISHR)),
        };
        let result = val1.as_int()? >> (val2.as_int()? & 0x1f);
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn lshr(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSHR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LSHL)),
        };
        let result = val1.as_long()? >> (val2.as_long()? & 0x3f);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn iushr(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IUSHR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IUSHR)),
        };
        let mut result = val1.as_int()? >> (val2.as_int()? & 0x1f);
        if *val1.as_int()? < 0 {
            // Account for propagated sign bit.
            // See: https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-6.html#jvms-6.5.fadd
            result += 2 << !(val2.as_int()? & 0x1f);
        }
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn lushr(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LUSHR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LUSHR)),
        };
        let mut result = val1.as_long()? >> (val2.as_long()? & 0x3f);
        if *val1.as_long()? < 0 {
            // Account for propagated sign bit.
            // See: https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-6.html#jvms-6.5.fadd
            result += 2 << !(val2.as_long()? & 0x3f);
        }
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn iand(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IAND)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IAND)),
        };
        let result = val1.as_int()? & val2.as_int()?;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn land(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LAND)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LAND)),
        };
        let result = val1.as_long()? & val2.as_long()?;
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn ior(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IOR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IOR)),
        };
        let result = val1.as_int()? | val2.as_int()?;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }    
    pub fn lor(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LOR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LOR)),
        };
        let result = val1.as_long()? | val2.as_long()?;
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn ixor(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IXOR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::IXOR)),
        };
        let result = val1.as_int()? ^ val2.as_int()?;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn lxor(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LXOR)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LXOR)),
        };
        let result = val1.as_long()? ^ val2.as_long()?;
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn iinc(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = frame.current_method.code()?[pc];
        let const_incr = frame.current_method.code()?[pc + 1];
        let var = &mut frame.local_variables[index as usize];
        // Confirm that this updates the value.
        *var.as_int_mut()? += const_incr as i32;
        Ok(())
    }
    // Conversions
    pub fn i2l(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2L)),
        };
        let result = i64::from(*val.as_int()?);
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn i2f(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2F)),
        };
        let result = *val.as_int()? as f32;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    pub fn i2d(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2D)),
        };
        let result = *val.as_int()? as f64;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
    pub fn l2i(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::L2I)),
        };
        let result = (val.as_long()? & 0xffffffff) as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn l2f(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::L2F)),
        };
        let result = *val.as_long()? as f32;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    pub fn l2d(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::L2D)),
        };
        let result = *val.as_long()? as f64;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
    // The next two functions, f2i and f2l, use the round-to-zero rule, which is not the IEEE-754 default (as far as I'm aware).
    // These functions might not work properly, and so will have to be reworked.
    pub fn f2i(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::F2I)),
        };
        let result = *val.as_float()? as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn f2l(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::F2L)),
        };
        let result = *val.as_float()? as i64;
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn f2d(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::F2D)),
        };
        let result = *val.as_float()? as f64;
        frame.op_stack.push(Value::Double(result));
        Ok(())
    }
    pub fn d2i(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::D2I)),
        };
        let result = *val.as_double()? as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn d2l(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::D2L)),
        };
        let result = *val.as_double()? as i64;
        frame.op_stack.push(Value::Long(result));
        Ok(())
    }
    pub fn d2f(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::D2F)),
        };
        let result = *val.as_double()? as f32;
        frame.op_stack.push(Value::Float(result));
        Ok(())
    }
    pub fn i2b(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2B)),
        };
        let result = ((val.as_int()? & 0xff) as i8) as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn i2c(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2C)),
        };
        let result = ((val.as_int()? & 0xffff) as u16) as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn i2s(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::I2S)),
        };
        let result = ((val.as_int()? & 0xffff) as i16) as i32;
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    // Comparisons
    pub fn lcmp(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LCMP)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::LCMP)),
        };
        let result = match val1.as_long()? > val2.as_long()? {
            true => 1 as i32,
            false => match val1.as_long()? < val2.as_long()? {
                true => -1 as i32,
                false => 0 as i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn fcmpl(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FCMPL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FCMPL)),
        };
        // Because this is a float comparison, we have to deal with NaNs.
        // in this function, we group them together with the < case and return -1.
        let result = match val1.as_float()? > val2.as_float()? {
            true => 1 as i32,
            false => match val1.as_float()? == val2.as_float()? {
                true => 0 as i32,
                false => -1 as i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn fcmpg(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FCMPG)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::FCMPG)),
        };
        // Because this is a float comparison, we have to deal with NaNs.
        // in this function, we group them together with the > case and return 1.
        let result = match val1.as_float()? < val2.as_float()? {
            true => -1 as i32,
            false => match val1.as_float()? == val2.as_float()? {
                true => 0 as i32,
                false => 1 as i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn dcmpl(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DCMPL)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DCMPL)),
        };
        // Because this is a double comparison, we have to deal with NaNs.
        // in this function, we group them together with the < case and return -1.
        let result = match val1.as_double()? > val2.as_double()? {
            true => 1 as i32,
            false => match val1.as_double()? == val2.as_double()? {
                true => 0 as i32,
                false => -1 as i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn dcmpg(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let val2 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DCMPG)),
        };
        let val1 = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::DCMPG)),
        };
        // Because this is a double comparison, we have to deal with NaNs.
        // in this function, we group them together with the > case and return 1.
        let result = match val1.as_double()? < val2.as_double()? {
            true => -1 as i32,
            false => match val1.as_double()? == val2.as_double()? {
                true => 0 as i32,
                false => 1 as i32,
            }
        };
        frame.op_stack.push(Value::Int(result));
        Ok(())
    }
    pub fn ifeq(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFEQ)),
            };
            match *val.as_int()? == 0 {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn ifne(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFNE)),
            };
            match *val.as_int()? != 0 {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn iflt(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFLT)),
            };
            match *val.as_int()? < 0 {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn ifge(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFGE)),
            };
            match *val.as_int()? >= 0 {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn ifgt(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFGT)),
            };
            match *val.as_int()? > 0 {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn ifle(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFLE)),
            };
            match *val.as_int()? <= 0 {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn if_icmpeq(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPEQ)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPEQ)),
            };
            match val1.as_int()? == val2.as_int()? {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn if_icmpne(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPNE)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPNE)),
            };
            match val1.as_int()? != val2.as_int()? {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn if_icmplt(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPLT)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPLT)),
            };
            match val1.as_int()? < val2.as_int()? {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn if_icmpge(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPGE)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPGE)),
            };
            match val1.as_int()? >= val2.as_int()? {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn if_icmpgt(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPGT)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPGT)),
            };
            match val1.as_int()? > val2.as_int()? {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn if_icmple(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPLE)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFICMPLE)),
            };
            match val1.as_int()? <= val2.as_int()? {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn if_amcpeq(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFACMPEQ)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFACMPEQ)),
            };
            match Reference::ptr_eq(&val1.as_reference()?, &val2.as_reference()?) {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn if_amcpne(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let val2 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFACMPNE)),
            };
            let val1 = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::IFACMPNE)),
            };
            match val1.as_reference()? != val2.as_reference()? {
                true => {
                    let bbyte1 = frame.current_method.code()?[pc] as usize;
                    let bbyte2 = frame.current_method.code()?[pc + 1] as usize;
                    bbyte1 << 8 | bbyte2
                },
                false => 3,
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    // Control
    pub fn goto(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let boffset = {
            let frame = access_macros::current_frame_mut!(thread);
            let bbyte1 = frame.current_method.code()?[pc] as i16;
            let bbyte2 = frame.current_method.code()?[pc + 1] as i16;
            bbyte1 << 8 | bbyte2
        };
        thread.inc_pc(boffset as isize)?;
        Ok(())

    }
    pub fn jsr(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let boffset = {
            let frame = access_macros::current_frame_mut!(thread);
            frame.op_stack.push(Value::ReturnAddress(pc as u16 + 2));
            let bbyte1 = frame.current_method.code()?[pc] as i16;
            let bbyte2 = frame.current_method.code()?[pc + 1] as i16;
            bbyte1 << 8 | bbyte2
        };
        thread.inc_pc(boffset as isize)?;
        Ok(())
    }    
    pub fn ret(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let addr = {
            let frame = access_macros::current_frame_mut!(thread);
            let index = frame.current_method.code()?[pc] as usize;
            *frame.local_variables[index].as_retaddr()?
        };
        thread.set_pc(addr as usize)?;
        Ok(())
    }
    pub fn tableswitch(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let mut pc = thread.pc() + 1;
        pc += 4 - (pc % 4); // pc has to be a multiple of 4.
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let index = match frame.op_stack.pop() {
                Some(v) => *v.as_int()?,
                None => return Err(Error::StackUnderflow(Opcode::TABLESWITCH)),
            } as usize;
            let code = frame.current_method.code()?;
            let default = ((code[pc] as usize) << 24) | ((code[pc + 1] as usize) << 16) | ((code[pc + 2] as usize) << 8) | (code[pc + 3] as usize);
            let low = ((code[pc + 4] as usize) << 24) | ((code[pc + 5] as usize) << 16) | ((code[pc + 6] as usize) << 8) | (code[pc + 7] as usize);
            let high = ((code[pc + 8] as usize) << 24) | ((code[pc + 9] as usize) << 16) | ((code[pc + 10] as usize) << 8) | (code[pc + 11] as usize);
            let target = match (index < low) | (index > high) {
                true => index - low,
                false => default,
            };
            let true_pc = pc - 1; // Offset from this address.
            ((code[true_pc + target] as isize) << 24) | ((code[true_pc + target + 1] as isize) << 16) | ((code[true_pc + target + 2] as isize) << 8) | (code[true_pc + target + 3] as isize)
        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    pub fn lookupswitch(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let mut pc = thread.pc() + 1;
        pc += 4 - (pc % 4); // pc has to be a multiple of 4.
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let key = match frame.op_stack.pop() {
                Some(v) => *v.as_int()?,
                None => return Err(Error::StackUnderflow(Opcode::LOOKUPSWITCH)),
            };
            let code = frame.current_method.code()?;
            let default = ((code[pc] as isize) << 24) | ((code[pc + 1] as isize) << 16) | ((code[pc + 2] as isize) << 8) | (code[pc + 3] as isize);
            let npairs = ((code[pc + 4] as usize) << 24) | ((code[pc + 5] as usize) << 16) | ((code[pc + 6] as usize) << 8) | (code[pc + 7] as usize);
            // This is a very slow iteration. It can be improved using a binary search, because the match values are sorted.
            // This is simple though, so it's the current solution.
            let mut final_offset = default;
            for index in 0..=npairs {
                let pc_offset = (index * 8) + 8;
                let match_value = ((code[pc_offset] as i32) << 24) | ((code[pc_offset + 1] as i32) << 16) | ((code[pc_offset + 2] as i32) << 8) | (code[pc_offset + 3] as i32);
                if key == match_value {
                    final_offset = ((code[pc_offset + 4] as isize) << 24) | ((code[pc_offset + 5] as isize) << 16) | ((code[pc_offset + 6] as isize) << 8) | (code[pc_offset + 7] as isize);
                    break;
                }
            }
            final_offset

        };
        thread.inc_pc(offset)?;
        Ok(())
    }
    // TODO: Monitor things in the return functions.
    // TODO: Handle being at the end of the call stack.
    pub fn ireturn(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            if !frame.current_method.returns_int(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::IRETURN));
            }
            let ret_type = frame.current_method.return_char(&frame.rt_const_pool.get_class_file())?;
            let ret_val = match frame.op_stack.pop() {
                Some(v) => *v.as_int()?,
                None => return Err(Error::StackUnderflow(Opcode::IRETURN)),
            };
            match ret_type {
                'B' => Value::Byte((ret_val as i8) as i32),
                'C' => Value::Char((ret_val as u16) as i32),
                'I' => Value::Int(ret_val),
                'S' => Value::Short((ret_val as i16) as i32),
                'Z' => Value::Byte((ret_val == 0) as i32),
                _ => unreachable!(),
            }
        };
        let _ = match thread.m_stack.pop() {
            Some(_2) => _2,
            None => return Err(Error::FrameStackUnderflow(Opcode::IRETURN)),
        };
        // This should sync up with method calls, but if we forget to not add the pc there, we will skip an opcode and corrupt the machine.
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    pub fn lreturn(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            if !frame.current_method.returns_long(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::LRETURN));
            }
            match frame.op_stack.pop() {
                Some(v) => Value::Long(*v.as_long()?),
                None => return Err(Error::StackUnderflow(Opcode::LRETURN)),
            }          
        };
        let _ = match thread.m_stack.pop() {
            Some(_2) => _2,
            None => return Err(Error::FrameStackUnderflow(Opcode::LRETURN)),
        };
        // This should sync up with method calls, but if we forget to not add the pc there, we will skip an opcode and corrupt the machine.
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    pub fn freturn(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            if !frame.current_method.returns_float(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::FRETURN));
            }
            match frame.op_stack.pop() {
                Some(v) => Value::Float(*v.as_float()?),
                None => return Err(Error::StackUnderflow(Opcode::FRETURN)),
            }          
        };
        let _ = match thread.m_stack.pop() {
            Some(_2) => _2,
            None => return Err(Error::FrameStackUnderflow(Opcode::FRETURN)),
        };
        // This should sync up with method calls, but if we forget to not add the pc there, we will skip an opcode and corrupt the machine.
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    pub fn dreturn(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            if !frame.current_method.returns_double(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::DRETURN));
            }
            match frame.op_stack.pop() {
                Some(v) => Value::Double(*v.as_double()?),
                None => return Err(Error::StackUnderflow(Opcode::DRETURN)),
            }          
        };
        let _ = match thread.m_stack.pop() {
            Some(_2) => _2,
            None => return Err(Error::FrameStackUnderflow(Opcode::DRETURN)),
        };
        // This should sync up with method calls, but if we forget to not add the pc there, we will skip an opcode and corrupt the machine.
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    // This function is really rough. We need to make it more readable.
    // It's also pretty broken. We don't validate the component types of arrays and we don't check for narrowing conversions in classes and interfaces.
    pub fn areturn(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let return_value = {
            let frame = access_macros::current_frame_mut!(thread);
            // This case needs to be fixed, see: https://docs.oracle.com/javase/specs/jls/se18/html/jls-5.html#jls-5.1.8
            if !frame.current_method.returns_reference(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::ARETURN));
            }   
            let ret_val = match frame.op_stack.pop() {
                Some(v) => v,
                None => return Err(Error::StackUnderflow(Opcode::ARETURN)),
            };
            frame.op_stack = Vec::new();
            let current_class = Rc::clone(&frame.rt_const_pool);
            {
                let ret_descriptor = frame.current_method.return_descriptor(&current_class.get_class_file())?;
                let mut ret_ref = ret_val.as_reference()?;
                // This is my understanding of the rules regarding reference type assignment compatibility:
                // If a type is Null, it matches with any reference type. 
                // If a type is an array, it is therefore an instance of java.lang.reflect.Array, and so can be assigned with an array or the class Object.
                // If a type is an object, it can be assigned to any interface, superinterface, or superclass.
                // Now as for Interface, I'm not too sure. I believe it just can assign to superinterfaces, but I don't know for sure. 
                
                // We actually have to write a recursive function for this situation, because of Array assignment. 
                // If we assign an array to another array, we need to check the compatibility of their inner types.
                // For now, we just allow all arrays to be cast to any other array.
                // FIXME: ^   
                match ret_ref {
                    Reference::Null => (),
                    Reference::Array(a, _) => {
                        if !((ret_descriptor.as_bytes()[0] as char == '[') | (ret_descriptor == "java/Lang/Object")) {
                            return Err(Error::IncompatibleReturnType(Opcode::ARETURN));
                        }
                    },
                    Reference::Object(o, _) => {
                        let mut current_class = o.class().clone();
                        let mut found = false;
                        // These nested loops are rough, they should be tested and probably refactored.
                        while current_class.get_class_file().has_super() && found == false {
                            if current_class.get_class_file().name() == ret_descriptor {
                                found = true;
                                break;
                            }
                            for interface_index in current_class.get_class_file().interfaces() {
                                let interface = current_class.get_class_file().cp_entry(*interface_index)?.as_class()?;
                                let mut current_interface = self.resolve_class_reference(current_class.get_class_file().cp_entry(*interface)?.as_utf8()?)?;
                                let mut found_interface = false;
                                while current_interface.get_class_file().has_super() {
                                    if current_interface.get_class_file().name() == ret_descriptor {
                                        found_interface = true;
                                        break;
                                    }
                                    current_interface = self.resolve_class_reference(current_interface.get_class_file().super_name().unwrap())?;
                                }
                                if found_interface {
                                    found = true;
                                    break;
                                }
                            }
                            current_class = self.resolve_class_reference(current_class.get_class_file().super_name().unwrap())?;
                        }
                        if !found {
                            return Err(Error::IncompatibleReturnType(Opcode::ARETURN));
                        }
                    },
                    Reference::Interface(i, _) => {
                        let mut current_interface = i.clone();
                        let mut found = false;
                        // These nested loops are rough, they should be tested and probably refactored.
                        while current_interface.get_class_file().has_super() && found == false {
                            if current_interface.get_class_file().name() == ret_descriptor {
                                found = true;
                                break;
                            }
                            current_interface = self.resolve_class_reference(current_interface.get_class_file().super_name().unwrap())?;
                        }
                        if !found {
                            return Err(Error::IncompatibleReturnType(Opcode::ARETURN));
                        }
                    }
                }
            } 
            ret_val   
        };
        let thread = access_macros::current_thread_mut!(self);
        let _ = match thread.m_stack.pop() {
            Some(_2) => _2,
            None => return Err(Error::FrameStackUnderflow(Opcode::DRETURN)),
        };
        // This should sync up with method calls, but if we forget to not add the pc there, we will skip an opcode and corrupt the machine.
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(return_value);
        Ok(())
    }
    pub fn return_(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        {
            let frame = access_macros::current_frame_mut!(thread);
            println!("Returning from function {}, in class {}", frame.rt_const_pool.get_class_file().cp_entry(frame.current_method.name_index)?.as_utf8()?,
        frame.rt_const_pool.get_class_file().name());
            if !frame.current_method.returns_void(&frame.rt_const_pool.get_class_file())? {
                return Err(Error::IncompatibleReturnType(Opcode::RETURN));
            }
        }
        let _ = match thread.m_stack.pop() {
            Some(_2) => _2,
            None => return Err(Error::FrameStackUnderflow(Opcode::RETURN)),
        };
        Ok(())
    }
    // References
    pub fn getstatic(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        let current_class = Rc::clone(&frame.rt_const_pool);
        let field = current_class.get_class_file().cp_entry(index)?.as_field_ref()?;
        let class_index = current_class.get_class_file().cp_entry(field.class_index)?.as_class()?;
        let class_name = current_class.get_class_file().cp_entry(*class_index)?.as_utf8()?;
        let class = self.resolve_class_reference(class_name)?;
        let name_and_type = current_class.get_class_file().cp_entry(field.name_and_type_index)?.as_name_and_type()?;
        let name = current_class.get_class_file().cp_entry(name_and_type.name_index)?.as_utf8()?;
        let descriptor = current_class.get_class_file().cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
        let field = class.get_static(name, &descriptor, self)?;
        let new_value;
        // If this value is a "copy" type, we make a new Value. Otherwise, we use the reference inside to make a new value.       
        if field.is_reference() {
            let inner_ref = field.as_reference().unwrap();
            new_value = Value::Reference(inner_ref.clone());
        }
        else {
            new_value = field.clone();
        }
        let thread = access_macros::current_thread_mut!(self);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(new_value);
        Ok(())
    }
    pub fn putstatic(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        let current_class = Rc::clone(&frame.rt_const_pool);
        let field = current_class.get_class_file().cp_entry(index)?.as_field_ref()?;
        let class_index = current_class.get_class_file().cp_entry(field.class_index)?.as_class()?;
        let class_name = current_class.get_class_file().cp_entry(*class_index)?.as_utf8()?;
        let mut class = self.resolve_class_reference(&class_name)?;
        let name_and_type = current_class.get_class_file().cp_entry(field.name_and_type_index)?.as_name_and_type()?;
        let name = current_class.get_class_file().cp_entry(name_and_type.name_index)?.as_utf8()?;
        let descriptor = current_class.get_class_file().cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
        let thread = access_macros::current_thread_mut!(self);
        let frame = access_macros::current_frame_mut!(thread);
        let field = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::PUTSTATIC)),
        };
        unsafe {Rc::get_mut_unchecked(&mut class).put_static(name, descriptor, field, self)?; }
        Ok(())
    }
    // TODO: There is a bug in these two functions that we fixed in the _statics. 
    // We have to resolve the name and type in the current function, and use string comparisons. 
    // Check the commit history on the _statics for more info.
    pub fn getfield(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        let mut object_ref = match frame.op_stack.pop() {
            Some(mut v) => v.as_reference_mut()?,
            None => return Err(Error::StackUnderflow(Opcode::GETFIELD)),
        };
        let current_class = frame.rt_const_pool.clone();
        let val = {
            let object = match object_ref {
                Reference::Object(o, _) => o,
                _ => return Err(Error::IncorrectReferenceType(Opcode::GETFIELD)),
            };
            object.get_field(current_class, index, self)?
        };
        let thread = access_macros::current_thread_mut!(self);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(val);
        Ok(())
    }
    pub fn putfield(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        let val = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::PUTFIELD)),
        };
        let mut object_ref = match frame.op_stack.pop() {
            Some(mut v) => v.as_reference_mut()?,
            None => return Err(Error::StackUnderflow(Opcode::PUTFIELD)),
        };
        let object = match object_ref {
            Reference::Object(o, _) => o,
            _ => return Err(Error::IncorrectReferenceType(Opcode::PUTFIELD)),
        };
        object.put_field(frame.rt_const_pool.clone(), index, self, val)?;
        Ok(())
    }
    pub fn invokevirtual(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let current_class = Rc::clone(&frame.rt_const_pool);
        let (mut c, name, descriptor) = {
            let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);   
            let method_ref = current_class.get_class_file().cp_entry(index)?.as_method_ref()?;
            let name_and_type = current_class.get_class_file().cp_entry(method_ref.name_and_type_index)?.as_name_and_type()?;
            let name = current_class.get_class_file().cp_entry(name_and_type.name_index)?.as_utf8()?;
            let descriptor = current_class.get_class_file().cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
            let c_info = current_class.get_class_file().cp_entry(method_ref.class_index)?.as_class()?;
            let c_name = current_class.get_class_file().cp_entry(*c_info)?.as_utf8()?;
            (self.resolve_class_reference(c_name.clone().as_str())?, name, descriptor)
        };
        let thread = access_macros::current_thread_mut!(self);
        let frame = access_macros::current_frame_mut!(thread);
        let object_val = match frame.op_stack.pop() {
            Some(o) => o,
            None => return Err(Error::StackUnderflow(Opcode::INVOKEVIRTUAL)),
        };
        let object_ref = object_val.as_reference()?;
        let object = match object_ref {
            Reference::Object(o, _) => o,
            _ => return Err(Error::IncorrectReferenceType(Opcode::INVOKEVIRTUAL)), 
        };
        let mut method_to_call = None; 
        // Resolve method
        {
            let mut found = false;
            while c.get_class_file().has_super() && !found {
                for method in c.get_class_file().methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = c.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = c.get_class_file().cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        method_to_call = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found {
                    c = self.resolve_class_reference(c.get_class_file().super_name().unwrap())?;
                }
                
            }
            // TODO: Search Superinterfaces of c.
            if !found {
                return Err(Error::NoSuchMethodError(Opcode::MethodInvoke));
            }
        }
        let mut resolved_method = method_to_call.unwrap();
        {
            // search the methods of the object to see if we can override. 
            if (resolved_method.access_flags.flags & flags::method::ACC_PRIVATE) == 0 {
                let mut obj_c = object.class().clone();
                let mut found = false;
                while obj_c.get_class_file().has_super() && !found {
                    for method in obj_c.get_class_file().methods() {
                        // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                        // We still need to check for signature polymorphic functions.
                        let method_descriptor = obj_c.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()?;
                        if method_descriptor != descriptor {
                            continue;
                        }
                        let method_name = obj_c.get_class_file().cp_entry(method.name_index)?.as_utf8()?;
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
                    }
                    
                }
                // TODO: Search Superinterfaces of c.
            }
        };
        if (resolved_method.access_flags.flags & flags::method::ACC_STATIC) > 0 {
            return Err(Error::IncompatibleClassChangeError(Opcode::INVOKEVIRTUAL));
        } 
        if (resolved_method.access_flags.flags & flags::method::ACC_ABSTRACT) > 0 {
            return Err(Error::AbstractMethodError(Opcode::INVOKEVIRTUAL));
        } 
        if (resolved_method.access_flags.flags & flags::method::ACC_SYNCHRONIZED) > 0 {
            // TODO: Enter monitors on Classes.
        } 
        if (resolved_method.access_flags.flags & flags::method::ACC_NATIVE) > 0 {
            return self.execute_native(&resolved_method, c);
        } 
        self.setup_method_call(&resolved_method, c, true)
    }
    pub fn invokespecial(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16); 
        let entry = frame.rt_const_pool.get_class_file().cp_entry(index)?;
        let (method_ref, is_interface) = match entry {
            Entry::MethodRef(refinfo) => (refinfo, false),
            Entry::InterfaceMethodRef(refinfo) => (refinfo, true),
            _ => return Err(Error::IllegalConstantLoad(Opcode::INVOKESTATIC)),
        };
        let current_class = Rc::clone(&frame.rt_const_pool);
        let name_and_type = current_class.get_class_file().cp_entry(method_ref.name_and_type_index)?.as_name_and_type()?;
        let name = current_class.get_class_file().cp_entry(name_and_type.name_index)?.as_utf8()?;
        let descriptor = current_class.get_class_file().cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
        let c_info = current_class.get_class_file().cp_entry(method_ref.class_index)?.as_class()?;
        let c_name = current_class.get_class_file().cp_entry(*c_info)?.as_utf8()?;
        let mut c = self.resolve_class_reference(c_name)?;
        if (c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) > 0 {
            return Err(Error::IncompatibleClassChangeError(Opcode::INVOKESPECIAL));
        } 
        // Resolve method
        let mut resolved_method_wrapped = None; 
        {
            let mut found = false;
            while c.get_class_file().has_super() && !found {
                for method in c.get_class_file().methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = c.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = c.get_class_file().cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        resolved_method_wrapped = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found {
                    c = self.resolve_class_reference(c.get_class_file().super_name().unwrap())?;
                }
                
            }
            // TODO: Search Superinterfaces of c.
            if !found {
                return Err(Error::NoSuchMethodError(Opcode::MethodInvoke));
            }
        }
        let mut resolved_method = resolved_method_wrapped.unwrap();
        let resolved_method_class = Rc::clone(&c); // Save the originating class of resolved_method so we can index it.
        let resolved_name = resolved_method_class.get_class_file().cp_entry(resolved_method.name_index)?.as_utf8()?;
        let resolved_desc = resolved_method_class.get_class_file().cp_entry(resolved_method.descriptor_index)?.as_utf8()?;
        // Next, possibly change C
        if (c.get_class_file().cp_entry(resolved_method.name_index)?.as_utf8()? != "<init>") && current_class.get_class_file().has_super() && ({
            // Figure out if c is a superclass of current_class
            let mut temp = Rc::clone(&current_class);
            let mut res = false;
            while temp.get_class_file().has_super() {
                let super_name = temp.get_class_file().super_name().unwrap();
                if super_name == c.get_class_file().name() {
                    res = true;
                    break;
                }
                temp = self.resolve_class_reference(super_name)?;
            }
            res
        })
        && ((c.get_class_file().access_flags().flags & flags::class::ACC_SUPER) > 0 ) {
            c = self.resolve_class_reference(c.get_class_file().super_name().unwrap())?;
        }
        let mut actual_method_wrapped = Some(resolved_method.clone());
        let mut actual_class = Rc::clone(&resolved_method_class);
        // Now, select *actual* method
        // All methods have to be instance methods. 
        // First, check methods of c for a method with the same name and desc.
        for method in c.get_class_file().methods() {
            if (method.access_flags.flags & flags::method::ACC_STATIC) > 0 {
                continue;
            }
            if (c.get_class_file().cp_entry(method.name_index)?.as_utf8()? == resolved_name) && (c.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()? == resolved_desc) {
                actual_method_wrapped = Some(method.clone());
                actual_class = Rc::clone(&c);
                break;
            }
        }
        // Check superclasses, if c is a class
        if actual_method_wrapped.is_none() & ((c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) == 0) {
            let mut c_super = Rc::clone(&c);
            while c_super.get_class_file().has_super() {
                for method in c.get_class_file().methods() {
                    if (method.access_flags.flags & flags::method::ACC_STATIC) > 0 {
                        continue;
                    }
                    if (c_super.get_class_file().cp_entry(method.name_index)?.as_utf8()? == resolved_name) && 
                        (c_super.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()? == resolved_desc) {
                        actual_method_wrapped = Some(method.clone());
                        actual_class = Rc::clone(&c_super);
                        break;
                    }
                }
                c_super = self.resolve_class_reference(c_super.get_class_file().super_name().unwrap())?;
            }
        }
        // Check object, if c in an interface.
        if actual_method_wrapped.is_none() & ((c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) > 0) {
            let obj_class = self.resolve_class_reference("java/lang/Object")?;
            for method in obj_class.get_class_file().methods() {
                if (method.access_flags.flags & flags::method::ACC_STATIC) > 0 {
                    continue;
                }
                if (obj_class.get_class_file().cp_entry(method.name_index)?.as_utf8()? == resolved_name) && 
                   (obj_class.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()? == resolved_desc) {
                    actual_method_wrapped = Some(method.clone());
                    actual_class = Rc::clone(&obj_class);
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
            let current_thread_number = self.m_thread_index;
            let thread = access_macros::current_thread_mut!(self);
            let frame = access_macros::current_frame_mut!(thread);
            let mut object = match frame.op_stack.pop() {
                Some(o) => o.as_reference()?,
                None => return Err(Error::StackUnderflow(Opcode::INVOKESPECIAL)),
            };
            match object {
                Reference::Null => return Err(Error::NullPointerException(Opcode::MONITORENTER)),
                Reference::Array(_, mrc) | Reference::Interface(_, mrc) | Reference::Object(_, mrc) => {
                    let m = unsafe { Rc::get_mut_unchecked(&mut mrc)};
                    let success = m.try_enter(current_thread_number);
                    if !success {
                        thread.current_monitor = Some(mrc.clone());
                        return Ok(()); // We have to block
                    }
                },
            }
        } 
        if (actual_method.access_flags.flags & flags::method::ACC_NATIVE) > 0 {
            return self.execute_native(&actual_method, actual_class);
        } 
        // TODO: Refactor self.setup_method_call() to handle native methods and synchronized ones (and rename it).
        self.setup_method_call(&actual_method, actual_class, false)
    }
    pub fn invokestatic(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);     
        let entry = frame.rt_const_pool.get_class_file().cp_entry(index)?;
        let (method_ref, is_interface) = match entry {
            Entry::MethodRef(refinfo) => (refinfo, false),
            Entry::InterfaceMethodRef(refinfo) => (refinfo, true),
            _ => return Err(Error::IllegalConstantLoad(Opcode::INVOKESTATIC)),
        };
        let current_class = Rc::clone(&frame.rt_const_pool);
        let name_and_type = current_class.get_class_file().cp_entry(method_ref.name_and_type_index)?.as_name_and_type()?;
        let name = current_class.get_class_file().cp_entry(name_and_type.name_index)?.as_utf8()?;
        let descriptor = current_class.get_class_file().cp_entry(name_and_type.descriptor_index)?.as_utf8()?;
        let c_info = current_class.get_class_file().cp_entry(method_ref.class_index)?.as_class()?;
        let c_name = current_class.get_class_file().cp_entry(*c_info)?.as_utf8()?;
        let mut c = self.resolve_class_reference(c_name)?;
        if is_interface {
            if !((c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) > 0) {
                return Err(Error::IncompatibleMethodRefAndClass(Opcode::INVOKESTATIC));
            }
        }
        else {
            if (c.get_class_file().access_flags().flags & flags::class::ACC_INTERFACE) > 0 {
                return Err(Error::IncompatibleMethodRefAndClass(Opcode::INVOKESTATIC));
            }
        }
        let mut method_to_call = None; 
        // Resolve method
        {
            let mut found = false;
            while c.get_class_file().has_super() && !found {
                for method in c.get_class_file().methods() {
                    // https://docs.oracle.com/javase/specs/jvms/se18/html/jvms-5.html#jvms-5.4.3.3
                    // We still need to check for signature polymorphic functions.
                    let method_descriptor = c.get_class_file().cp_entry(method.descriptor_index)?.as_utf8()?;
                    if method_descriptor != descriptor {
                        continue;
                    }
                    let method_name = c.get_class_file().cp_entry(method.name_index)?.as_utf8()?;
                    if method_name == name {
                        method_to_call = Some(method.clone());
                        found = true;
                        break;
                    }
                }   
                // Recurse up the inheritance tree.
                if !found {
                    c = self.resolve_class_reference(c.get_class_file().super_name().unwrap())?;
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
        if (method.access_flags.flags & flags::method::ACC_NATIVE) > 0 {
            return self.execute_native(&method, c);
        } 
        println!("Invoking static method {}{} in class {}", name, descriptor, c_name);
        self.setup_method_call(&method, c, true)
    }
    pub fn invokeinterface(&mut self) -> Result<(), Error> {
        Err(Error::Todo(Opcode::INVOKEINTERFACE))
    }
    pub fn invokedynamic(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(5)?; // the 5 is intentional
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        // Resolve the index into an instance of java.lang.invoke.CallSite.
        // This procedure also occurs in ldc, so this should be a subroutine.
        let invoke_dynamic_info = frame.rt_const_pool.get_class_file().cp_entry(index)?.as_invoke_dynamic()?;
        let bootstrap_index = invoke_dynamic_info.bootstrap_method_attr_index;
        let bootstrap_methods = match &frame.rt_const_pool.get_class_file().bootstrap_methods {
            Some(m) => m,
            None => return Err(Error::MissingBootstrapTable(Opcode::INVOKEDYNAMIC)),
        };
        let bootstrap_method = &bootstrap_methods[bootstrap_index as usize];
        let method_handle = frame.rt_const_pool.get_class_file().cp_entry(bootstrap_method.bootstrap_method_ref)?.as_method_handle()?;
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
    pub fn new(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        let current_class = frame.rt_const_pool.clone();
        drop(frame);
        drop(thread);
        let objectref = Reference::new_object(current_class, index, self)?;
        let thread = access_macros::current_thread_mut!(self);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Reference(objectref));
        Ok(())
    }
    pub fn newarray(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(2)?;
        let frame = access_macros::current_frame_mut!(thread);
        let atype = frame.current_method.code()?[pc];
        let count = match frame.op_stack.pop() {
            Some(v) => *v.as_int()?,
            None => return Err(Error::StackUnderflow(Opcode::NEWARRAY)),
        };
        if count < 0 {
            return Err(Error::NegativeArraySizeException(Opcode::NEWARRAY));
        }
        let arrayref = Reference::new_array(count as usize, atype);
        let arrayval = Value::Reference(arrayref);
        frame.op_stack.push(arrayval);
        Ok(())
 
    }
    pub fn anewarray(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        let count = match frame.op_stack.pop() {
            Some(v) => v,
            None => return Err(Error::StackUnderflow(Opcode::PUTFIELD)),
        };
        let descriptor = {
            let class_ref = frame.rt_const_pool.get_class_file().cp_entry(index)?.as_class()?;
            let class_name = frame.rt_const_pool.get_class_file().cp_entry(*class_ref)?.as_utf8()?;
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
    pub fn arraylength(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
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
    pub fn athrow(&mut self) -> Result<(), Error> {
        let (should_init, null_class) = {
            let thread = access_macros::current_thread_mut!(self);
            let frame = access_macros::current_frame_mut!(thread);
            let len = frame.op_stack.len();
            if len == 0 {
                return Err(Error::StackUnderflow(Opcode::ATHROW));
            }
            let objectref_rc = match &frame.op_stack[len - 1] {
                Value::Reference(r) => r,
                _ => return Err(Error::UnexpectedTypeOnStack(Opcode::ATHROW)),
            };
            let objectref = &*objectref_rc;
            drop(frame);
            drop(thread);            
            match objectref {
                Reference::Array(_, _) | Reference::Interface(_, _) => return Err(Error::IncorrectReferenceType(Opcode::ATHROW)),
                Reference::Null => {
                    // We could pop the other exception here, but we already clear it in self.handle_exception, so there's not really a point.
                    let exception = object::new_object("java/Lang/NullPointerException", self)?;
                    let exception_class = exception.class().clone();
                    let exception_ref = Reference::Object(exception, Rc::new(Monitor::new()));
                    let exception_val = Value::Reference(exception_ref);
                    let thread = access_macros::current_thread_mut!(self);
                    let frame = access_macros::current_frame_mut!(thread);
                    frame.op_stack.push(exception_val);
                    (true, Some(exception_class))
                },
                Reference::Object(_, _) => (false, None),
            }
        };
        if should_init {
            self.setup_method_call_from_name("<init>", "()V", null_class.unwrap(), false)?;
            self.run_until_method_exit();
        }
        Err(Error::Exception)
    }
    pub fn checkcast(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        let op_stack_len = frame.op_stack.len();
        let objectref = &frame.op_stack[op_stack_len - 1].as_reference()?;
        let name: String;
        let object_desc = match *objectref {
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
        let class_reference = *class.get_class_file().cp_entry(index)?.as_class()?;
        let class_desc = class.get_class_file().cp_entry(class_reference)?.as_utf8()?.as_str();
        drop(frame);
        drop(thread);
        if self.check_class(object_desc, class_desc)? {
            return Ok(());
        }
        Err(Error::ClassCastException(Opcode::CHECKCAST))

    }
    pub fn instanceof(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(3)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        let objectref = match frame.op_stack.pop() {
            Some(v) => v.as_reference()?,
            None => return Err(Error::StackUnderflow(Opcode::INSTANCEOF)),
        };
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
            Reference::Null => {
                frame.op_stack.push(Value::Int(0));
                return Ok(());
            },
        };
        let class = Rc::clone(&frame.rt_const_pool);
        let class_reference = *class.get_class_file().cp_entry(index)?.as_class()?;
        let class_desc = class.get_class_file().cp_entry(class_reference)?.as_utf8()?.as_str();
        drop(frame);
        drop(thread);
        if self.check_class(object_desc, class_desc)? {
            let thread = access_macros::current_thread_mut!(self);
            let frame = access_macros::current_frame_mut!(thread);
            frame.op_stack.push(Value::Int(0));
            return Ok(());
        }
        let thread = access_macros::current_thread_mut!(self);
        let frame = access_macros::current_frame_mut!(thread);
        frame.op_stack.push(Value::Int(1));
        Ok(())
    }
    pub fn monitorenter(&mut self) -> Result<(), Error> {
        let current_thread_number = self.m_thread_index;
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let mut object = match frame.op_stack.pop() {
            Some(o) => o.as_reference()?,
            None => return Err(Error::StackUnderflow(Opcode::MONITORENTER)),
        };
        match object {
            Reference::Null => return Err(Error::NullPointerException(Opcode::MONITORENTER)),
            Reference::Array(_, mrc) | Reference::Interface(_, mrc) | Reference::Object(_, mrc) => {
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
    pub fn monitorexit(&mut self) -> Result<(), Error> {
        let current_thread_number = self.m_thread_index;
        let thread = access_macros::current_thread_mut!(self);
        thread.inc_pc(1)?;
        let frame = access_macros::current_frame_mut!(thread);
        let mut object = match frame.op_stack.pop() {
            Some(o) => o.as_reference()?,
            None => return Err(Error::StackUnderflow(Opcode::MONITOREXIT)),
        };
        match object {
            Reference::Null => return Err(Error::NullPointerException(Opcode::MONITOREXIT)),
            Reference::Array(_, mrc) | Reference::Interface(_, mrc) | Reference::Object(_, mrc) => {
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
    // Extended
    pub fn wide(&mut self) -> Result<(), Error> {
        Err(Error::Wide)
    }
    pub fn multianewarray(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        thread.inc_pc(4)?;
        let frame = access_macros::current_frame_mut!(thread);
        let index = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
        let dimensions = frame.current_method.code()?[pc + 2] as usize;
        if dimensions == 0 {
            return Err(Error::IllegalDimensionCount);
        }
        let len = frame.op_stack.len();
        if len >= dimensions {
            return Err(Error::NotEnoughDimensionValues);
        }
        let counts = &frame.op_stack[(len - dimensions)..0];
        let array_class_desc_index = *frame.rt_const_pool.get_class_file().cp_entry(index)?.as_class()?;
        let array_class_desc = frame.rt_const_pool.get_class_file().cp_entry(array_class_desc_index)?.as_utf8()?.clone();
        let array = Array::new_multi(dimensions as u8, counts, array_class_desc)?;
        let array_ref = Reference::Array(Rc::new(array), Rc::new(Monitor::new()));
        let array_ref_val = Value::Reference(array_ref);
        frame.op_stack.push(array_ref_val);
        Ok(())
        
    }
    pub fn ifnull(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let offset = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
            let value = match frame.op_stack.pop() {
                Some(v) => v.as_reference()?,
                None => return Err(Error::StackUnderflow(Opcode::IFNONNULL)),
            };
            if let Reference::Null = value {
                offset
            }
            else {
                3
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn ifnonnull(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let offset = {
            let frame = access_macros::current_frame_mut!(thread);
            let offset = (frame.current_method.code()?[pc] as u16) << 8 | (frame.current_method.code()?[pc + 1] as u16);
            let value = match frame.op_stack.pop() {
                Some(v) => v.as_reference()?,
                None => return Err(Error::StackUnderflow(Opcode::IFNONNULL)),
            };
            if let Reference::Null = value {
                3
            }
            else {
                offset
            }
        };
        thread.inc_pc(offset as isize)?;
        Ok(())
    }
    pub fn goto_w(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let boffset = {
            let frame = access_macros::current_frame_mut!(thread);
            let bbyte1 = frame.current_method.code()?[pc] as i32;
            let bbyte2 = frame.current_method.code()?[pc + 1] as i32;
            let bbyte3 = frame.current_method.code()?[pc + 2] as i32;
            let bbyte4 = frame.current_method.code()?[pc + 3] as i32;
            bbyte1 << 24 | bbyte2 << 16 | bbyte3 << 8 | bbyte4
        };
        thread.inc_pc(boffset as isize)?;
        Ok(())
    }
    pub fn jsr_w(&mut self) -> Result<(), Error> {
        let thread = access_macros::current_thread_mut!(self);
        let pc = thread.pc() + 1;
        let boffset = {
            let frame = access_macros::current_frame_mut!(thread);
            frame.op_stack.push(Value::ReturnAddress(pc as u16 + 2));
            let bbyte1 = frame.current_method.code()?[pc] as i32;
            let bbyte2 = frame.current_method.code()?[pc + 1] as i32;
            let bbyte3 = frame.current_method.code()?[pc + 2] as i32;
            let bbyte4 = frame.current_method.code()?[pc + 3] as i32;
            bbyte1 << 24 | bbyte2 << 16 | bbyte3 << 8 | bbyte4
        };
        thread.inc_pc(boffset as isize)?;
        Ok(())
    }
    // Reserved
    pub fn breakpoint(&mut self) -> Result<(), Error> {
        Err(Error::Breakpoint)
    }
    pub fn impdep1(&mut self) -> Result<(), Error> {
        Err(Error::ImpDep1)
    }
    pub fn impdep2(&mut self) -> Result<(), Error> {
        Err(Error::ImpDep2)
    }
}