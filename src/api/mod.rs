use std::ptr::null_mut;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Verifies that /proc is mounted and readable
/// # Panics
/// per libproc2's docs [1] this call will result in a panic to the caller,
/// thus we will not return an error.
///
/// [1]: https://man.archlinux.org/man/procps_pids.3.en
pub fn verify_mounted_proc() {
    unsafe { fatal_proc_unmounted(null_mut(), 0) };

    unsafe {
        let mut info = std::mem::MaybeUninit::<*mut pids_info>::uninit();
        let mut items = vec![
            pids_item_PIDS_TICS_ALL,
            pids_item_PIDS_TICS_USER,
            pids_item_PIDS_TICS_SYSTEM,
        ];
        procps_pids_new(
            info.as_mut_ptr(),
            items.as_mut_ptr(),
            items.len().try_into().unwrap(),
        );
    }
}
