use super::{rcall, Type};
use slang_sys as sys;

#[repr(transparent)]
pub struct TypeParameter(sys::SlangReflectionTypeParameter);

impl TypeParameter {
	pub fn name(&self) -> &str {
		let name = rcall!(spReflectionTypeParameter_GetName(self));
		unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() }
	}

	pub fn index(&self) -> u32 {
		rcall!(spReflectionTypeParameter_GetIndex(self))
	}

	pub fn constraint_count(&self) -> u32 {
		rcall!(spReflectionTypeParameter_GetConstraintCount(self))
	}

	pub fn constraint_by_index(&self, index: u32) -> &Type {
		rcall!(spReflectionTypeParameter_GetConstraintByIndex(self, index) as &Type)
	}

	pub fn constraints(&self) -> impl ExactSizeIterator<Item = &Type> {
		(0..self.constraint_count())
			.map(move |i| rcall!(spReflectionTypeParameter_GetConstraintByIndex(self, i) as &Type))
	}
}
