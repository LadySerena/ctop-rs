# Helpful docs

These are links I've found to be helpful that I come back to while I'm working
on this project. I hope it'll be helpful for you too.

## Links

- [procps-ng/procps] - upstream repo for libproc2
- [rust ffi patterns] - rust ffi patterns
- [rust refcell] - rust book guide on interior mutability

## Man pages

These man pages usually require you to install additional manpages packages
depending on your distribution. I suggest running the `apropos` commands
suggested below to verify you have the docs installed.

### Manpages of interest

Manpages in this section were found via `apropos proc_pid_` and
`apropos procps`. There are online manpages such as [man.archlinux.org], but I
recommend reading the man pages on your system.

```shell
proc_pid_stat (5)    - status information
proc_pid_statm (5)   - memory usage information
proc_pid_status (5)  - memory usage and status information
...
procps (3)           - API to access system level information in the /proc filesystem
procps_misc (3)      - API for miscellaneous information in the /proc filesystem
procps_pids (3)      - API to access process information in the /proc filesystem
```

[man.archlinux.org]: https://man.archlinux.org/
[procps-ng/procps]: https://gitlab.com/procps-ng/procps
[rust ffi patterns]:
  https://rust-unofficial.github.io/patterns/patterns/ffi/export.html
[rust refcell]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
