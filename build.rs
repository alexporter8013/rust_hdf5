use std::env;

fn main() {

	println!("Hello, build!");
	let key = "HDF5_INCLUDE";
	match env::var_os(key) {
	    Some(val) => println!("{}: {:?}", key, val),
	    None => println!("{} is not defined in the environment.", key)
	}
	let key = "HDF5_LIB";
	match env::var_os(key) {
	    Some(val) => println!("{}: {:?}", key, val),
	    None => println!("{} is not defined in the environment.", key)
	}
	for (key, value) in env::vars() {
    	println!("{}: {}", key, value);
	}
}