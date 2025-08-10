use super::{Generic, UserAttribute, Variable, rcall};
use crate::{
	Blob, Error, IUnknown, ResourceAccess, ResourceShape, Result, ScalarType, TypeKind, succeeded,
	sys,
};

#[repr(transparent)]
pub struct Type(sys::SlangReflectionType);

impl Type {
	pub fn kind(&self) -> TypeKind {
		rcall!(spReflectionType_GetKind(self))
	}

	pub fn field_count(&self) -> u32 {
		rcall!(spReflectionType_GetFieldCount(self))
	}

	pub fn field_by_index(&self, index: u32) -> Option<&Variable> {
		rcall!(spReflectionType_GetFieldByIndex(self, index) as Option<&Variable>)
	}

	pub fn fields(&self) -> impl ExactSizeIterator<Item = &Variable> {
		(0..self.field_count()).map(|i| self.field_by_index(i).unwrap())
	}

	pub fn is_array(&self) -> bool {
		self.kind() == TypeKind::Array
	}

	pub fn unwrap_array(&self) -> &Type {
		let mut ty = self;
		while ty.is_array() {
			ty = match ty.element_type() {
				Some(t) => t,
				None => break,
			};
		}
		ty
	}

	pub fn total_array_element_count(&self) -> usize {
		if !self.is_array() {
			return 0;
		}
		let mut result = 1;
		let mut ty = Some(self);
		while let Some(t) = ty {
			if !t.is_array() {
				break;
			}
			result *= t.element_count();
			ty = t.element_type();
		}
		result
	}

	pub fn element_count(&self) -> usize {
		rcall!(spReflectionType_GetElementCount(self))
	}

	pub fn element_type(&self) -> Option<&Type> {
		rcall!(spReflectionType_GetElementType(self) as Option<&Type>)
	}

	pub fn row_count(&self) -> u32 {
		rcall!(spReflectionType_GetRowCount(self))
	}

	pub fn column_count(&self) -> u32 {
		rcall!(spReflectionType_GetColumnCount(self))
	}

	pub fn scalar_type(&self) -> ScalarType {
		rcall!(spReflectionType_GetScalarType(self))
	}

	pub fn resource_result_type(&self) -> Option<&Type> {
		rcall!(spReflectionType_GetResourceResultType(self) as Option<&Type>)
	}

	pub fn resource_shape(&self) -> ResourceShape {
		rcall!(spReflectionType_GetResourceShape(self))
	}

	pub fn resource_access(&self) -> ResourceAccess {
		rcall!(spReflectionType_GetResourceAccess(self))
	}

	pub fn name(&self) -> Option<&str> {
		rcall!(spReflectionType_GetName(self) as Option<&str>)
	}

	pub fn full_name(&self) -> Result<Blob> {
		let mut name = std::ptr::null_mut();
		let result = rcall!(spReflectionType_GetFullName(self, &mut name));

		if succeeded(result) && !name.is_null() {
			Ok(Blob(IUnknown(
				std::ptr::NonNull::new(name as *mut _).unwrap(),
			)))
		} else {
			Err(Error::Code(result))
		}
	}

	pub fn user_attribute_count(&self) -> u32 {
		rcall!(spReflectionType_GetUserAttributeCount(self))
	}

	pub fn user_attribute_by_index(&self, index: u32) -> Option<&UserAttribute> {
		rcall!(spReflectionType_GetUserAttribute(self, index) as Option<&UserAttribute>)
	}

	pub fn user_attributes(&self) -> impl ExactSizeIterator<Item = &UserAttribute> {
		(0..self.user_attribute_count()).map(|i| self.user_attribute_by_index(i).unwrap())
	}

	pub fn find_user_attribute_by_name(&self, name: &str) -> Option<&UserAttribute> {
		let name = std::ffi::CString::new(name).unwrap();
		rcall!(
			spReflectionType_FindUserAttributeByName(self, name.as_ptr()) as Option<&UserAttribute>
		)
	}

	pub fn generic_container(&self) -> Option<&Generic> {
		rcall!(spReflectionType_GetGenericContainer(self) as Option<&Generic>)
	}

	pub fn apply_specializations(&self, generic: &Generic) -> Option<&Type> {
		rcall!(
			spReflectionType_applySpecializations(self, generic as *const _ as *mut _)
				as Option<&Type>
		)
	}
}
