use std::{cell::RefCell, rc::Rc};

use bindings::pids_info;
use errors::{InitError, ReadError};
use init::{start, unref};
use read::{scan_processes, ProcessInfo};

#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod errors;
mod read;
pub use crate::proc_bindings::bindings::pids_item;

pub struct Procfs {
    process_stack: Rc<RefCell<*mut pids_info>>,
    requested_items: Vec<pids_item>,
}

impl Procfs {
    pub fn new(mut items: Vec<pids_item>) -> Result<Self, InitError> {
        start()?;
        let stack = init::new(&mut items)?;
        Ok(Procfs {
            process_stack: stack,
            requested_items: items,
        })
    }

    pub fn scan_pids(&self) -> Result<ProcessInfo, ReadError> {
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

mod init;
