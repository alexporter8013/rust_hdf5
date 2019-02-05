extern crate libc;
use std::ffi::CString;

extern {
	fn ex_create_hdf5_file(filename: *const libc::c_char) -> libc::c_int;
}

pub fn create_hdf5_file(filename: &str) {
	let fname = CString::new(filename).expect("CString::new failed");
	let result = unsafe { ex_create_hdf5_file(fname.as_ptr()); };
	println!("{:?} created. result: {:?}", fname, result);
	
}

pub type herr_t = libc::c_int;
pub type hid_t = libc::int64_t;

pub fn return_true() -> bool {
		true
}

//pub fn create_hdf5_file()


#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        assert_eq!(return_true(), true);
    }
    #[test]
    fn made_file() {
    	create_hdf5_file("file.h5");
    	assert_eq!(1, 1);
    }
    
}
