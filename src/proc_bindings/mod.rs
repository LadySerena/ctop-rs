use bindings::pids_info;
use init::{start, unref, InitError};

#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use crate::proc_bindings::bindings::pids_item;

pub struct Procfs {
    process_stack: pids_info,
}

impl Procfs {
    pub fn new(mut items: Vec<pids_item>) -> Result<Self, InitError> {
        start()?;
        let stack = init::new(&mut items)?.to_owned();
        Ok(Procfs {
            process_stack: stack,
        })
    }

    pub fn scan_pids(&mut self) {
        let foo = unsafe {
            bindings::procps_pids_reap(
                &mut self.process_stack,
                bindings::pids_fetch_type::PIDS_FETCH_TASKS_ONLY,
            )
        };
    }
}

impl Drop for Procfs {
    fn drop(&mut self) {
        unsafe { unref(&mut self.process_stack) };
    }
}

mod init;
