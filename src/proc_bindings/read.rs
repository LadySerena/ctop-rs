use std::{
    ffi::CStr,
    fmt::Display,
    io::{self, Write},
};

use nix::errno::Errno;
use tabwriter::TabWriter;

use super::{
    bindings::{self, pids_fetch_type, pids_info, pids_result},
    errors::{InvalidFieldError, LibProcError, ReadError},
    pids_item,
};

pub struct ProcessInfo {
    procs: Vec<Vec<Value>>,
    items: Vec<pids_item>,
}

impl ProcessInfo {
    pub fn write_table<U: io::Write>(&self, w: U) -> io::Result<()> {
        let mut tw = TabWriter::new(w);
        // writing the header
        // TODO figure out why I can't call items.join("\t")
        write!(tw, "{:?}", self.items.first().unwrap())?;
        for item in &self.items[1..] {
            write!(tw, "\t{item:?}")?;
        }
        writeln!(tw).unwrap();

        // write out processes
        let output = &self.procs;
        for process in output {
            write!(tw, "{}", process.first().unwrap())?;
            for info in &process[1..] {
                write!(tw, "\t{info}")?
            }
            writeln!(tw).unwrap();
        }
        tw.flush()
    }
}

impl IntoIterator for ProcessInfo {
    type Item = Vec<Value>;

    type IntoIter = <Vec<Vec<Value>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.procs.into_iter()
    }
}

#[derive(Debug)]
pub enum Value {
    Char(i8),
    Int32(i32),
    Str(String),
    Uint32(u32),
}
impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::Char(value)
    }
}
impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Uint32(value)
    }
}
impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int32(value)
    }
}

impl From<*mut i8> for Value {
    fn from(value: *mut i8) -> Self {
        let str = unsafe { CStr::from_ptr(value).to_string_lossy().to_string() };
        Self::Str(str)
    }
}

impl From<*mut *mut i8> for Value {
    fn from(value: *mut *mut i8) -> Self {
        unsafe { Value::from(*value) }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Value::Char(c) => c.to_string(),
            Value::Int32(int) => int.to_string(),
            Value::Str(str) => str.to_string(),
            Value::Uint32(u) => u.to_string(),
        };
        write!(f, "{output}")
    }
}

pub unsafe fn scan_processes(
    info_pointer: *mut pids_info,
    items: &[pids_item],
) -> Result<ProcessInfo, ReadError> {
    let fetch = bindings::procps_pids_reap(info_pointer, pids_fetch_type::PIDS_FETCH_TASKS_ONLY);
    if fetch.is_null() {
        return Err(ReadError::LibProcError(LibProcError { err: Errno::last() }));
    }
    let loop_bound = usize::try_from((*(*fetch).counts).total).expect("convert total to usize");
    let mut process_info: Vec<Vec<Value>> = Vec::with_capacity(loop_bound);
    for n in 0..loop_bound {
        let stack = unsafe { (*(*(*fetch).stacks.add(n))).head };
        let mut entry: Vec<Value> = Vec::with_capacity(items.len());
        for i in 0..items.len() {
            let inner = *stack.add(i);
            match read_from_union(inner) {
                Ok(value) => entry.push(value),
                Err(err) => return Err(ReadError::InvalidFieldError(err)),
            };
        }
        process_info.push(entry);
    }

    Ok(ProcessInfo {
        procs: process_info,
        items: items.to_vec(),
    })
}
pub fn read_from_union(result: pids_result) -> Result<Value, InvalidFieldError> {
    match result.item {
        pids_item::PIDS_CGNAME
        | pids_item::PIDS_CGROUP
        | pids_item::PIDS_CMD
        | pids_item::PIDS_CMDLINE
        | pids_item::PIDS_ENVIRON
        | pids_item::PIDS_ID_EGROUP
        | pids_item::PIDS_ID_EUSER
        | pids_item::PIDS_ID_RGROUP
        | pids_item::PIDS_SD_SLICE
        | pids_item::PIDS_SD_UNIT
        | pids_item::PIDS_SD_UUNIT => unsafe { Ok(result.result.str_.into()) },
        pids_item::PIDS_CGROUP_V | pids_item::PIDS_CMDLINE_V | pids_item::PIDS_ENVIRON_V => unsafe {
            Ok(result.result.strv.into())
        },
        pids_item::PIDS_ID_EGID
        | pids_item::PIDS_ID_EUID
        | pids_item::PIDS_ID_FGID
        | pids_item::PIDS_ID_RGID => unsafe { Ok(result.result.u_int.into()) },
        pids_item::PIDS_ID_PID | pids_item::PIDS_ID_PPID | pids_item::PIDS_ID_TGID => unsafe {
            Ok(result.result.s_int.into())
        },
        // TODO I don't love that we can't give the user errors at compile time - @LadySerena
        _ => Err(InvalidFieldError { field: result.item }),
    }
}
