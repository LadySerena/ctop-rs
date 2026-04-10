use std::collections::HashMap;
use std::io::{self, Write};

use tabwriter::TabWriter;

use crate::{
    ContainerMeta, NetworkInfo, pids_item,
    read::{AllProcInfo, ProcEntry},
};

#[derive(Debug)]
pub struct SuperInfo {
    items: Vec<pids_item>,
    processes: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    proc: ProcEntry,
    container: Option<ContainerMeta>,
    net: Option<Vec<NetworkInfo>>,
}

impl SuperInfo {
    pub fn write_table<U: io::Write>(&self, w: U) -> io::Result<()> {
        // TODO add new columns for container meta and network info
        let mut tw = TabWriter::new(w);
        self.write_header(&mut tw)?;
        for entry in &self.processes {
            entry.write_line(&mut tw)?;
        }
        tw.flush()
    }

    fn write_header<U: io::Write>(&self, w: &mut TabWriter<U>) -> io::Result<()> {
        write!(w, "{:?}", self.items.first().unwrap())?;
        for item in &self.items[1..] {
            write!(w, "\t{item:?}")?;
        }
        // container columns
        write!(w, "\tname\tnamespace")?;
        // write out network stats and sockets
        write!(w, "\treceive_bytes\ttransmit_bytes\tsocket_count")?;
        writeln!(w)
    }
}

impl Entry {
    fn write_line<U: io::Write>(&self, w: &mut TabWriter<U>) -> io::Result<()> {
        write!(w, "{}", self.proc.stat.first().unwrap())?;
        for info in &self.proc.stat[1..] {
            write!(w, "\t{info}")?
        }
        // container data if available
        match &self.container {
            Some(c) => write!(w, "\t{}\t{}", c.name, c.namespace)?,
            None => write!(w, "\tN/A\tN/A")?,
        }
        match &self.net {
            Some(n) => {
                let receive_bytes: u64 = n.iter().map(|i| i.receive.bytes).sum();
                let transmit_bytes: u64 = n.iter().map(|i| i.transmit.bytes).sum();
                let socket = self.proc.socket_count;
                write!(w, "\t{receive_bytes}\t{transmit_bytes}\t{socket}")?
            }
            None => write!(w, "\tN/A\tN/A\tN/a")?,
        }
        writeln!(w)
    }
}

pub fn join(
    procs: AllProcInfo,
    containers: HashMap<i32, ContainerMeta>,
    net: HashMap<i32, Vec<NetworkInfo>>,
) -> SuperInfo {
    let mut res: Vec<Entry> = Vec::with_capacity(procs.procs.len());
    // iterate over containers since that will always be a subset of all processes
    for (pid, proc) in procs.procs {
        // not every process is containerized
        let container = containers.get(&pid).cloned();
        // sometimes you can't read the proc/pid/network/dev
        // idk it's a pseudo filesystem *shrug*
        let net = net.get(&pid).cloned();
        res.push(Entry {
            proc,
            container,
            net,
        });
    }
    SuperInfo {
        items: procs.items.clone(),
        processes: res,
    }
}
