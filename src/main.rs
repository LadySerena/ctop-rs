pub mod proc;

use std::{
    ffi::OsStr,
    fs::{self},
    io,
    path::PathBuf,
};

use clap::Parser;
use nix::unistd;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = OsStr::new("/proc"))]
    proc_path: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let entries = fs::read_dir(args.proc_path)?.filter_map(|e| e.ok());
    let clock_rate = unistd::sysconf(unistd::SysconfVar::CLK_TCK)
        .unwrap()
        .unwrap();

    println!("{clock_rate}");

    for entry in entries {
        // TODO filter out non /proc/pid dirs
        // TODO parse out stat
        println!("{:?}", entry.path())
    }

    Ok(())
}
