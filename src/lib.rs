use std::{collections::HashMap, future::Future};

pub use bindings::pids_item;
use container_meta_reader::ContainerMeta;
use errors::{InitError, ReadError};
use read::AllProcInfo;

use crate::network_reader::NetworkInfo;

#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub mod container_meta_reader;
mod errors;
mod init;
pub mod network_reader;
pub mod proc_reader;
mod read;
pub trait ProcReader {
    fn new(items: Vec<pids_item>) -> Result<Self, InitError>
    where
        Self: Sized;

    fn scan_pids(&self) -> Result<AllProcInfo, ReadError>;
}

pub trait ContainerMetaReader {
    fn new(runtime_endpoint: String) -> impl Future<Output = Result<Self, InitError>>
    where
        Self: Sized;
    fn proc_to_container(
        self,
        info: AllProcInfo,
    ) -> impl Future<Output = Result<Vec<ContainerMeta>, ReadError>>;
}

pub trait NetworkReader {
    fn new() -> Result<Self, InitError>
    where
        Self: Sized;

    fn proc_to_network(
        self,
        info: AllProcInfo,
    ) -> Result<HashMap<String, Vec<NetworkInfo>>, ReadError>;
}
