use api::verify_mounted_proc;

use crate::api::{pids_item, scan_procs};

pub mod api;

// TODO parse out cpu tics to percentage of uptime
// https://stackoverflow.com/questions/16726779/how-do-i-get-the-total-cpu-usage-of-an-application-from-proc-pid-stat

fn main() {
    verify_mounted_proc();
    let proc_fields = vec![
        pids_item::PIDS_TICS_ALL,
        pids_item::PIDS_TICS_USER,
        pids_item::PIDS_TICS_SYSTEM,
        pids_item::PIDS_TICS_BEGAN,
        pids_item::PIDS_TIME_ELAPSED,
        pids_item::PIDS_CMD,
        pids_item::PIDS_ID_PID,
        pids_item::PIDS_CGROUP,
        pids_item::PIDS_CMDLINE_V,
    ];
    let output = scan_procs(proc_fields);
    for process in output.iter() {
        println!("{:#?} {:#?}", process.pid, process.info)
    }
    println!("nothing blew up");
}
