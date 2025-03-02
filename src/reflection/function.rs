use super::{Type, UserAttribute, Variable, rcall};
use slang_sys as sys;

#[repr(transparent)]
pub struct Function(sys::SlangReflectionFunction);

impl Function {
	pub fn name(&self) -> &str {
		let name = rcall!(spReflectionFunction_GetName(self));
		unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() }
	}

	pub fn return_type(&self) -> &Type {
		rcall!(spReflectionFunction_GetResultType(self) as &Type)
	}

	pub fn parameter_count(&self) -> u32 {
		rcall!(spReflectionFunction_GetParameterCount(self))
	}

	pub fn parameter_by_index(&self, index: u32) -> Option<&Variable> {
		rcall!(spReflectionFunction_GetParameter(self, index) as Option<&Variable>)
	}

	pub fn parameters(&self) -> impl ExactSizeIterator<Item = &Variable> {
		(0..self.parameter_count())
			.map(move |i| rcall!(spReflectionFunction_GetParameter(self, i) as &Variable))
	}

	pub fn user_attribute_count(&self) -> u32 {
		rcall!(spReflectionFunction_GetUserAttributeCount(self))
	}

	pub fn user_attribute_by_index(&self, index: u32) -> Option<&UserAttribute> {
		rcall!(spReflectionFunction_GetUserAttribute(self, index) as Option<&UserAttribute>)
	}

	pub fn user_attributes(&self) -> impl ExactSizeIterator<Item = &UserAttribute> {
		(0..self.user_attribute_count())
			.map(move |i| rcall!(spReflectionFunction_GetUserAttribute(self, i) as &UserAttribute))
	}

	// TODO: find_user_attribute_by_name
	// TODO: find_modifier
	// TODO: generic_container
	// TODO: apply_specializations
	// TODO: specialize_with_arg_types

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
		(0..self.overload_count())
			.map(move |i| rcall!(spReflectionFunction_getOverload(self, i) as &Function))
	}
}
