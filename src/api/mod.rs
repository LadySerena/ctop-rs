use std::ptr::null_mut;

use nix::errno::Errno;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Pid_Counts {
    pub total: i32,
    pub running: i32,
    pub sleeping: i32,
    pub stopped: i32,
    pub zombied: i32,
    pub other: i32,
}

impl Pid_Counts {
    fn from_ptr(ptr: *mut pids_counts) -> Self {
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

/// Verifies that /proc is mounted and readable
/// # Panics
/// per libproc2's docs [1] this call will result in a panic to the caller,
/// thus we will not return an error.
///
/// [1]: https://man.archlinux.org/man/procps_pids.3.en
pub fn verify_mounted_proc() {
    unsafe { fatal_proc_unmounted(null_mut(), 0) };
}

pub fn scan_procs(infos: &mut [pids_item]) -> &pids_fetch {
    let container = new(infos);
    let pids = reap(container);
    // TODO convert pointer bullshit into real struct
    // drop container?
    let counts = Pid_Counts::from_ptr(pids.counts);
    let loop_bound = usize::try_from(counts.total).expect("convert total to usize");
    // pointer arithmetic see ~/Code/ladyserena/ctop/main.c
    for n in 0..loop_bound {
        let stack = unsafe { (*(*pids.stacks.add(n))).head };
        // TODO lookup table of pids_item to expected datadog
        // see pids_item enum in libproc2/pids.h for info
        // unsafe { (*stack).result.u_int}
    }
    pids
}

fn reap(info: &mut pids_info) -> &pids_fetch {
    unsafe {
        let fetch = procps_pids_reap(info, pids_fetch_type_PIDS_FETCH_TASKS_ONLY);
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

fn new(infos: &mut [pids_item]) -> &mut pids_info {
    let mut info = std::mem::MaybeUninit::<*mut pids_info>::uninit();
    unsafe {
        let res = procps_pids_new(
            info.as_mut_ptr(),
            infos.as_mut_ptr(),
            infos.len().try_into().unwrap(),
        );
        // error happened
        if res < 0 {
            // man page says they return the inverse of well known errno values
            // hence negate the value
            let parsed_errno = Errno::from_raw(-res);
            // TODO don't panic
            panic!("error with allocation: {}", parsed_errno.desc())
        }

        if let Some(stuff) = info.assume_init().as_mut() {
            stuff
        } else {
            panic!("convert pointer to reference")
        }
    }
}
