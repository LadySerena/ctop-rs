use std::future::Future;

pub use bindings::pids_item;
use container_meta_reader::ContainerMeta;
use errors::{InitError, ReadError};
use read::ProcessInfo;

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
pub mod proc_reader;
mod read;

pub trait ProcReader {
    fn new(items: Vec<pids_item>) -> Result<Self, InitError>
    where
        Self: Sized;

    fn scan_pids(&self) -> Result<ProcessInfo, ReadError>;
}

pub trait ContainerMetaReader {
    fn new(runtime_endpoint: String) -> impl Future<Output = Result<Self, InitError>>
    where
        Self: Sized;
    fn proc_to_container(
        self,
        info: ProcessInfo,
    ) -> impl Future<Output = Result<Vec<ContainerMeta>, ReadError>>;
}
