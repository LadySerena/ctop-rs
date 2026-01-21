use std::{cell::RefCell, ptr::null_mut, rc::Rc};

use super::{
    bindings::{fatal_proc_unmounted, pids_info, pids_item, procps_pids_new, procps_pids_unref},
    errors::{InitError, LibProcError},
};
use nix::errno::Errno;

pub fn start() -> Result<(), InitError> {
    let mut empty_item = vec![pids_item::PIDS_noop];
    let test_allocation = new(&mut empty_item)?;
    (unsafe { verify_mounted_proc(&test_allocation) })?;
    let inner_refcell = Rc::into_inner(test_allocation).ok_or(InitError::EmptyPointer)?;
    unsafe { unref(inner_refcell.into_inner()) };
    Ok(())
}

pub fn new(items: &mut [pids_item]) -> Result<Rc<RefCell<*mut pids_info>>, InitError> {
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
            return Err(InitError::LibProcError(LibProcError { err: parsed_errno }));
        };
        let wrapped_pointer = RefCell::new(info);
        let outer_pointer = Rc::new(wrapped_pointer);
        Ok(outer_pointer)
    }
}

pub unsafe fn unref(mut info: *mut pids_info) {
    let p: *mut *mut pids_info = &mut info;

    procps_pids_unref(p);
}
unsafe fn verify_mounted_proc(
    info: &Rc<RefCell<*mut pids_info>>,
) -> Result<Box<pids_info>, InitError> {
    let pointer: *mut pids_info = *info.borrow_mut();
    let res = fatal_proc_unmounted(pointer, 1);
    if res.is_null() {
        return Err(InitError::EmptyPointer);
    }
    Ok(Box::from_raw(pointer))
}
