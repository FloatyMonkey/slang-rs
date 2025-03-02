use super::{UserAttribute, Variable, VariableLayout, rcall};
use slang_sys as sys;

#[repr(transparent)]
pub struct Type(sys::SlangReflectionType);

impl Type {
	pub fn kind(&self) -> sys::SlangTypeKind {
		rcall!(spReflectionType_GetKind(self))
	}

	pub fn field_count(&self) -> u32 {
		rcall!(spReflectionType_GetFieldCount(self))
	}

	pub fn field_by_index(&self, index: u32) -> Option<&Variable> {
		rcall!(spReflectionType_GetFieldByIndex(self, index) as Option<&Variable>)
	}

	pub fn fields(&self) -> impl ExactSizeIterator<Item = &Variable> {
		(0..self.field_count())
			.map(move |i| rcall!(spReflectionType_GetFieldByIndex(self, i) as &Variable))
	}

	// TODO: is_array

	// TODO: unwrap_array

	pub fn element_count(&self) -> usize {
		rcall!(spReflectionType_GetElementCount(self))
	}

	// TODO: total_array_element_count

	pub fn element_type(&self) -> &Type {
		rcall!(spReflectionType_GetElementType(self) as &Type)
	}

	pub fn row_count(&self) -> u32 {
		rcall!(spReflectionType_GetRowCount(self))
	}

	pub fn column_count(&self) -> u32 {
		rcall!(spReflectionType_GetColumnCount(self))
	}

	pub fn scalar_type(&self) -> sys::SlangScalarType {
		rcall!(spReflectionType_GetScalarType(self))
	}

	pub fn resource_result_type(&self) -> &Type {
		rcall!(spReflectionType_GetResourceResultType(self) as &Type)
	}

	pub fn resource_shape(&self) -> sys::SlangResourceShape {
		rcall!(spReflectionType_GetResourceShape(self))
	}

	pub fn resource_access(&self) -> sys::SlangResourceAccess {
		rcall!(spReflectionType_GetResourceAccess(self))
	}

	pub fn name(&self) -> Option<&str> {
		let name = rcall!(spReflectionType_GetName(self));
		unsafe { (!name.is_null()).then(|| std::ffi::CStr::from_ptr(name).to_str().unwrap()) }
	}

	// TODO: full_name

	pub fn user_attribute_count(&self) -> u32 {
		rcall!(spReflectionType_GetUserAttributeCount(self))
	}

	pub fn user_attribute_by_index(&self, index: u32) -> Option<&UserAttribute> {
		rcall!(spReflectionType_GetUserAttribute(self, index) as Option<&UserAttribute>)
	}

	pub fn user_attributes(&self) -> impl ExactSizeIterator<Item = &UserAttribute> {
		(0..self.user_attribute_count())
			.map(move |i| rcall!(spReflectionType_GetUserAttribute(self, i) as &UserAttribute))
	}

	pub fn find_user_attribute_by_name(&self, name: &str) -> Option<&UserAttribute> {
		let name = std::ffi::CString::new(name).unwrap();
		rcall!(
			spReflectionType_FindUserAttributeByName(self, name.as_ptr()) as Option<&UserAttribute>
		)
	}
}

#[repr(transparent)]
pub struct TypeLayout(sys::SlangReflectionTypeLayout);

impl TypeLayout {
	pub fn ty(&self) -> &Type {
		rcall!(spReflectionTypeLayout_GetType(self) as &Type)
	}

	pub fn kind(&self) -> sys::SlangTypeKind {
		rcall!(spReflectionTypeLayout_getKind(self))
	}

	pub fn size(&self, category: sys::SlangParameterCategory) -> usize {
		rcall!(spReflectionTypeLayout_GetSize(self, category))
	}

	pub fn stride(&self, category: sys::SlangParameterCategory) -> usize {
		rcall!(spReflectionTypeLayout_GetStride(self, category))
	}

	pub fn alignment(&self, category: sys::SlangParameterCategory) -> i32 {
		rcall!(spReflectionTypeLayout_getAlignment(self, category))
	}

	pub fn field_count(&self) -> u32 {
		rcall!(spReflectionTypeLayout_GetFieldCount(self))
	}

	pub fn field_by_index(&self, index: u32) -> Option<&VariableLayout> {
		rcall!(spReflectionTypeLayout_GetFieldByIndex(self, index) as Option<&VariableLayout>)
	}

	pub fn fields(&self) -> impl ExactSizeIterator<Item = &VariableLayout> {
		(0..self.field_count()).map(move |i| {
			rcall!(spReflectionTypeLayout_GetFieldByIndex(self, i) as &VariableLayout)
		})
	}

	// TODO: find_field_index_by_name
	// TODO: explicit_counter
	// TODO: is_array
	// TODO: unwrap_array

	pub fn element_count(&self) -> usize {
		self.ty().element_count()
	}

	// TODO: total_array_element_count

	pub fn element_stride(&self, category: sys::SlangParameterCategory) -> usize {
		rcall!(spReflectionTypeLayout_GetElementStride(self, category))
	}

	pub fn element_type_layout(&self) -> &TypeLayout {
		rcall!(spReflectionTypeLayout_GetElementTypeLayout(self) as &TypeLayout)
	}

	pub fn element_var_layout(&self) -> &VariableLayout {
		rcall!(spReflectionTypeLayout_GetElementVarLayout(self) as &VariableLayout)
	}

	pub fn container_var_layout(&self) -> &VariableLayout {
		rcall!(spReflectionTypeLayout_getContainerVarLayout(self) as &VariableLayout)
	}

	pub fn parameter_category(&self) -> sys::SlangParameterCategory {
		rcall!(spReflectionTypeLayout_GetParameterCategory(self))
	}

	pub fn category_count(&self) -> u32 {
		rcall!(spReflectionTypeLayout_GetCategoryCount(self))
	}

	pub fn category_by_index(&self, index: u32) -> sys::SlangParameterCategory {
		rcall!(spReflectionTypeLayout_GetCategoryByIndex(self, index))
	}

	pub fn categories(&self) -> impl ExactSizeIterator<Item = sys::SlangParameterCategory> + '_ {
		(0..self.category_count())
			.map(move |i| rcall!(spReflectionTypeLayout_GetCategoryByIndex(self, i)))
	}

	pub fn row_count(&self) -> u32 {
		self.ty().row_count()
	}

	pub fn column_count(&self) -> u32 {
		self.ty().column_count()
	}

	pub fn scalar_type(&self) -> sys::SlangScalarType {
		self.ty().scalar_type()
	}

	pub fn resource_result_type(&self) -> &Type {
		self.ty().resource_result_type()
	}

	pub fn resource_shape(&self) -> sys::SlangResourceShape {
		self.ty().resource_shape()
	}

	pub fn resource_access(&self) -> sys::SlangResourceAccess {
		self.ty().resource_access()
	}

	pub fn name(&self) -> Option<&str> {
		self.ty().name()
	}

	pub fn matrix_layout_mode(&self) -> sys::SlangMatrixLayoutMode {
		rcall!(spReflectionTypeLayout_GetMatrixLayoutMode(self))
	}

	pub fn generic_param_index(&self) -> i32 {
		rcall!(spReflectionTypeLayout_getGenericParamIndex(self))
	}

	pub fn pending_data_type_layout(&self) -> &TypeLayout {
		rcall!(spReflectionTypeLayout_getPendingDataTypeLayout(self) as &TypeLayout)
	}

	pub fn specialized_type_pending_data_var_layout(&self) -> &VariableLayout {
		rcall!(
			spReflectionTypeLayout_getSpecializedTypePendingDataVarLayout(self) as &VariableLayout
		)
	}

	pub fn binding_range_count(&self) -> i64 {
		rcall!(spReflectionTypeLayout_getBindingRangeCount(self))
	}

	pub fn binding_range_type(&self, index: i64) -> sys::SlangBindingType {
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

	pub fn binding_range_leaf_type_layout(&self, index: i64) -> &TypeLayout {
		rcall!(spReflectionTypeLayout_getBindingRangeLeafTypeLayout(self, index) as &TypeLayout)
	}

	pub fn binding_range_leaf_variable(&self, index: i64) -> &Variable {
		rcall!(spReflectionTypeLayout_getBindingRangeLeafVariable(self, index) as &Variable)
	}

	pub fn binding_range_image_format(&self, index: i64) -> sys::SlangImageFormat {
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
	) -> sys::SlangBindingType {
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
	) -> sys::SlangParameterCategory {
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

	pub fn sub_object_range_offset(&self, sub_object_range_index: i64) -> &VariableLayout {
		rcall!(
			spReflectionTypeLayout_getSubObjectRangeOffset(self, sub_object_range_index)
				as &VariableLayout
		)
	}
}
