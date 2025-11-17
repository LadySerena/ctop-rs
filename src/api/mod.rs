use std::ptr::null_mut;

use nix::errno::Errno;

use crate::api::c_symbols::{
    fatal_proc_unmounted, pids_fetch, pids_info, pids_result__bindgen_ty_1, procps_pids_new,
    procps_pids_reap,
};

mod c_symbols {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use c_symbols::pids_item;
pub use c_symbols::pids_item::*;

pub struct Pid_Counts {
    pub total: i32,
    pub running: i32,
    pub sleeping: i32,
    pub stopped: i32,
    pub zombied: i32,
    pub other: i32,
}

impl Pid_Counts {
    fn from_ptr(ptr: *mut c_symbols::pids_counts) -> Self {
        let de_ref = unsafe { *ptr };
        Pid_Counts {
            total: de_ref.total,
            running: de_ref.running,
            sleeping: de_ref.sleeping,
            stopped: de_ref.stopped,
            zombied: de_ref.zombied,
            other: de_ref.other,
        }
    }
}

fn read_from_union(result: pids_result__bindgen_ty_1) {
    unsafe {
        match result {
            pids_result__bindgen_ty_1 { s_ch } => {
                println!("s_ch {}", s_ch)
            }
            pids_result__bindgen_ty_1 { s_int } => {
                println!("{}", s_int)
            }
            pids_result__bindgen_ty_1 { u_int } => {
                println!("{}", u_int)
            }
            pids_result__bindgen_ty_1 { ul_int } => {
                println!("{}", ul_int)
            }
            pids_result__bindgen_ty_1 { ull_int } => {
                println!("{}", ull_int)
            }
            pids_result__bindgen_ty_1 { str_ } => {
                println!("{:#?}", str_)
            }
            pids_result__bindgen_ty_1 { strv } => {
                println!("{:#?}", strv)
            }
            pids_result__bindgen_ty_1 { real } => {
                println!("{:#?}", real)
            }
        };
    };
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

pub fn scan_procs(infos: Vec<pids_item::Type>) -> pids_fetch {
    let mut cloned_vec = infos.clone();
    let container = new(&mut cloned_vec);
    let pids = reap(container);
    // TODO convert pointer bullshit into real struct
    // drop container?
    let counts = Pid_Counts::from_ptr(pids.counts);
    let loop_bound = usize::try_from(counts.total).expect("convert total to usize");
    // pointer arithmetic see ~/Code/ladyserena/ctop/main.c
    for n in 0..loop_bound {
        let stack = unsafe { (*(*pids.stacks.add(n))).head };
        // let result = pids_result__bindgen_ty_1::s_ch;
        // TODO lookup table of pids_item to expected datatype from union
        // see pids_item enum in libproc2/pids.h for info
        // unsafe { (*stack).result.u_int}
        //
        // let key = unsafe { (*stack).item };
        // https://doc.rust-lang.org/reference/items/unions.html#r-items.union.pattern.subpattern
        // TODO maybe don't futz with the key beyond building a map
        // and then implement display???
        let result = unsafe { (*stack).result };
        read_from_union(result);
    }
    pids.to_owned()
}

fn reap(info: &mut pids_info) -> &pids_fetch {
    unsafe {
        let fetch = procps_pids_reap(info, c_symbols::pids_fetch_type::PIDS_FETCH_TASKS_ONLY);
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

fn new(items: &mut [pids_item::Type]) -> &mut pids_info {
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
