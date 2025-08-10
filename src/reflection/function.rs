use super::{Generic, Type, UserAttribute, Variable, rcall};
use crate::{GlobalSession, Modifier, ModifierID, sys};

#[repr(transparent)]
pub struct Function(sys::SlangReflectionFunction);

impl Function {
	pub fn name(&self) -> Option<&str> {
		rcall!(spReflectionFunction_GetName(self) as Option<&str>)
	}

	pub fn return_type(&self) -> Option<&Type> {
		rcall!(spReflectionFunction_GetResultType(self) as Option<&Type>)
	}

	pub fn parameter_count(&self) -> u32 {
		rcall!(spReflectionFunction_GetParameterCount(self))
	}

	pub fn parameter_by_index(&self, index: u32) -> Option<&Variable> {
		rcall!(spReflectionFunction_GetParameter(self, index) as Option<&Variable>)
	}

	pub fn parameters(&self) -> impl ExactSizeIterator<Item = &Variable> {
		(0..self.parameter_count()).map(|i| self.parameter_by_index(i).unwrap())
	}

	pub fn user_attribute_count(&self) -> u32 {
		rcall!(spReflectionFunction_GetUserAttributeCount(self))
	}

	pub fn user_attribute_by_index(&self, index: u32) -> Option<&UserAttribute> {
		rcall!(spReflectionFunction_GetUserAttribute(self, index) as Option<&UserAttribute>)
	}

	pub fn user_attributes(&self) -> impl ExactSizeIterator<Item = &UserAttribute> {
		(0..self.user_attribute_count()).map(|i| self.user_attribute_by_index(i).unwrap())
	}

	pub fn find_user_attribute_by_name(
		&self,
		global_session: &GlobalSession,
		name: &str,
	) -> Option<&UserAttribute> {
		let name = std::ffi::CString::new(name).unwrap();
		rcall!(spReflectionFunction_FindUserAttributeByName(
			self,
			global_session as *const _ as *mut _,
			name.as_ptr()
		) as Option<&UserAttribute>)
	}

	pub fn find_modifier(&self, id: ModifierID) -> Option<&Modifier> {
		rcall!(spReflectionFunction_FindModifier(self, id) as Option<&Modifier>)
	}

	pub fn generic_container(&self) -> Option<&Generic> {
		rcall!(spReflectionFunction_GetGenericContainer(self) as Option<&Generic>)
	}

	pub fn apply_specializations(&self, generic: &Generic) -> Option<&Function> {
		rcall!(
			spReflectionFunction_applySpecializations(self, generic as *const _ as *mut _)
				as Option<&Function>
		)
	}

	pub fn specialize_with_arg_types(&self, types: &[&Type]) -> Option<&Function> {
		rcall!(spReflectionFunction_specializeWithArgTypes(
			self,
			types.len() as i64,
			types.as_ptr() as *mut _
		) as Option<&Function>)
	}

	pub fn is_overloaded(&self) -> bool {
		rcall!(spReflectionFunction_isOverloaded(self))
	}

	pub fn overload_count(&self) -> u32 {
		rcall!(spReflectionFunction_getOverloadCount(self))
	}

	pub fn overload_by_index(&self, index: u32) -> Option<&Function> {
		rcall!(spReflectionFunction_getOverload(self, index) as Option<&Function>)
	}

	pub fn overloads(&self) -> impl ExactSizeIterator<Item = &Function> {
		(0..self.overload_count()).map(|i| self.overload_by_index(i).unwrap())
	}
}
