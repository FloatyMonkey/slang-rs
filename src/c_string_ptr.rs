use std::ffi::{c_char, CString};

#[derive(Debug)]
#[repr(C)]
pub struct CStringPtr(*mut c_char);

impl From<CString> for CStringPtr {
	fn from(c_string: CString) -> Self {
		Self(c_string.into_raw())
	}
}

impl Drop for CStringPtr {
	fn drop(&mut self) {
		std::mem::drop(unsafe { CString::from_raw(self.0) });
	}
}
