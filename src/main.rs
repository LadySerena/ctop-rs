#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use api::verify_mounted_proc;

use crate::api::{pids_item, scan_procs};

pub mod api;

fn main() {
    verify_mounted_proc();
    let proc_fields = vec![
        pids_item::PIDS_TICS_ALL,
        pids_item::PIDS_TICS_USER,
        pids_item::PIDS_TICS_SYSTEM,
        pids_item::PIDS_CMD,
        pids_item::PIDS_ID_PID,
        pids_item::PIDS_CGROUP,
    ];
    scan_procs(proc_fields);
    println!("nothing blew up");
}
