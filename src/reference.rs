pub mod array;
pub mod object; 
pub mod packedboolarray;

use crate::jvm::JVM;
use crate::reference::array::Array;
use crate::class::Class;
use crate::errorcodes::{Error, Opcode};
use crate::reference::object::Object;

use std::rc::Rc;

use self::object::customobject::CustomObject;

use colored::Colorize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monitor {
    pub owned_thread: usize,
    pub entry_count: usize,
}

impl Monitor {
    pub fn new() -> Monitor {
        Monitor { owned_thread: 0, entry_count: 0 } // It doesn't matter what thread originally creates it, because an object is not locked until someone tries to enter it.
    } 
    // These functions will have to be atmoic if we want actual multithreading.
    // See: https://doc.rust-lang.org/nomicon/atomics.html
    // For now, we are a single threaded JVM pretending to be multithreaded, so we don't have to worry.
    pub fn try_enter(&mut self, current_thread: usize) -> bool {
        if self.entry_count == 0 {
            self.owned_thread = current_thread;
            self.entry_count = 1;
            true
        }
        else if self.owned_thread == current_thread {
            self.entry_count += 1;
            true
        }
        else {
            false
        }
    }
    pub fn try_exit(&mut self, current_thread: usize) -> bool {
        if self.owned_thread != current_thread || self.entry_count == 0  {
            false
        }
        else {
            self.entry_count -= 1;
            true
        }
    }
}

impl Default for Monitor {
    fn default() -> Self {
        Self::new()
    }
}

// We might want to have all these types hold their own monitors.
#[derive(Debug, PartialEq)]
pub enum Reference<C: Class + ?Sized, O: Object + ?Sized> {
    Null,
    Array(Rc<Array<C, O>>, Rc<Monitor>),
    Interface(Rc<C>, Rc<Monitor>), // As far as I can tell, Interfaces are just special classes, and so can just use the Class type.
    Object(Rc<O>, Rc<Monitor>),
}

impl<C: Class + ?Sized, O: Object + ?Sized> Clone for Reference<C, O> {
    fn clone(&self) -> Self {
        match self {
            Reference::Null => Reference::Null,
            Reference::Array(a, m) => Reference::Array(Rc::clone(a),Rc::clone(m)),
            Reference::Interface(i, m) => Reference::Interface(Rc::clone(i),Rc::clone(m)),
            Reference::Object(o, m) => Reference::Object(Rc::clone(o),Rc::clone(m)),
        }
    }
}

// Should we make a wrapper for Array::new_multi?

impl Reference<dyn Class, dyn Object> {
    pub fn new() -> Reference<dyn Class, dyn Object> {
        Reference::Null
    }
    pub fn new_refarray(size: usize, descriptor: String) -> Reference<dyn Class, dyn Object> {
        Reference::Array(Rc::new(Array::new_ref(size, descriptor)), Rc::new(Monitor::new()))
    }
    pub fn new_array(size: usize, atype: u8) ->  Reference<dyn Class, dyn Object> {
        Reference::Array(Rc::new(Array::new(size, atype)), Rc::new(Monitor::new()))
    }
    pub fn new_interface<CC: Class + ?Sized>(c: Rc<CC>) ->  Reference<CC, dyn Object> {
        Reference::Interface(c, Rc::new(Monitor::new()))
    }
    pub fn new_object(current_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Reference<dyn Class, dyn Object>, Error> {
        Ok(Reference::Object(
            object::new_object(Some(current_class), 
            Some(class_index), 
            jvm)?, 
        Rc::new(Monitor::new())))
    }

    // Again, the runtime hit hurts, but we need to test while the design is still unstable, and I can't think of a better solution. 
    pub fn to_refarray(reference: Reference<dyn Class, dyn Object>, size: usize, descriptor: String) -> Result<Reference<dyn Class, dyn Object>, Error> {
        // As far as I understand it, this should make ref illegal to use after this function.
        match reference {
            Reference::Null => Ok(Reference::new_refarray(size, descriptor)),
            _ => Err(Error::IllegalReferenceCastToArray),
        }
    }
    pub fn to_interface<CC: Class>(reference: Reference<dyn Class, dyn Object>, c: Rc<CC>) -> Result<Reference<CC, dyn Object>, Error> {
        match reference {
            Reference::Null => Ok(Reference::new_interface(c)),
            _ => Err(Error::IllegalReferenceCastToInterface),
        }
    }
    pub fn to_object(reference: Reference<dyn Class, dyn Object>, current_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Reference<dyn Class, dyn Object>, Error> {
        match reference {
            Reference::Null => Ok(Reference::<dyn Class, dyn Object>::new_object(current_class, class_index, jvm)?),
            _  => Err(Error::IllegalReferenceCastToObject),
        }
    }

    pub fn ptr_eq(this: &Reference<dyn Class, dyn Object>, other: &Reference<dyn Class, dyn Object>) -> bool {
        match this {
            Reference::Null => matches!(other, Reference::Null),
            Reference::Array(a, m) => match other {
                #[allow(clippy::vtable_address_comparisons)] 
                Reference::Array(a1, m1) =>            
                    Rc::ptr_eq(a, a1) && Rc::ptr_eq(m, m1),
                _ => false,
            },
            Reference::Interface(i, m) => match other {
                #[allow(clippy::vtable_address_comparisons)] 
                Reference::Interface(i1, m1) => 
                    Rc::ptr_eq(i, i1) && Rc::ptr_eq(m, m1),
                _ => false,
            },
            Reference::Object(o, m) => match other {
                #[allow(clippy::vtable_address_comparisons)] 
                Reference::Object(o1, m1) => 
                Rc::ptr_eq(o, o1) && Rc::ptr_eq(m, m1),
                _ => false,
            },
        }
    }

    pub fn as_object(&self) -> Result<&Rc<dyn Object>, Error> {
        if let Reference::Object(o, _) = self {
            Ok(o)
        }
        else if let Reference::Null = self {
            Err(Error::NullPointerException(Opcode::Unknown))
        }
        else {
            Err(Error::IllegalReferenceCastToObject)
        }
    }

    pub fn as_object_mut(&mut self) -> Result<&mut Rc<dyn Object>, Error> {
        if let Reference::Object(o, _) = self {
            Ok(o)
        }
        else if let Reference::Null = self {
            Err(Error::NullPointerException(Opcode::Unknown))
        }
        else {
            Err(Error::IllegalReferenceCastToObject)
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Reference::Null)
    }
    
}


impl PartialEq for Reference<dyn Class, dyn Object> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Reference::Null => matches!(other, Reference::Null),
            Reference::Array(a, m) => match other {
                Reference::Array(a1, m1) => 
                    a == a1 && Rc::ptr_eq(m, m1),
                _ => false,
            },
            Reference::Interface(i, m) => match other {
                #[allow(clippy::vtable_address_comparisons)]
                Reference::Interface(i1, m1) => 
                    Rc::ptr_eq(i, i1) && Rc::ptr_eq(m, m1),
                _ => false,
            },
            Reference::Object(o, m) => match other {
                Reference::Object(o1, m1) => 
                    o == o1 && Rc::ptr_eq(m, m1),
                _ => false,
            },
        }
    }
}

impl Default for Reference<dyn Class, dyn Object> {
    fn default() -> Self {
        Self::new()
    }
}


