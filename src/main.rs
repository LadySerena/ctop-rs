use proc_bindings::Procfs;

#[cfg(target_os = "linux")]
mod proc_bindings;

#[cfg(target_os = "linux")]
fn main() {
    use proc_bindings::pids_item;

    let mut getter = Procfs::new(vec![
        pids_item::PIDS_CGROUP_V,
        pids_item::PIDS_ID_PID,
        pids_item::PIDS_CMDLINE_V,
    ])
    .unwrap();

    getter.scan_pids();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("warning this project will not build on non linux systems")
}
