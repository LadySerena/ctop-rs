use std::{cell::RefCell, ptr::null_mut, rc::Rc};

use nix::errno::Errno;

use crate::bindings::{pids_info, pids_item, procps_pids_new};

#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Enum containing fields from proc_pid_stat
/// for more information consult `man proc_pid_*`
#[derive(Clone, Copy)]
pub enum ProcPidItems {
    /// Process id (stat) i32
    Pid,
    /// process state (stat) string
    State,
    /// parent PID (stat) i32
    Ppid,
    /// parent group (stat) i32
    Pgrp,
    /// minor page fault (no disk reload) (stat) u64
    Minflt,
    /// child minor page fault (stat) u64
    Cminflt,
    /// Major fault (reload page from disk) (stat) u64
    Majflt,
    /// child Major fault (reload page from disk) (stat) u64
    Cmajflt,
    /// user mode time (stat) u64
    Utime,
    /// kernel mode time (stat) u64
    Stime,
    /// child user mode time (stat) i64
    Cutime,
    /// child kernel mode time (stat) i64
    Cstime,
    /// number of OS threads (stat) i64
    NumThreads,
    /// Time process started after boot (clock ticks) (stat) u128
    Starttime,
    /// virtual memory size (bytes) (stat) u64
    Vsize,
    /// resident set size (number of pages) (stat) i64
    Rss,
    /// soft limit in bytes of RSS for process (stat) u64
    Rsslim,
    /// block i/o delays clock ticks (stat) u128
    DelayacctBlkioTicks,
    /// number of clock ticks a guest VM ran on a virtual CPU(stat) u64
    GuestTime,
    /// guest time of process's children (stat) i64
    CguestTime,
    /// cgroup name (cgroup) string
    CgroupName,
    /// (cmdline) string
    CmdLine,
    /// (io) i64
    ReadBytes,
    /// (io) i64
    WriteBytes,
    /// (io) i64
    SysCallRead,
    /// (io) i64
    SysCallWrite,
}

impl From<ProcPidItems> for pids_item {
    fn from(value: ProcPidItems) -> Self {
        match value {
            ProcPidItems::Pid => pids_item::PIDS_ID_PID,
            ProcPidItems::State => pids_item::PIDS_STATE,
            ProcPidItems::Ppid => pids_item::PIDS_ID_PPID,
            ProcPidItems::Pgrp => pids_item::PIDS_ID_PGRP,
            ProcPidItems::Minflt => pids_item::PIDS_FLT_MIN,
            ProcPidItems::Cminflt => pids_item::PIDS_FLT_MIN_C,
            ProcPidItems::Majflt => pids_item::PIDS_FLT_MAJ,
            ProcPidItems::Cmajflt => pids_item::PIDS_FLT_MAJ_C,
            ProcPidItems::Utime => pids_item::PIDS_TICS_USER,
            ProcPidItems::Stime => pids_item::PIDS_TICS_SYSTEM,
            ProcPidItems::Cutime => pids_item::PIDS_TICS_USER_C,
            ProcPidItems::Cstime => pids_item::PIDS_TICS_SYSTEM_C,
            ProcPidItems::NumThreads => pids_item::PIDS_NLWP,
            ProcPidItems::Starttime => pids_item::PIDS_TICS_BEGAN,
            ProcPidItems::Vsize => pids_item::PIDS_VSIZE_BYTES,
            ProcPidItems::Rss => pids_item::PIDS_RSS,
            ProcPidItems::Rsslim => pids_item::PIDS_RSS_RLIM,
            ProcPidItems::DelayacctBlkioTicks => pids_item::PIDS_TICS_BLKIO,
            ProcPidItems::GuestTime => pids_item::PIDS_TICS_GUEST,
            ProcPidItems::CguestTime => pids_item::PIDS_TICS_GUEST_C,
            ProcPidItems::CgroupName => pids_item::PIDS_CGROUP_V,
            ProcPidItems::CmdLine => pids_item::PIDS_CMDLINE_V,
            ProcPidItems::ReadBytes => pids_item::PIDS_IO_READ_BYTES,
            ProcPidItems::WriteBytes => pids_item::PIDS_IO_WRITE_BYTES,
            ProcPidItems::SysCallRead => pids_item::PIDS_IO_READ_OPS,
            ProcPidItems::SysCallWrite => pids_item::PIDS_IO_WRITE_OPS,
        }
    }
}

pub struct ProcReader {
    requested_items: Vec<ProcPidItems>,
    stack: Rc<RefCell<*mut pids_info>>,
}

impl ProcReader {
    fn new(items: &[ProcPidItems]) -> Self {
        // convert internal to lib types
        let mut c_items: Vec<pids_item> = items
            .iter()
            .map(|i: &ProcPidItems| ProcPidItems::into(*i))
            .collect();
        let mut info: *mut pids_info = null_mut();
        unsafe {
            let res = procps_pids_new(
                &mut info,
                c_items.as_mut_ptr(),
                c_items.len().try_into().unwrap(),
            );
            if res < 0 {
                let parsed_errno = Errno::from_raw(-res);
                panic!("{}", parsed_errno)
            }
            let stack_ptr = Rc::new(RefCell::new(info));
            ProcReader {
                requested_items: items.to_vec(),
                stack: stack_ptr,
            }
        }
    }
}
