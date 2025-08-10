use super::{Type, Variable, VariableLayout, rcall};
use crate::{
	BindingType, ImageFormat, MatrixLayoutMode, ParameterCategory, ResourceAccess, ResourceShape,
	ScalarType, TypeKind, sys,
};

#[repr(transparent)]
pub struct TypeLayout(sys::SlangReflectionTypeLayout);

impl TypeLayout {
	pub fn ty(&self) -> Option<&Type> {
		rcall!(spReflectionTypeLayout_GetType(self) as Option<&Type>)
	}

	pub fn kind(&self) -> TypeKind {
		rcall!(spReflectionTypeLayout_getKind(self))
	}

	pub fn size(&self, category: ParameterCategory) -> usize {
		rcall!(spReflectionTypeLayout_GetSize(self, category))
	}

	pub fn stride(&self, category: ParameterCategory) -> usize {
		rcall!(spReflectionTypeLayout_GetStride(self, category))
	}

	pub fn alignment(&self, category: ParameterCategory) -> i32 {
		rcall!(spReflectionTypeLayout_getAlignment(self, category))
	}

	pub fn field_count(&self) -> u32 {
		rcall!(spReflectionTypeLayout_GetFieldCount(self))
	}

	pub fn field_by_index(&self, index: u32) -> Option<&VariableLayout> {
		rcall!(spReflectionTypeLayout_GetFieldByIndex(self, index) as Option<&VariableLayout>)
	}

	pub fn fields(&self) -> impl ExactSizeIterator<Item = &VariableLayout> {
		(0..self.field_count()).map(|i| self.field_by_index(i).unwrap())
	}

	pub fn find_field_index_by_name(&self, name: &str) -> i64 {
		let (start, end) = (name.as_ptr(), unsafe { name.as_ptr().add(name.len()) });
		rcall!(spReflectionTypeLayout_findFieldIndexByName(
			self,
			start as *const _,
			end as *const _
		))
	}

	pub fn explicit_counter(&self) -> Option<&VariableLayout> {
		rcall!(spReflectionTypeLayout_GetExplicitCounter(self) as Option<&VariableLayout>)
	}

	pub fn is_array(&self) -> bool {
		self.ty().map(|t| t.is_array()).unwrap_or(false)
	}

	pub fn unwrap_array(&self) -> &TypeLayout {
		let mut ty = self;
		while ty.is_array() {
			ty = match ty.element_type_layout() {
				Some(t) => t,
				None => break,
			};
		}
		ty
	}

	pub fn total_array_element_count(&self) -> usize {
		self.ty()
			.map(|t| t.total_array_element_count())
			.unwrap_or(0)
	}

	pub fn element_count(&self) -> Option<usize> {
		Some(self.ty()?.element_count())
	}

	pub fn element_stride(&self, category: ParameterCategory) -> usize {
		rcall!(spReflectionTypeLayout_GetElementStride(self, category))
	}

	pub fn element_type_layout(&self) -> Option<&TypeLayout> {
		rcall!(spReflectionTypeLayout_GetElementTypeLayout(self) as Option<&TypeLayout>)
	}

	pub fn element_var_layout(&self) -> Option<&VariableLayout> {
		rcall!(spReflectionTypeLayout_GetElementVarLayout(self) as Option<&VariableLayout>)
	}

	pub fn container_var_layout(&self) -> Option<&VariableLayout> {
		rcall!(spReflectionTypeLayout_getContainerVarLayout(self) as Option<&VariableLayout>)
	}

	pub fn parameter_category(&self) -> ParameterCategory {
		rcall!(spReflectionTypeLayout_GetParameterCategory(self))
	}

	pub fn category_count(&self) -> u32 {
		rcall!(spReflectionTypeLayout_GetCategoryCount(self))
	}

	pub fn category_by_index(&self, index: u32) -> ParameterCategory {
		rcall!(spReflectionTypeLayout_GetCategoryByIndex(self, index))
	}

	pub fn categories(&self) -> impl ExactSizeIterator<Item = ParameterCategory> {
		(0..self.category_count()).map(|i| self.category_by_index(i))
	}

	pub fn row_count(&self) -> Option<u32> {
		Some(self.ty()?.row_count())
	}

	pub fn column_count(&self) -> Option<u32> {
		Some(self.ty()?.column_count())
	}

	pub fn scalar_type(&self) -> Option<ScalarType> {
		Some(self.ty()?.scalar_type())
	}

	pub fn resource_result_type(&self) -> Option<&Type> {
		self.ty()?.resource_result_type()
	}

	pub fn resource_shape(&self) -> Option<ResourceShape> {
		Some(self.ty()?.resource_shape())
	}

	pub fn resource_access(&self) -> Option<ResourceAccess> {
		Some(self.ty()?.resource_access())
	}

	pub fn name(&self) -> Option<&str> {
		self.ty()?.name()
	}

	pub fn matrix_layout_mode(&self) -> MatrixLayoutMode {
		rcall!(spReflectionTypeLayout_GetMatrixLayoutMode(self))
	}

	pub fn generic_param_index(&self) -> i32 {
		rcall!(spReflectionTypeLayout_getGenericParamIndex(self))
	}

	pub fn pending_data_type_layout(&self) -> Option<&TypeLayout> {
		rcall!(spReflectionTypeLayout_getPendingDataTypeLayout(self) as Option<&TypeLayout>)
	}

	pub fn specialized_type_pending_data_var_layout(&self) -> Option<&VariableLayout> {
		rcall!(
			spReflectionTypeLayout_getSpecializedTypePendingDataVarLayout(self)
				as Option<&VariableLayout>
		)
	}

	pub fn binding_range_count(&self) -> i64 {
		rcall!(spReflectionTypeLayout_getBindingRangeCount(self))
	}

	pub fn binding_range_type(&self, index: i64) -> BindingType {
		rcall!(spReflectionTypeLayout_getBindingRangeType(self, index))
	}

	pub fn is_binding_range_specializable(&self, index: i64) -> bool {
		rcall!(spReflectionTypeLayout_isBindingRangeSpecializable(
			self, index
		)) != 0
	}

	pub fn binding_range_binding_count(&self, index: i64) -> i64 {
		rcall!(spReflectionTypeLayout_getBindingRangeBindingCount(
			self, index
		))
	}

	pub fn field_binding_range_offset(&self, field_index: i64) -> i64 {
		rcall!(spReflectionTypeLayout_getFieldBindingRangeOffset(
			self,
			field_index
		))
	}

	pub fn explicit_counter_binding_range_offset(&self) -> i64 {
		rcall!(spReflectionTypeLayout_getExplicitCounterBindingRangeOffset(
			self
		))
	}

	pub fn binding_range_leaf_type_layout(&self, index: i64) -> Option<&TypeLayout> {
		rcall!(
			spReflectionTypeLayout_getBindingRangeLeafTypeLayout(self, index)
				as Option<&TypeLayout>
		)
	}

	pub fn binding_range_leaf_variable(&self, index: i64) -> Option<&Variable> {
		rcall!(spReflectionTypeLayout_getBindingRangeLeafVariable(self, index) as Option<&Variable>)
	}

	pub fn binding_range_image_format(&self, index: i64) -> ImageFormat {
		rcall!(spReflectionTypeLayout_getBindingRangeImageFormat(
			self, index
		))
	}

	pub fn binding_range_descriptor_set_index(&self, index: i64) -> i64 {
		rcall!(spReflectionTypeLayout_getBindingRangeDescriptorSetIndex(
			self, index
		))
	}

	pub fn binding_range_first_descriptor_range_index(&self, index: i64) -> i64 {
		rcall!(spReflectionTypeLayout_getBindingRangeFirstDescriptorRangeIndex(self, index))
	}

	pub fn binding_range_descriptor_range_count(&self, index: i64) -> i64 {
		rcall!(spReflectionTypeLayout_getBindingRangeDescriptorRangeCount(
			self, index
		))
	}

	pub fn descriptor_set_count(&self) -> i64 {
		rcall!(spReflectionTypeLayout_getDescriptorSetCount(self))
	}

	pub fn descriptor_set_space_offset(&self, set_index: i64) -> i64 {
		rcall!(spReflectionTypeLayout_getDescriptorSetSpaceOffset(
			self, set_index
		))
	}

	pub fn descriptor_set_descriptor_range_count(&self, set_index: i64) -> i64 {
		rcall!(spReflectionTypeLayout_getDescriptorSetDescriptorRangeCount(
			self, set_index
		))
	}

	pub fn descriptor_set_descriptor_range_index_offset(
		&self,
		set_index: i64,
		range_index: i64,
	) -> i64 {
		rcall!(
			spReflectionTypeLayout_getDescriptorSetDescriptorRangeIndexOffset(
				self,
				set_index,
				range_index
			)
		)
	}

	pub fn descriptor_set_descriptor_range_descriptor_count(
		&self,
		set_index: i64,
		range_index: i64,
	) -> i64 {
		rcall!(
			spReflectionTypeLayout_getDescriptorSetDescriptorRangeDescriptorCount(
				self,
				set_index,
				range_index
			)
		)
	}

	pub fn descriptor_set_descriptor_range_type(
		&self,
		set_index: i64,
		range_index: i64,
	) -> BindingType {
		rcall!(spReflectionTypeLayout_getDescriptorSetDescriptorRangeType(
			self,
			set_index,
			range_index
		))
	}

	pub fn descriptor_set_descriptor_range_category(
		&self,
		set_index: i64,
		range_index: i64,
	) -> ParameterCategory {
		rcall!(
			spReflectionTypeLayout_getDescriptorSetDescriptorRangeCategory(
				self,
				set_index,
				range_index
			)
		)
	}

	pub fn sub_object_range_count(&self) -> i64 {
		rcall!(spReflectionTypeLayout_getSubObjectRangeCount(self))
	}

	pub fn sub_object_range_binding_range_index(&self, sub_object_range_index: i64) -> i64 {
		rcall!(spReflectionTypeLayout_getSubObjectRangeBindingRangeIndex(
			self,
			sub_object_range_index
		))
	}

	pub fn sub_object_range_space_offset(&self, sub_object_range_index: i64) -> i64 {
		rcall!(spReflectionTypeLayout_getSubObjectRangeSpaceOffset(
			self,
			sub_object_range_index
		))
	}

	pub fn sub_object_range_offset(&self, sub_object_range_index: i64) -> Option<&VariableLayout> {
		rcall!(
			spReflectionTypeLayout_getSubObjectRangeOffset(self, sub_object_range_index)
				as Option<&VariableLayout>
		)
	}
}
