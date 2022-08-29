#[macro_export]
macro_rules! current_thread_mut {
    ($this:ident) => {
        &mut $this.m_threads[$this.m_thread_index]
    }
}
pub (crate) use current_thread_mut;

#[macro_export]
macro_rules! set_pc {
    ($this:ident, $new_pc:expr, $max_pc:expr) => ({
        if $new_pc > $max_pc {
            return Err(Error::ProgramCounterOverflow);
        }
        $this.m_pc = $new_pc;
        Ok(())
    })
}
pub (crate) use set_pc;

#[macro_export]
macro_rules! current_frame_mut {
    ($this:ident) => ({
        let len = $this.m_stack.len(); 
        &mut $this.m_stack[len - 1]
    })
}
pub (crate) use current_frame_mut;

#[macro_export]
macro_rules! resolve_class_reference {
    ($this_loaded_classes:expr, $reference:expr) => ({
        if !$this_loaded_classes.contains_key($reference) {
            JVM::load_class_file(&mut $this_loaded_classes, $reference)?;
            // TODO: init classes
        }
        let rc = $this_loaded_classes.get($reference).unwrap();
        let new_rc = Rc::clone(rc);
        drop(rc);
        Ok(new_rc)
    })
}
pub (crate) use resolve_class_reference;