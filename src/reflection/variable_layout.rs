use super::{Type, TypeLayout, Variable, rcall};
use crate::{Modifier, ModifierID, ParameterCategory, Stage};
use slang_sys as sys;

#[repr(transparent)]
pub struct VariableLayout(sys::SlangReflectionVariableLayout);

impl VariableLayout {
	pub fn variable(&self) -> Option<&Variable> {
		rcall!(spReflectionVariableLayout_GetVariable(self) as Option<&Variable>)
	}

	pub fn name(&self) -> Option<&str> {
		self.variable().map(|v| v.name())
	}

	pub fn find_modifier(&self, id: ModifierID) -> Option<&Modifier> {
		self.variable().and_then(|v| v.find_modifier(id))
	}

	pub fn type_layout(&self) -> &TypeLayout {
		rcall!(spReflectionVariableLayout_GetTypeLayout(self) as &TypeLayout)
	}

	pub fn category(&self) -> ParameterCategory {
		self.type_layout().parameter_category()
	}

	pub fn category_count(&self) -> u32 {
		self.type_layout().category_count()
	}

	pub fn category_by_index(&self, index: u32) -> ParameterCategory {
		self.type_layout().category_by_index(index)
	}

	pub fn offset(&self, category: ParameterCategory) -> usize {
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

	pub fn binding_space_with_category(&self, category: ParameterCategory) -> usize {
		rcall!(spReflectionVariableLayout_GetSpace(self, category))
	}

	pub fn semantic_name(&self) -> Option<&str> {
		let name = rcall!(spReflectionVariableLayout_GetSemanticName(self));
		unsafe { (!name.is_null()).then(|| std::ffi::CStr::from_ptr(name).to_str().unwrap()) }
	}

	pub fn semantic_index(&self) -> usize {
		rcall!(spReflectionVariableLayout_GetSemanticIndex(self))
	}

	pub fn stage(&self) -> Stage {
		rcall!(spReflectionVariableLayout_getStage(self))
	}

	pub fn pending_data_layout(&self) -> &VariableLayout {
		rcall!(spReflectionVariableLayout_getPendingDataLayout(self) as &VariableLayout)
	}
}
