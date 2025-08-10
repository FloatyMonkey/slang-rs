use super::{Type, rcall};
use crate::sys;

#[repr(transparent)]
pub struct UserAttribute(sys::SlangReflectionUserAttribute);

impl UserAttribute {
	pub fn name(&self) -> Option<&str> {
		rcall!(spReflectionUserAttribute_GetName(self) as Option<&str>)
	}

	pub fn argument_count(&self) -> u32 {
		rcall!(spReflectionUserAttribute_GetArgumentCount(self))
	}

	pub fn argument_type(&self, index: u32) -> Option<&Type> {
		rcall!(spReflectionUserAttribute_GetArgumentType(self, index) as Option<&Type>)
	}

	pub fn argument_value_int(&self, index: u32) -> Option<i32> {
		let mut out = 0;
		let result = rcall!(spReflectionUserAttribute_GetArgumentValueInt(
			self, index, &mut out
		));

		crate::succeeded(result).then(|| out)
	}

	pub fn argument_value_float(&self, index: u32) -> Option<f32> {
		let mut out = 0.0;
		let result = rcall!(spReflectionUserAttribute_GetArgumentValueFloat(
			self, index, &mut out
		));

		crate::succeeded(result).then(|| out)
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
