use nix::errno::Errno;

use crate::api::bindings::{procps_hertz_get, procps_uptime};

fn get_hertz() -> i64 {
    unsafe { procps_hertz_get() }
}

fn get_uptime() -> f64 {
    let mut uptime: f64 = 0.0;
    unsafe {
        let uptime_ptr: *mut f64 = &mut uptime;
        let uptime_err = procps_uptime(uptime_ptr, std::ptr::null_mut());
        if uptime_err < 0 {
            let parsed_error = Errno::from_raw(uptime_err);
            panic!("error with getting uptime: {}", parsed_error.desc())
        }
    };
    uptime
}
