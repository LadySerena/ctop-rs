use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use containerd_client::tonic;
use nix::errno::Errno;

use super::pids_item;

#[derive(Debug)]
pub enum InitError {
    EmptyPointer,
    LibProcError(LibProcError),
    ContainerClientInit(tonic::transport::Error),
}

impl Error for InitError {}
impl Display for InitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InitError::EmptyPointer => {
                write!(f, "pointer option was empty, expected non null")
            }
            InitError::LibProcError(lib_proc_error) => {
                write!(f, "{lib_proc_error}")
            }
            InitError::ContainerClientInit(err) => write!(f, "{err}"),
        }
    }
}

#[derive(Debug)]
pub enum ReadError {
    InvalidField(InvalidFieldError),
    LibProc(LibProcError),
    MissingItem(pids_item),
}

impl Error for ReadError {}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::InvalidField(invalid_field_error) => invalid_field_error.fmt(f),
            ReadError::LibProc(lib_proc_error) => lib_proc_error.fmt(f),
            ReadError::MissingItem(pids_item) => {
                write!(f, "process info missing {pids_item:?}")
            }
        }
    }
}

#[derive(Debug)]
pub struct InvalidFieldError {
    pub field: pids_item,
}

impl Error for InvalidFieldError {}

impl Display for InvalidFieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unsupported field supplied {:?}", self.field)
    }
}

#[derive(Debug)]
pub struct LibProcError {
    pub err: Errno,
}

impl Display for LibProcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "libproc2 returned {0}", self.err)
    }
}
