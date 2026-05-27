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
    // TODO whether to translate this
    // also need to sort out libc nonsense for _SC_CLK_TCK
    // google says I can use the libc crate, but during link time point to musl libc
    // then we get one static binary :3
    Starttime(u128),
    /// virtual memory size (bytes) (stat)
    Vsize(u64),
    /// parent PID (stat)
    Rss(i64),
    /// parent PID (stat)
    Rsslim(u64),
    /// parent PID (stat)
    DelayacctBlkioTicks(u128),
    /// parent PID (stat)
    GuestTime(u64),
    /// parent PID (stat)
    CguestTime(i64),
    /// parent PID (cgroup)
    CgroupName(String),
    /// (cmdline)
    CmdLine(String),
    /// (io)
    ReadBytes(i64),
    /// (io)
    WriteBytes(i64),
    /// (io)
    SysCallRead(i64),
    /// (io)
    SysCallWrite(i64),
}
