pub mod array;
pub mod object; 
pub mod packedboolarray;

use crate::reference::array::Array;
use crate::class::Class;
use crate::errorcodes::Error;
use crate::reference::object::Object;

use std::collections::HashMap;
use std::rc::Rc;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Reference {
    Null,
    Array(Array, Rc<Monitor>),
    Interface(Rc<Class>, Rc<Monitor>), // As far as I can tell, Interfaces are just special classes, and so can just use the Class type.
    Object(Object, Rc<Monitor>),
}

// Should we make a wrapper for Array::new_multi?

impl<'a>  Reference {
    pub fn new() ->  Reference {
        Reference::Null
    }
    pub fn new_refarray(size: usize, descriptor: String) ->  Reference {
        Reference::Array(Array::new_ref(size, descriptor), Rc::new(Monitor::new()))
    }
    pub fn new_array(size: usize, atype: u8) ->  Reference {
        Reference::Array(Array::new(size, atype), Rc::new(Monitor::new()))
    }
    pub fn new_interface(c: Rc< Class>) ->  Reference {
        Reference::Interface(c, Rc::new(Monitor::new()))
    }
    pub fn new_object(current_class: Rc<Class>, class_index: u16, loaded_classses: &'a mut HashMap<String, Rc< Class>>) -> Result< Reference, Error> {
        Ok(Reference::Object(Object::new(current_class, class_index, loaded_classses)?, Rc::new(Monitor::new())))
    }

    // Again, the runtime hit hurts, but we need to test while the design is still unstable, and I can't think of a better solution. 
    pub fn to_refarray(reference: Reference, size: usize, descriptor: String) -> Result<Reference, Error> {
        // As far as I understand it, this should make ref illegal to use after this function.
        match reference {
            Reference::Null => Ok(Reference::new_refarray(size, descriptor)),
            _ => Err(Error::IllegalReferenceCastToArray),
        }
    }

    pub fn to_interface(reference: Reference, c: Rc< Class>) -> Result< Reference, Error> {
        match reference {
            Reference::Null => Ok(Reference::new_interface(c)),
            _ => Err(Error::IllegalReferenceCastToInterface),
        }
    }


    pub fn to_object(reference: Reference, current_class: Rc<Class>, class_index: u16, loaded_classses: &'a mut HashMap<String, Rc< Class>>) -> Result< Reference, Error> {
        match reference {
            Reference::Null => Ok(Reference::new_object(current_class, class_index, loaded_classses)?),
            _  => Err(Error::IllegalReferenceCastToObject),
        }
    }

    
}