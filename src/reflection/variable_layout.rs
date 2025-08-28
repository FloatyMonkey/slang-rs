use super::{Type, TypeLayout, Variable, rcall};
use crate::{ImageFormat, Modifier, ModifierID, ParameterCategory, Stage, sys};

#[repr(transparent)]
pub struct VariableLayout(sys::SlangReflectionVariableLayout);

impl VariableLayout {
	pub fn variable(&self) -> Option<&Variable> {
		rcall!(spReflectionVariableLayout_GetVariable(self) as Option<&Variable>)
	}

	pub fn name(&self) -> Option<&str> {
		self.variable()?.name()
	}

	pub fn find_modifier(&self, id: ModifierID) -> Option<&Modifier> {
		self.variable().and_then(|v| v.find_modifier(id))
	}

	pub fn type_layout(&self) -> Option<&TypeLayout> {
		rcall!(spReflectionVariableLayout_GetTypeLayout(self) as Option<&TypeLayout>)
	}

	pub fn category(&self) -> Option<ParameterCategory> {
		Some(self.type_layout()?.parameter_category())
	}

	pub fn category_count(&self) -> u32 {
		self.type_layout().map_or(0, |tl| tl.category_count())
	}

	pub fn category_by_index(&self, index: u32) -> Option<ParameterCategory> {
		Some(self.type_layout()?.category_by_index(index))
	}

	pub fn categories(&self) -> impl ExactSizeIterator<Item = ParameterCategory> {
		(0..self.category_count()).map(|i| self.category_by_index(i).unwrap())
	}

	pub fn offset(&self, category: ParameterCategory) -> usize {
		rcall!(spReflectionVariableLayout_GetOffset(self, category))
	}

	pub fn ty(&self) -> Option<&Type> {
		self.variable()?.ty()
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

	pub fn image_format(&self) -> ImageFormat {
		rcall!(spReflectionVariableLayout_GetImageFormat(self))
	}

	pub fn semantic_name(&self) -> Option<&str> {
		let name = rcall!(spReflectionVariableLayout_GetSemanticName(self));
		(!name.is_null()).then(|| unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() })
	}

	pub fn semantic_index(&self) -> usize {
		rcall!(spReflectionVariableLayout_GetSemanticIndex(self))
	}

	pub fn stage(&self) -> Stage {
		rcall!(spReflectionVariableLayout_getStage(self))
	}

	pub fn pending_data_layout(&self) -> Option<&VariableLayout> {
		rcall!(spReflectionVariableLayout_getPendingDataLayout(self) as Option<&VariableLayout>)
	}
}
