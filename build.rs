use std::{env, path::PathBuf};

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
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rustified_enum(".*")
        .generate_comments(true)
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
