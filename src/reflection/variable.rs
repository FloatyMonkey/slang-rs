use super::{Generic, Type, UserAttribute, rcall};
use crate::{GlobalSession, Modifier, ModifierID, succeeded, sys};

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

	pub fn find_modifier(&self, id: ModifierID) -> Option<&Modifier> {
		rcall!(spReflectionVariable_FindModifier(self, id) as Option<&Modifier>)
	}

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

	pub fn find_user_attribute_by_name(
		&self,
		global_session: &GlobalSession,
		name: &str,
	) -> Option<&UserAttribute> {
		let name = std::ffi::CString::new(name).unwrap();
		rcall!(spReflectionVariable_FindUserAttributeByName(
			self,
			global_session as *const _ as *mut _,
			name.as_ptr()
		) as Option<&UserAttribute>)
	}

	pub fn has_default_value(&self) -> bool {
		rcall!(spReflectionVariable_HasDefaultValue(self))
	}

	pub fn default_value_int(&self) -> Option<i64> {
		let mut value = 0;
		let result = rcall!(spReflectionVariable_GetDefaultValueInt(self, &mut value));
		if succeeded(result) { Some(value) } else { None }
	}

	pub fn generic_container(&self) -> Option<&Generic> {
		rcall!(spReflectionVariable_GetGenericContainer(self) as Option<&Generic>)
	}

	pub fn apply_specializations(&self, generic: &Generic) -> Option<&Variable> {
		rcall!(
			spReflectionVariable_applySpecializations(self, generic as *const _ as *mut _)
				as Option<&Variable>
		)
	}
}
