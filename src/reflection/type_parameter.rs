use super::{Type, rcall};
use crate::sys;

#[repr(transparent)]
pub struct TypeParameter(sys::SlangReflectionTypeParameter);

impl TypeParameter {
	pub fn name(&self) -> Option<&str> {
		rcall!(spReflectionTypeParameter_GetName(self) as Option<&str>)
	}

	pub fn index(&self) -> u32 {
		rcall!(spReflectionTypeParameter_GetIndex(self))
	}

	pub fn constraint_count(&self) -> u32 {
		rcall!(spReflectionTypeParameter_GetConstraintCount(self))
	}

	pub fn constraint_by_index(&self, index: u32) -> Option<&Type> {
		rcall!(spReflectionTypeParameter_GetConstraintByIndex(self, index) as Option<&Type>)
	}

	pub fn constraints(&self) -> impl ExactSizeIterator<Item = &Type> {
		(0..self.constraint_count()).map(|i| self.constraint_by_index(i).unwrap())
	}
}
