use std::{cell::RefCell, rc::Rc};

use crate::{
    bindings::pids_info,
    errors::{InitError, ReadError},
    init::{self, start, unref},
    pids_item,
    read::scan_processes,
    ProcReader, ProcessInfo,
};

pub struct Procfs {
    process_stack: Rc<RefCell<*mut pids_info>>,
    requested_items: Vec<pids_item>,
}

impl ProcReader for Procfs {
    fn new(mut items: Vec<pids_item>) -> Result<Self, InitError> {
        start()?;
        let stack = init::new(&mut items)?;
        Ok(Procfs {
            process_stack: stack,
            requested_items: items,
        })
    }

    fn scan_pids(&self) -> Result<ProcessInfo, ReadError> {
        let proc_infos =
            unsafe { scan_processes(*self.process_stack.borrow_mut(), &self.requested_items) }?;
        Ok(proc_infos)
    }
}

impl Drop for Procfs {
    fn drop(&mut self) {
        unsafe { unref(*self.process_stack.borrow_mut()) };
    }
}
