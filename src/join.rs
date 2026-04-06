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
        let tw = TabWriter::new(w);
        self.write_header(tw)?;
        Ok(())
    }

    fn write_header<U: io::Write>(&self, mut w: U) -> io::Result<()> {
        write!(w, "{:?}", self.items.first().unwrap())?;
        for item in &self.items[1..] {
            write!(w, "\t{item:?}")?;
        }
        Ok(())
    }
}

impl Entry {
    pub fn write_line<U: io::Write>(&self, mut w: U) -> io::Result<()> {
        Ok(())
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
