#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use api::verify_mounted_proc;

pub mod api;

fn main() {
    verify_mounted_proc();
    println!("look i didn't blow up!")
}
