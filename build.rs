extern crate bindgen;

use std::env;
use std::path::Path;
use std::path::PathBuf;
//use std::ffi::OsString;

fn main() {

	println!("Hello, build!");
	let inc_var = env::var("HDF5_INCLUDE").expect("Could not find HDF5_INCLUDE");
	let inc_path = Path::new(&inc_var);
	println!("{:?}", inc_path);
	let lib_var = env::var("HDF5_LIB").expect("Could not find HDF5_LIB");
	let lib_path = Path::new(&lib_var);
	println!("{:?}", lib_path);
	println!("cargo:include={:?}", inc_path);
	println!("cargo:rustc-link-search=native={:?}", lib_path);
	//cc::Build::new()
	//	.file("src/hdf5_wrapper.c")
	//	.include(inc_path)
	//	.include(lib_path)
	//	.include("src")
	//	.compile("my_hdf5");
	let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("hdf5.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}