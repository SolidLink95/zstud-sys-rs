use std::env;
use std::path::PathBuf;

// use bindgen;

fn main() {
    // Get the output directory from the environment variable
    let out_dir = env::var("OUT_DIR").expect("Failed to read OUT_DIR environment variable");

    // Generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header("zstd.c")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Set up the C compiler and build the zstd.c file
    let mut builder = cc::Build::new();
    let src_files = ["./zstd.c"];

    builder
        .files(src_files.iter())
        .compile("zstd"); // Compile the C source files

    // Write the generated bindings to the output directory
    let output_path = PathBuf::from(out_dir).join("bindings.rs");
    bindings
        .write_to_file(&output_path)
        .expect("Couldn't write bindings to file");

    println!("cargo:rerun-if-changed=zstd.c"); // Re-run if zstd.c changes
}
