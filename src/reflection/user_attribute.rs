use super::{rcall, Type};
use slang_sys as sys;

fn succeeded(result: sys::SlangResult) -> bool {
	result >= 0
}

#[repr(transparent)]
pub struct UserAttribute(sys::SlangReflectionUserAttribute);

impl UserAttribute {
	pub fn name(&self) -> &str {
		let name = rcall!(spReflectionUserAttribute_GetName(self));
		unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() }
	}

	pub fn argument_count(&self) -> u32 {
		rcall!(spReflectionUserAttribute_GetArgumentCount(self))
	}

	pub fn argument_type(&self, index: u32) -> &Type {
		rcall!(spReflectionUserAttribute_GetArgumentType(self, index) as &Type)
	}

	pub fn argument_value_int(&self, index: u32) -> Option<i32> {
		let mut out = 0;
		let result = rcall!(spReflectionUserAttribute_GetArgumentValueInt(
			self, index, &mut out
		));

		succeeded(result).then(|| out)
	}

	pub fn argument_value_float(&self, index: u32) -> Option<f32> {
		let mut out = 0.0;
		let result = rcall!(spReflectionUserAttribute_GetArgumentValueFloat(
			self, index, &mut out
		));

		succeeded(result).then(|| out)
	}

	pub fn argument_value_string(&self, index: u32) -> Option<&str> {
		let mut len = 0;
		let result = rcall!(spReflectionUserAttribute_GetArgumentValueString(
			self, index, &mut len
		));

		(!result.is_null()).then(|| {
			let slice = unsafe { std::slice::from_raw_parts(result as *const u8, len as usize) };
			std::str::from_utf8(slice).unwrap()
		})
	}
}
