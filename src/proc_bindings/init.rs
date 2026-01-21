use std::{
    error::Error,
    fmt::Display,
    ptr::{self, null_mut},
};

use nix::errno::Errno;

use super::bindings::{
    fatal_proc_unmounted, pids_info, pids_item, procps_pids_new, procps_pids_unref,
};

#[derive(Debug)]
pub enum InitError {
    ErrnoRaised(Errno),
    EmptyPointer,
}

impl Error for InitError {}
impl Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitError::ErrnoRaised(errno) => {
                write!(f, "libproc2 returned {errno}")
            }
            InitError::EmptyPointer => {
                write!(f, "pointer option was empty, expected non null")
            }
        }
    }
}

pub fn start() -> Result<(), InitError> {
    let mut empty_item = vec![pids_item::PIDS_noop];
    let test_allocation = new(&mut empty_item)?;
    (unsafe { verify_mounted_proc(test_allocation) })?;
    unsafe { unref(test_allocation) };
    Ok(())
}

pub fn new(items: &mut [pids_item]) -> Result<&mut pids_info, InitError> {
    let mut info: *mut pids_info = null_mut();
    unsafe {
        let info_ptr: *mut *mut pids_info = &mut info;
        let res = procps_pids_new(
            info_ptr,
            items.as_mut_ptr(),
            items.len().try_into().unwrap(),
        );

        if res < 0 {
            let parsed_errno = Errno::from_raw(-res);
            return Err(InitError::ErrnoRaised(parsed_errno));
        };

        let return_option = info.as_mut();
        match return_option {
            Some(pointer) => Ok(pointer),
            None => Err(InitError::EmptyPointer),
        }
    }
}

pub unsafe fn unref(info: &mut pids_info) {
    let p: *mut *mut pids_info = &mut ptr::from_mut(info);

    procps_pids_unref(p);
}
unsafe fn verify_mounted_proc(info: &mut pids_info) -> Result<(), InitError> {
    let res = fatal_proc_unmounted(info, 1);
    if res.is_null() {
        return Err(InitError::EmptyPointer);
    }
    Ok(())
}
