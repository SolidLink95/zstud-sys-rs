use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let bindings =
        bindgen::Builder::default()
            .header("zstd.c")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

    let mut builder = cc::Build::new();

    let src = [
        "./zstd.c",
    ];

    let build = builder
        .files(src.iter())
        .flag("-Wno-unused-parameter");

    build.compile("zstd");

    let mut output_path = PathBuf::from(out_dir);
    output_path.push("bindings.rs");

    bindings
        .write_to_file(output_path)
        .expect("Couldn't write bindings!");
}
