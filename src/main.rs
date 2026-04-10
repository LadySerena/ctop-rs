use clap::Parser;
use ctop_rs::{
    App, ContainerMetaReader, ContainerdReader, NetworkReader, ProcNetReader, ProcReader, Procfs,
    pids_item,
};
use std::io::stdout;

use ctop_rs::join;

#[tokio::main]
async fn main() {
    let cli_opts = App::parse();
    let items = vec![
        pids_item::PIDS_ID_PID,
        pids_item::PIDS_CGROUP_V,
        pids_item::PIDS_TICS_ALL,
    ];
    let getter = Procfs::new(items.clone()).unwrap();
    let output = getter.scan_pids().unwrap();
    // TODO make configurable
    let meta = ContainerdReader::new(cli_opts.containerd_socket.to_string_lossy().to_string())
        .await
        .unwrap();
    let mapping = meta.proc_to_container(&output).await.unwrap();

    let net_reader = ProcNetReader::new().unwrap();
    let net = net_reader.proc_to_network(&output).unwrap();
    let joined = join(output, mapping, net);
    joined.write_table(stdout()).unwrap();
}
