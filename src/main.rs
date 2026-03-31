use ctop_rs::{network_reader::ProcNetReader, pids_item};
#[cfg(target_os = "linux")]
use std::io::stdout;
#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    use ctop_rs::{
        container_meta_reader::ContainerdReader, proc_reader::Procfs, ContainerMetaReader,
        NetworkReader, ProcReader,
    };

    let items = vec![
        pids_item::PIDS_ID_PID,
        pids_item::PIDS_CGROUP_V,
        pids_item::PIDS_TICS_ALL,
    ];
    let getter = Procfs::new(items.clone()).unwrap();
    let output = getter.scan_pids().unwrap();
    output.write_table(stdout()).unwrap();
    // TODO make configurable
    let meta = ContainerdReader::new("/proc/3820/root/run/containerd/containerd.sock".to_string())
        .await
        .unwrap();
    let mapping = meta.proc_to_container(output.clone()).await.unwrap();
    for meta in mapping {
        println!("{meta:?}");
    }

    let net_reader = ProcNetReader::new().unwrap();
    net_reader.proc_to_network(output).unwrap();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("warning this project will not build on non linux systems")
}
