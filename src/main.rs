#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use api::verify_mounted_proc;

use crate::api::{
    pids_item_PIDS_CGROUP, pids_item_PIDS_CMD, pids_item_PIDS_ID_PID, pids_item_PIDS_TICS_ALL,
    pids_item_PIDS_TICS_SYSTEM, pids_item_PIDS_TICS_USER, scan_procs,
};

pub mod api;

fn main() {
    verify_mounted_proc();
    let mut proc_fields = [
        pids_item_PIDS_TICS_ALL,
        pids_item_PIDS_TICS_USER,
        pids_item_PIDS_TICS_SYSTEM,
        pids_item_PIDS_CMD,
        pids_item_PIDS_ID_PID,
        pids_item_PIDS_CGROUP,
    ];
    let procs = scan_procs(&mut proc_fields);
    println!("nothing blew up");
}
