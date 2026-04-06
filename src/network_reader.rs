use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{NetworkReader, errors::ReadError, read::AllProcInfo};

#[derive(Debug, Clone)]
pub struct NetworkEntry {
    pub bytes: u64,
    pub errors: u64,
    pub dropped: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub interface: String,
    pub transmit: NetworkEntry,
    pub receive: NetworkEntry,
}

impl From<HashMap<String, String>> for NetworkInfo {
    fn from(value: HashMap<String, String>) -> Self {
        Self {
            interface: value.get("interface").unwrap().to_string(),
            transmit: NetworkEntry {
                bytes: access_raw_network(&value, "transmit_bytes"),
                errors: access_raw_network(&value, "transmit_errs"),
                dropped: access_raw_network(&value, "transmit_drop"),
            },
            receive: NetworkEntry {
                bytes: access_raw_network(&value, "receive_bytes"),
                errors: access_raw_network(&value, "receive_errs"),
                dropped: access_raw_network(&value, "receive_drop"),
            },
        }
    }
}

fn access_raw_network(value: &HashMap<String, String>, key: &str) -> u64 {
    value
        .get(key)
        .unwrap_or_else(|| panic!("{}", (key.to_string() + " to be present")))
        .parse::<u64>()
        .unwrap()
}

pub struct ProcNetReader {}

impl NetworkReader for ProcNetReader {
    fn new() -> Result<Self, crate::errors::InitError>
    where
        Self: Sized,
    {
        Ok(ProcNetReader {})
    }

    fn proc_to_network(
        self,
        info: &AllProcInfo,
    ) -> Result<HashMap<i32, Vec<NetworkInfo>>, ReadError> {
        // building header struct
        let parsed_header = build_header();

        let mut res = HashMap::with_capacity(info.procs.len());

        for pid in info.procs.keys() {
            let path = build_net_dev_path(pid);
            // TODO handle not found (ENOENT)
            let handle = match File::open(&path) {
                Ok(handle) => handle,
                Err(e) => {
                    // sometimes a file won't be populated in procfs
                    println!("{:?} {e}", &path);
                    continue;
                }
            };

            let reader = BufReader::new(handle);
            // skip header since they are the same across processes
            let lines = reader.lines().skip(2);
            let mut interfaces = Vec::new();
            for line in lines {
                let info = parse_proc_net(&parsed_header, line.unwrap());
                interfaces.push(info);
            }
            res.insert(*pid, interfaces);
        }

        for (pid, interfaces) in &res {
            println!("{pid}");
            for interface in interfaces {
                println!("\t{:?}", interface);
            }
        }
        Ok(res)
    }
}

fn build_header() -> Vec<String> {
    let global_net_path = PathBuf::from("/proc/net/dev");
    let global_handle = File::open(global_net_path).unwrap();
    // TODO should I use the tokio apis?
    let buffered = BufReader::new(global_handle);

    // skip first line of header
    // looks like (should be inter-face but it's separated by newline)
    // Inter-|   Receive                                                |  Transmit
    let mut lines = buffered.lines().skip(1);
    // consume second line from iterator
    //  face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    let header = lines.next().unwrap().unwrap();

    // skip first header split
    let parsed_header: Vec<String> = header
        .split("|")
        .skip(1)
        .enumerate()
        .flat_map(|(index, header)| parse_headings(index, header))
        .collect();
    parsed_header
}

fn parse_proc_net(header: &[String], data_line: String) -> NetworkInfo {
    let mut data_iter = data_line.split_whitespace();
    // all interfaces end with a colon which we're not interested in
    let interface = data_iter.next().unwrap().replace(":", "");

    let mut parsed_data: HashMap<String, String> = header
        .iter()
        .zip(data_iter)
        .map(|(heading, data)| (heading.to_owned(), data.to_string()))
        .collect();
    parsed_data.insert("interface".to_string(), interface);
    parsed_data.into()
}

// TODO add result or better yet bounds checking
fn parse_headings(index: usize, header: &str) -> Vec<String> {
    let prefix = if index == 0 {
        "receive".to_string()
    } else if index == 1 {
        "transmit".to_string()
    } else {
        panic!("shouldn't be more than 2 headings")
    };

    header
        .split_whitespace()
        .map(|entry| prefix.clone() + "_" + entry)
        .collect()
}

fn build_net_dev_path(pid: &i32) -> PathBuf {
    let mut net_path = PathBuf::from("/proc");
    net_path.push(pid.to_string());
    net_path.push("net/dev");
    net_path
}
