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
    /// Process id (stat)
    Pid(i32),
    /// process state (stat)
    State(String),
    /// parent PID (stat)
    Ppid(i32),
    /// parent group (stat)
    Pgrp(i32),
    /// minor page fault (no disk reload) (stat)
    Minflt(u64),
    /// child minor page fault (stat)
    Cminflt(u64),
    /// Major fault (reload page from disk) (stat)
    Majflt(u64),
    /// child Major fault (reload page from disk) (stat)
    Cmajflt(u64),
    /// user mode time (stat)
    Utime(u64),
    /// kernel mode time (stat)
    Stime(u64),
    /// child user mode time (stat)
    Cutime(i64),
    /// child kernel mode time (stat)
    Cstime(i64),
    /// number of OS threads (stat)
    NumThreads(i64),
    /// Time process started after boot (clock ticks) (stat)
    Starttime(u128),
    /// virtual memory size (bytes) (stat)
    Vsize(u64),
    /// resident set size (number of pages) (stat)
    Rss(i64),
    /// soft limit in bytes of RSS for process (stat)
    Rsslim(u64),
    /// block i/o delays clock ticks (stat)
    DelayacctBlkioTicks(u128),
    /// number of clock ticks a guest VM ran on a virtual CPU(stat)
    GuestTime(u64),
    /// guest time of process's children (stat)
    CguestTime(i64),
    /// cgroup name (cgroup)
    CgroupName(String),
    /// (cmdline)
    CmdLine(String),
}

/// enum for items in proc_pid_io
pub enum PidIOItems {
    /// (io)
    ReadBytes(i64),
    /// (io)
    WriteBytes(i64),
    /// (io)
    SysCallRead(i64),
    /// (io)
    SysCallWrite(i64),
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
