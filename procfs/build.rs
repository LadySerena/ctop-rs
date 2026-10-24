use std::{env, path::PathBuf};

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Operating system is unsupported see [docs/local-dev.md]")
}

#[cfg(target_os = "linux")]
fn main() {
    let library = pkg_config::probe_library("libproc2").unwrap();

    let bindings = bindgen::Builder::default()
        .clang_args(
            library
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .clang_args(library.libs.iter().map(|lib| format!("-l{lib}")))
        .clang_arg("-fparse-all-comments")
        .clang_arg("-fretain-comments-from-system-headers")
        .rustified_non_exhaustive_enum("pids_item|pids_fetch_type")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
