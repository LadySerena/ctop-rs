use std::{ffi::CStr, ptr::null_mut};

use nix::errno::Errno;

use crate::api::c_symbols::{
    fatal_proc_unmounted, pids_fetch, pids_info, pids_result__bindgen_ty_1, procps_pids_new,
    procps_pids_reap,
};

mod c_symbols {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use c_symbols::pids_item;

pub struct Pid_Counts {
    pub total: i32,
    pub running: i32,
    pub sleeping: i32,
    pub stopped: i32,
    pub zombied: i32,
    pub other: i32,
}

#[derive(Debug)]
pub enum Pid_result {
    char(i8),
    int32(i32),
    uint32(u32),
    uint64(u64),
    str(String),
    real(f64),
    empty(Option<()>),
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

fn read_from_union(item: pids_item, result: pids_result__bindgen_ty_1) -> Pid_result {
    unsafe {
        match item {
            pids_item::PIDS_noop => Pid_result::empty(None),
            pids_item::PIDS_extra => Pid_result::empty(None),
            pids_item::PIDS_ADDR_CODE_END => todo!(),
            pids_item::PIDS_ADDR_CODE_START => todo!(),
            pids_item::PIDS_ADDR_CURR_EIP => todo!(),
            pids_item::PIDS_ADDR_CURR_ESP => todo!(),
            pids_item::PIDS_ADDR_STACK_START => todo!(),
            pids_item::PIDS_AUTOGRP_ID => todo!(),
            pids_item::PIDS_AUTOGRP_NICE => todo!(),
            pids_item::PIDS_CGNAME => todo!(),
            pids_item::PIDS_CGROUP => {
                let meep = CStr::from_ptr(result.str_);
                Pid_result::str(meep.to_string_lossy().to_string())
            }
            pids_item::PIDS_CGROUP_V => todo!(),
            pids_item::PIDS_CMD => {
                let meep = CStr::from_ptr(result.str_);
                Pid_result::str(meep.to_string_lossy().to_string())
            }
            pids_item::PIDS_CMDLINE => todo!(),
            pids_item::PIDS_CMDLINE_V => todo!(),
            pids_item::PIDS_ENVIRON => todo!(),
            pids_item::PIDS_ENVIRON_V => todo!(),
            pids_item::PIDS_EXE => todo!(),
            pids_item::PIDS_EXIT_SIGNAL => todo!(),
            pids_item::PIDS_FLAGS => todo!(),
            pids_item::PIDS_FLT_MAJ => todo!(),
            pids_item::PIDS_FLT_MAJ_C => todo!(),
            pids_item::PIDS_FLT_MAJ_DELTA => todo!(),
            pids_item::PIDS_FLT_MIN => todo!(),
            pids_item::PIDS_FLT_MIN_C => todo!(),
            pids_item::PIDS_FLT_MIN_DELTA => todo!(),
            pids_item::PIDS_ID_EGID => todo!(),
            pids_item::PIDS_ID_EGROUP => todo!(),
            pids_item::PIDS_ID_EUID => todo!(),
            pids_item::PIDS_ID_EUSER => todo!(),
            pids_item::PIDS_ID_FGID => todo!(),
            pids_item::PIDS_ID_FGROUP => todo!(),
            pids_item::PIDS_ID_FUID => todo!(),
            pids_item::PIDS_ID_FUSER => todo!(),
            pids_item::PIDS_ID_LOGIN => todo!(),
            pids_item::PIDS_ID_PGRP => todo!(),
            pids_item::PIDS_ID_PID => Pid_result::int32(result.s_int),
            pids_item::PIDS_ID_PPID => todo!(),
            pids_item::PIDS_ID_RGID => todo!(),
            pids_item::PIDS_ID_RGROUP => todo!(),
            pids_item::PIDS_ID_RUID => todo!(),
            pids_item::PIDS_ID_RUSER => todo!(),
            pids_item::PIDS_ID_SESSION => todo!(),
            pids_item::PIDS_ID_SGID => todo!(),
            pids_item::PIDS_ID_SGROUP => todo!(),
            pids_item::PIDS_ID_SUID => todo!(),
            pids_item::PIDS_ID_SUSER => todo!(),
            pids_item::PIDS_ID_TGID => todo!(),
            pids_item::PIDS_ID_TID => todo!(),
            pids_item::PIDS_ID_TPGID => todo!(),
            pids_item::PIDS_IO_READ_BYTES => todo!(),
            pids_item::PIDS_IO_READ_CHARS => todo!(),
            pids_item::PIDS_IO_READ_OPS => todo!(),
            pids_item::PIDS_IO_WRITE_BYTES => todo!(),
            pids_item::PIDS_IO_WRITE_CBYTES => todo!(),
            pids_item::PIDS_IO_WRITE_CHARS => todo!(),
            pids_item::PIDS_IO_WRITE_OPS => todo!(),
            pids_item::PIDS_LXCNAME => todo!(),
            pids_item::PIDS_MEM_CODE => todo!(),
            pids_item::PIDS_MEM_CODE_PGS => todo!(),
            pids_item::PIDS_MEM_DATA => todo!(),
            pids_item::PIDS_MEM_DATA_PGS => todo!(),
            pids_item::PIDS_MEM_RES => todo!(),
            pids_item::PIDS_MEM_RES_PGS => todo!(),
            pids_item::PIDS_MEM_SHR => todo!(),
            pids_item::PIDS_MEM_SHR_PGS => todo!(),
            pids_item::PIDS_MEM_VIRT => todo!(),
            pids_item::PIDS_MEM_VIRT_PGS => todo!(),
            pids_item::PIDS_NICE => todo!(),
            pids_item::PIDS_NLWP => todo!(),
            pids_item::PIDS_NS_CGROUP => todo!(),
            pids_item::PIDS_NS_IPC => todo!(),
            pids_item::PIDS_NS_MNT => todo!(),
            pids_item::PIDS_NS_NET => todo!(),
            pids_item::PIDS_NS_PID => todo!(),
            pids_item::PIDS_NS_TIME => todo!(),
            pids_item::PIDS_NS_USER => todo!(),
            pids_item::PIDS_NS_UTS => todo!(),
            pids_item::PIDS_OOM_ADJ => todo!(),
            pids_item::PIDS_OOM_SCORE => todo!(),
            pids_item::PIDS_PRIORITY => todo!(),
            pids_item::PIDS_PRIORITY_RT => todo!(),
            pids_item::PIDS_PROCESSOR => todo!(),
            pids_item::PIDS_PROCESSOR_NODE => todo!(),
            pids_item::PIDS_RSS => todo!(),
            pids_item::PIDS_RSS_RLIM => todo!(),
            pids_item::PIDS_SCHED_CLASS => todo!(),
            pids_item::PIDS_SD_MACH => todo!(),
            pids_item::PIDS_SD_OUID => todo!(),
            pids_item::PIDS_SD_SEAT => todo!(),
            pids_item::PIDS_SD_SESS => todo!(),
            pids_item::PIDS_SD_SLICE => todo!(),
            pids_item::PIDS_SD_UNIT => todo!(),
            pids_item::PIDS_SD_UUNIT => todo!(),
            pids_item::PIDS_SIGBLOCKED => todo!(),
            pids_item::PIDS_SIGCATCH => todo!(),
            pids_item::PIDS_SIGIGNORE => todo!(),
            pids_item::PIDS_SIGNALS => todo!(),
            pids_item::PIDS_SIGPENDING => todo!(),
            pids_item::PIDS_SMAP_ANONYMOUS => todo!(),
            pids_item::PIDS_SMAP_HUGE_ANON => todo!(),
            pids_item::PIDS_SMAP_HUGE_FILE => todo!(),
            pids_item::PIDS_SMAP_HUGE_SHMEM => todo!(),
            pids_item::PIDS_SMAP_HUGE_TLBPRV => todo!(),
            pids_item::PIDS_SMAP_HUGE_TLBSHR => todo!(),
            pids_item::PIDS_SMAP_LAZY_FREE => todo!(),
            pids_item::PIDS_SMAP_LOCKED => todo!(),
            pids_item::PIDS_SMAP_PRV_CLEAN => todo!(),
            pids_item::PIDS_SMAP_PRV_DIRTY => todo!(),
            pids_item::PIDS_SMAP_PRV_TOTAL => todo!(),
            pids_item::PIDS_SMAP_PSS => todo!(),
            pids_item::PIDS_SMAP_PSS_ANON => todo!(),
            pids_item::PIDS_SMAP_PSS_FILE => todo!(),
            pids_item::PIDS_SMAP_PSS_SHMEM => todo!(),
            pids_item::PIDS_SMAP_REFERENCED => todo!(),
            pids_item::PIDS_SMAP_RSS => todo!(),
            pids_item::PIDS_SMAP_SHR_CLEAN => todo!(),
            pids_item::PIDS_SMAP_SHR_DIRTY => todo!(),
            pids_item::PIDS_SMAP_SWAP => todo!(),
            pids_item::PIDS_SMAP_SWAP_PSS => todo!(),
            pids_item::PIDS_STATE => todo!(),
            pids_item::PIDS_SUPGIDS => todo!(),
            pids_item::PIDS_SUPGROUPS => todo!(),
            pids_item::PIDS_TICS_ALL => Pid_result::uint64(result.ull_int),
            pids_item::PIDS_TICS_ALL_C => todo!(),
            pids_item::PIDS_TICS_ALL_DELTA => todo!(),
            pids_item::PIDS_TICS_BEGAN => todo!(),
            pids_item::PIDS_TICS_BLKIO => todo!(),
            pids_item::PIDS_TICS_GUEST => todo!(),
            pids_item::PIDS_TICS_GUEST_C => todo!(),
            pids_item::PIDS_TICS_SYSTEM => Pid_result::uint64(result.ull_int),
            pids_item::PIDS_TICS_SYSTEM_C => todo!(),
            pids_item::PIDS_TICS_USER => Pid_result::uint64(result.ull_int),
            pids_item::PIDS_TICS_USER_C => todo!(),
            pids_item::PIDS_TIME_ALL => todo!(),
            pids_item::PIDS_TIME_ALL_C => todo!(),
            pids_item::PIDS_TIME_ELAPSED => todo!(),
            pids_item::PIDS_TIME_START => todo!(),
            pids_item::PIDS_TTY => todo!(),
            pids_item::PIDS_TTY_NAME => todo!(),
            pids_item::PIDS_TTY_NUMBER => todo!(),
            pids_item::PIDS_UTILIZATION => todo!(),
            pids_item::PIDS_UTILIZATION_C => todo!(),
            pids_item::PIDS_VM_DATA => todo!(),
            pids_item::PIDS_VM_EXE => todo!(),
            pids_item::PIDS_VM_LIB => todo!(),
            pids_item::PIDS_VM_RSS => todo!(),
            pids_item::PIDS_VM_RSS_ANON => todo!(),
            pids_item::PIDS_VM_RSS_FILE => todo!(),
            pids_item::PIDS_VM_RSS_LOCKED => todo!(),
            pids_item::PIDS_VM_RSS_SHARED => todo!(),
            pids_item::PIDS_VM_SIZE => todo!(),
            pids_item::PIDS_VM_STACK => todo!(),
            pids_item::PIDS_VM_SWAP => todo!(),
            pids_item::PIDS_VM_USED => todo!(),
            pids_item::PIDS_VSIZE_BYTES => todo!(),
            pids_item::PIDS_WCHAN_NAME => todo!(),
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

pub fn scan_procs(items: Vec<pids_item>) -> pids_fetch {
    let mut cloned_vec = items.clone();
    let container = new(&mut cloned_vec);
    let pids = reap(container);
    // TODO convert pointer bullshit into real struct
    // drop container?
    let counts = Pid_Counts::from_ptr(pids.counts);
    let loop_bound = usize::try_from(counts.total).expect("convert total to usize");
    // pointer arithmetic see ~/Code/ladyserena/ctop/main.c
    for n in 0..loop_bound {
        let stack = unsafe { (*(*pids.stacks.add(n))).head };
        for i in 0..items.len() {
            let inner = unsafe { *stack.add(i) };
            let key: pids_item = inner.item;
            let result = inner.result;
            let data = read_from_union(key, result);
            println!("{:#?} {:#?}", key, data)
        }
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
