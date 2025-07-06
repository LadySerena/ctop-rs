pub const STAT_PATH: &str = "stat";

/// struct containing members representing /proc/pid/stat.
/// See [man 5 proc_pid_stat](https://man.archlinux.org/man/proc_pid_stat.5.en)
pub struct Stat {
    /// process id (0)
    pid: i32,
    // TASK_COMM_LEN is 16 but /prod/_pid_/stat surrounds _comm_ with parenthesis
    comm: [u8; 18],
    state: [u8; 1],
    ppid: i32,
    user_time: u64,
    kernel_time: u64,
}

impl Stat {}
