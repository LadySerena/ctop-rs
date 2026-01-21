#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(target_os = "linux")]
mod proc_bindings;

#[cfg(target_os = "linux")]
fn main() {
    proc_bindings::scan_procfs(vec![proc_bindings::pids_item::PIDS_CGROUP_V]).unwrap()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("warning this project will not build on non linux systems")
}
