use init::InitError;

#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use crate::proc_bindings::bindings::pids_item;

mod init;

pub fn scan_procfs(items: Vec<pids_item>) -> Result<(), InitError> {
    init::start()?;
    Ok(())
}
