// refactoring into a composite pattern
// common ops
// format specifier
// for presentation is the field visible in output

pub struct PidItem {
    format_specifier: String,
    display_visibility: bool,
}

/// Enum containing fields from proc_pid_stat
/// for more information consult `man proc_pid_stat`
pub enum PidStatItems {
    /// Process id (stat) i32
    Pid,
    /// process state (stat) string
    State,
    /// parent PID (stat) i32
    Ppid,
    /// parent group (stat) i32
    Pgrp,
    /// minor page fault (no disk reload) (stat) u64
    Minflt,
    /// child minor page fault (stat) u64
    Cminflt,
    /// Major fault (reload page from disk) (stat) u64
    Majflt,
    /// child Major fault (reload page from disk) (stat) u64
    Cmajflt,
    /// user mode time (stat) u64
    Utime,
    /// kernel mode time (stat) u64
    Stime,
    /// child user mode time (stat) i64
    Cutime,
    /// child kernel mode time (stat) i64
    Cstime,
    /// number of OS threads (stat) i64
    NumThreads,
    /// Time process started after boot (clock ticks) (stat) u128
    Starttime,
    /// virtual memory size (bytes) (stat) u64
    Vsize,
    /// resident set size (number of pages) (stat) i64
    Rss,
    /// soft limit in bytes of RSS for process (stat) u64
    Rsslim,
    /// block i/o delays clock ticks (stat) u128
    DelayacctBlkioTicks,
    /// number of clock ticks a guest VM ran on a virtual CPU(stat) u64
    GuestTime,
    /// guest time of process's children (stat) i64
    CguestTime,
    /// cgroup name (cgroup) string
    CgroupName,
    /// (cmdline) string
    CmdLine,
}

/// enum for items in proc_pid_io
pub enum PidIOItems {
    /// (io) i64
    ReadBytes,
    /// (io) i64
    WriteBytes,
    /// (io) i64
    SysCallRead,
    /// (io) i64
    SysCallWrite,
}

/// enum containing identifiers for /proc/net/dev
pub enum PidNetDevItems {
    Bytes(u64),
    Packets(u64),
    Dropped(u64),
}

/// enum containing /proc/pid/*/cgroup
pub enum PidCgroup {
    Cgroup(String),
}
