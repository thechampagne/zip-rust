pub mod raw;
use std::str;
use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_int;

pub struct Zip {
    zip: *mut raw::zip_t
}

impl Zip {
    pub fn open(
        zipname: &str,
        level: i32,
        mode: char,
    ) ->  Result<Self, &'static str> {
	unsafe {
	    let c_zipname = CString::new(zipname).expect("CString::new failed");
	    let mut err: c_int = 0;
	    let zip = raw::zip_openwitherror(c_zipname.as_ptr() as _,
					     level as _,
					     mode as _,
					     (&mut err) as *mut _);
	    if err != 0 {
		let err_str = raw::zip_strerror(err);
		let err_bytes = CStr::from_ptr(err_str).to_bytes();
		return Err(str::from_utf8_unchecked(err_bytes));
	    }
	    Ok(Self {
		zip: zip
	    })
	}
    }

    pub fn close(&mut self) {
	unsafe {
	    raw::zip_close(self.zip);
	    self.zip = ptr::null_mut();
	}
    }
}
