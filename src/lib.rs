use std::{collections::HashMap, future::Future};

pub use crate::container_meta_reader::ContainerMeta;
pub use crate::network_reader::NetworkInfo;
pub use crate::network_reader::ProcNetReader;
pub use crate::proc_reader::Procfs;
pub use bindings::pids_item;
pub use container_meta_reader::ContainerdReader;
use errors::{InitError, ReadError};
pub use join::join;
use read::AllProcInfo;

#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
mod container_meta_reader;
mod errors;
mod init;
mod join;
mod network_reader;
mod proc_reader;
mod read;
pub mod util;

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
        info: &AllProcInfo,
    ) -> impl Future<Output = Result<HashMap<i32, ContainerMeta>, ReadError>>;
}

pub trait NetworkReader {
    fn new() -> Result<Self, InitError>
    where
        Self: Sized;

    fn proc_to_network(
        self,
        info: &AllProcInfo,
    ) -> Result<HashMap<i32, Vec<NetworkInfo>>, ReadError>;
}
