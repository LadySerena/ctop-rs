use crate::{errors::ReadError, read::ProcessInfo, NetworkReader};

pub struct NetworkInfo {
    pub interface: String,
    pub bytes_sent: u64,
    pub bytes_recieved: u64,
}

pub struct ProcNetReader {}

impl NetworkReader for ProcNetReader {
    fn new() -> Result<Self, crate::errors::InitError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn proc_to_network(self, info: ProcessInfo) -> Result<(), ReadError> {
        let proc = info.procs.first().expect("at least 1 proccess");

        let id_index = info.items.iter().find_map(f)
    }
}
