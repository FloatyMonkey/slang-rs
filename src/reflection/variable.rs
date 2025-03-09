use super::{Type, TypeLayout, UserAttribute, rcall};
use slang_sys as sys;

#[repr(transparent)]
pub struct Variable(sys::SlangReflectionVariable);

impl Variable {
	pub fn name(&self) -> &str {
		let name = rcall!(spReflectionVariable_GetName(self));
		unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() }
	}

	pub fn ty(&self) -> &Type {
		rcall!(spReflectionVariable_GetType(self) as &Type)
	}

	// TODO: find_modifier

	pub fn user_attribute_count(&self) -> u32 {
		rcall!(spReflectionVariable_GetUserAttributeCount(self))
	}

	pub fn user_attribute_by_index(&self, index: u32) -> Option<&UserAttribute> {
		rcall!(spReflectionVariable_GetUserAttribute(self, index) as Option<&UserAttribute>)
	}

	pub fn user_attributes(&self) -> impl ExactSizeIterator<Item = &UserAttribute> {
		(0..self.user_attribute_count())
			.map(move |i| rcall!(spReflectionVariable_GetUserAttribute(self, i) as &UserAttribute))
	}

	// TODO: find_user_attribute_by_name

	pub fn has_default_value(&self) -> bool {
		rcall!(spReflectionVariable_HasDefaultValue(self))
	}

	// TODO: generic_container
	// TODO: apply_specializations
}

#[repr(transparent)]
pub struct VariableLayout(sys::SlangReflectionVariableLayout);

impl VariableLayout {
	pub fn variable(&self) -> Option<&Variable> {
		rcall!(spReflectionVariableLayout_GetVariable(self) as Option<&Variable>)
	}

	// TODO: get_name
	// TODO: find_modifier

	pub fn type_layout(&self) -> &TypeLayout {
		rcall!(spReflectionVariableLayout_GetTypeLayout(self) as &TypeLayout)
	}

	pub fn category(&self) -> sys::SlangParameterCategory {
		self.type_layout().parameter_category()
	}

	pub fn category_count(&self) -> u32 {
		self.type_layout().category_count()
	}

	pub fn category_by_index(&self, index: u32) -> sys::SlangParameterCategory {
		self.type_layout().category_by_index(index)
	}

	pub fn offset(&self, category: sys::SlangParameterCategory) -> usize {
		rcall!(spReflectionVariableLayout_GetOffset(self, category))
	}

	pub fn ty(&self) -> Option<&Type> {
		Some(self.variable()?.ty())
	}

	pub fn binding_index(&self) -> u32 {
		rcall!(spReflectionParameter_GetBindingIndex(self))
	}

	pub fn binding_space(&self) -> u32 {
		rcall!(spReflectionParameter_GetBindingSpace(self))
	}

	pub fn binding_space_with_category(&self, category: sys::SlangParameterCategory) -> usize {
		rcall!(spReflectionVariableLayout_GetSpace(self, category))
	}

	pub fn semantic_name(&self) -> Option<&str> {
		let name = rcall!(spReflectionVariableLayout_GetSemanticName(self));
		unsafe { (!name.is_null()).then(|| std::ffi::CStr::from_ptr(name).to_str().unwrap()) }
	}

	pub fn semantic_index(&self) -> usize {
		rcall!(spReflectionVariableLayout_GetSemanticIndex(self))
	}

	pub fn stage(&self) -> sys::SlangStage {
		rcall!(spReflectionVariableLayout_getStage(self))
	}

	pub fn pending_data_layout(&self) -> &VariableLayout {
		rcall!(spReflectionVariableLayout_getPendingDataLayout(self) as &VariableLayout)
	}
}
