use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    // println!("cargo:rerun-if-changed=tauri.conf.json");
    // println!("cargo:warning=Debug: checking build.rs execution");

    let out_dir = env::var("OUT_DIR").unwrap();

    let bindings =
        bindgen::Builder::default()
			// .clang_args(&["-D__FLT16_SUPPORT=0"])
            .header("zstd.c")
            .parse_callbacks(Box::new(CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

    let mut builder = cc::Build::new();

    let src = [
        "./zstd.c",
    ];

    let build = builder
        .files(src.iter());
        //.flag("-Wno-unused-parameter");

    build.compile("zstd");

    let mut output_path = PathBuf::from(out_dir);
    output_path.push("bindings.rs");

    bindings
        .write_to_file(output_path)
        .expect("Couldn't write bindings!");
}
