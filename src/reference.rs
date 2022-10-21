pub mod array;
pub mod object; 
pub mod packedboolarray;

use crate::jvm::JVM;
use crate::reference::array::Array;
use crate::class::Class;
use crate::errorcodes::Error;
use crate::reference::object::Object;

use std::collections::HashMap;
use std::rc::Rc;

use self::object::customobject::CustomObject;

#[derive(Clone, Debug, PartialEq)]
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
        if self.owned_thread != current_thread {
            false
        }
        else if self.entry_count == 0 {
            false
        }
        else {
            self.entry_count -= 1;
            true
        }
    }
}

// We might want to have all these types hold their own monitors.
#[derive(Debug, Eq, PartialEq)]
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

impl<C: Class + ?Sized, O: Object + ?Sized>  Reference<C, O> {
    pub fn new() -> Reference<C, O> {
        Reference::Null
    }
    pub fn new_refarray(size: usize, descriptor: String) -> Reference<C, O> {
        Reference::Array(Rc::new(Array::new_ref(size, descriptor)), Rc::new(Monitor::new()))
    }
    pub fn new_array(size: usize, atype: u8) ->  Reference<C, O> {
        Reference::Array(Rc::new(Array::new(size, atype)), Rc::new(Monitor::new()))
    }
    pub fn new_interface(c: Rc<C>) ->  Reference<C, O> {
        Reference::Interface(c, Rc::new(Monitor::new()))
    }
    pub fn new_object(current_class: Rc<dyn Class>, class_index: u16, jvm: &mut JVM) -> Result<Reference<dyn Class, dyn Object>, Error> {
        Ok(Reference::Object(CustomObject::<dyn Class>::new(current_class, class_index, jvm)?, Rc::new(Monitor::new())))
    }

    // Again, the runtime hit hurts, but we need to test while the design is still unstable, and I can't think of a better solution. 
    pub fn to_refarray(reference: Reference<C, O>, size: usize, descriptor: String) -> Result<Reference<C, O>, Error> {
        // As far as I understand it, this should make ref illegal to use after this function.
        match reference {
            Reference::Null => Ok(Reference::new_refarray(size, descriptor)),
            _ => Err(Error::IllegalReferenceCastToArray),
        }
    }
    pub fn to_interface<CC: Class>(reference: Reference<C, O>, c: Rc<CC>) -> Result<Reference<CC, O>, Error> {
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
            Reference::Null => match other {
                Reference::Null => true,
                _ => false,
            },
            Reference::Array(a, m) => match other {
                Reference::Array(a1, m1) => 
                    Rc::ptr_eq(a, a1) && Rc::ptr_eq(m, m1),
                _ => false,
            },
            Reference::Interface(i, m) => match other {
                Reference::Interface(i1, m1) => 
                    Rc::ptr_eq(i, i1) && Rc::ptr_eq(m, m1),
                _ => false,
            },
            Reference::Object(o, m) => match other {
                Reference::Object(o1, m1) => 
                Rc::ptr_eq(o, o1) && Rc::ptr_eq(m, m1),
                _ => false,
            },
        }
    }
    
}

impl PartialEq for Reference<dyn Class, dyn Object> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Reference::Null => match other {
                Reference::Null => true,
                _ => false,
            },
            Reference::Array(a, m) => match other {
                Reference::Array(a1, m1) => 
                    a == a1 && Rc::ptr_eq(m, m1),
                _ => false,
            },
            Reference::Interface(i, m) => match other {
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
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

