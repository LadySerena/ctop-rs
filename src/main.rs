use ctop_rs::pids_item;
#[cfg(target_os = "linux")]
use std::io::stdout;
#[cfg(target_os = "linux")]
fn main() {
    use ctop_rs::{proc_reader::Procfs, ProcReader};

    let items = vec![
        pids_item::PIDS_ID_PID,
        pids_item::PIDS_SD_SLICE,
        pids_item::PIDS_CGROUP_V,
        pids_item::PIDS_TICS_ALL,
    ];
    let getter = Procfs::new(items.clone()).unwrap();
    let output = getter.scan_pids().unwrap();
    output.write_table(stdout()).unwrap();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("warning this project will not build on non linux systems")
}
