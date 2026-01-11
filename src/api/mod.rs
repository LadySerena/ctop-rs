use std::{collections::HashMap, ptr::null_mut};

use nix::errno::Errno;

use crate::api::{
    bindings::{
        fatal_proc_unmounted, pids_counts, pids_fetch, pids_fetch_type, pids_info, procps_pids_new,
        procps_pids_reap,
    },
    result_parsing::{read_from_union, PidResult},
};

#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
#[allow(dead_code)]
mod result_parsing;

mod time;

pub use bindings::pids_item;

pub struct PidEntry {
    pub pid: i32,
    pub info: HashMap<pids_item, PidResult>,
}

pub struct PidCounts {
    pub total: i32,
    pub running: i32,
    pub sleeping: i32,
    pub stopped: i32,
    pub zombied: i32,
    pub other: i32,
}

impl PidCounts {
    fn from_ptr(ptr: *mut pids_counts) -> Self {
        let de_ref = unsafe { *ptr };
        PidCounts {
            total: de_ref.total,
            running: de_ref.running,
            sleeping: de_ref.sleeping,
            stopped: de_ref.stopped,
            zombied: de_ref.zombied,
            other: de_ref.other,
        }
    }
}

/// Verifies that /proc is mounted and readable
/// # Panics
/// per libproc2's docs [1] this call will result in a panic to the caller,
/// thus we will not return an error.
///
/// [1]: https://man.archlinux.org/man/procps_pids.3.en
pub fn verify_mounted_proc() {
    unsafe { fatal_proc_unmounted(null_mut(), 0) };
}

pub fn scan_procs(mut items: Vec<pids_item>) -> Vec<PidEntry> {
    let item_len = items.len();
    let container = new(&mut items);
    let pids = reap(container);
    let counts = PidCounts::from_ptr(pids.counts);
    let loop_bound = usize::try_from(counts.total).expect("convert total to usize");

    let mut list: Vec<PidEntry> = Vec::with_capacity(loop_bound);
    // pointer arithmetic
    // each pid has a stack
    // each stack contains the items from procfs as defined by items variable
    for n in 0..loop_bound {
        let stack = unsafe { (*(*pids.stacks.add(n))).head };
        let mut data_hash: HashMap<pids_item, PidResult> = HashMap::with_capacity(item_len);
        for i in 0..item_len {
            let inner = unsafe { *stack.add(i) };
            let key: pids_item = inner.item;
            let result = inner.result;
            let data = read_from_union(key, result);
            data_hash.insert(key, data);
        }
        list.push(PidEntry {
            pid: 0,
            info: data_hash,
        });
    }
    list
}

fn reap(info: &mut pids_info) -> &pids_fetch {
    unsafe {
        let fetch = procps_pids_reap(info, pids_fetch_type::PIDS_FETCH_TASKS_ONLY);
        if fetch.is_null() {
            panic!("error with reading proc: {}", Errno::last().desc())
        }
        match fetch.as_ref() {
            Some(valid) => valid,
            None => {
                panic!("convert pointer to reference")
            }
        }
    }
}

fn new(items: &mut [pids_item]) -> &mut pids_info {
    let mut info: *mut pids_info = std::ptr::null_mut();
    unsafe {
        let info_ptr: *mut *mut pids_info = &mut info;
        let res = procps_pids_new(
            info_ptr,
            items.as_mut_ptr(),
            items.len().try_into().unwrap(),
        );
        // error happened
        if res < 0 {
            // man page says they return the inverse of well known errno values
            // hence negate the value
            let parsed_errno = Errno::from_raw(-res);
            // TODO don't panic
            panic!("error with allocation: {}", parsed_errno.desc())
        }

        info.as_mut().unwrap()
    }
}
