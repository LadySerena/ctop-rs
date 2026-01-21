use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use nix::errno::Errno;

use super::pids_item;

#[derive(Debug)]
pub enum InitError {
    EmptyPointer,
    LibProcError(LibProcError),
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
        }
    }
}

#[derive(Debug)]
pub enum ReadError {
    InvalidFieldError(InvalidFieldError),
    LibProcError(LibProcError),
}

impl Error for ReadError {}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::InvalidFieldError(invalid_field_error) => invalid_field_error.fmt(f),
            ReadError::LibProcError(lib_proc_error) => lib_proc_error.fmt(f),
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
