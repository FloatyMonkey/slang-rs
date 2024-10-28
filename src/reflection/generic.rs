use super::{rcall, Decl, Type, TypeParameter, Variable};
use slang_sys as sys;

#[repr(transparent)]
pub struct Generic(sys::SlangReflectionGeneric);

impl Generic {
	pub fn as_decl(&self) -> &Decl {
		rcall!(spReflectionGeneric_asDecl(self) as &Decl)
	}

	pub fn name(&self) -> &str {
		let name = rcall!(spReflectionGeneric_GetName(self));
		unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() }
	}

	pub fn type_parameter_count(&self) -> u32 {
		rcall!(spReflectionGeneric_GetTypeParameterCount(self))
	}

	pub fn type_parameter_by_index(&self, index: u32) -> Option<&TypeParameter> {
		rcall!(spReflectionGeneric_GetTypeParameter(self, index) as Option<&TypeParameter>)
	}

	pub fn type_parameters(&self) -> impl ExactSizeIterator<Item = &TypeParameter> {
		(0..self.type_parameter_count())
			.map(move |i| rcall!(spReflectionGeneric_GetTypeParameter(self, i) as &TypeParameter))
	}

	pub fn value_parameter_count(&self) -> u32 {
		rcall!(spReflectionGeneric_GetValueParameterCount(self))
	}

	pub fn value_parameter_by_index(&self, index: u32) -> Option<&Variable> {
		rcall!(spReflectionGeneric_GetValueParameter(self, index) as Option<&Variable>)
	}

	pub fn value_parameters(&self) -> impl ExactSizeIterator<Item = &Variable> {
		(0..self.value_parameter_count())
			.map(move |i| rcall!(spReflectionGeneric_GetValueParameter(self, i) as &Variable))
	}

	pub fn type_parameter_constraint_count(&self, type_param: &Variable) -> u32 {
		rcall!(spReflectionGeneric_GetTypeParameterConstraintCount(
			self,
			type_param as *const _ as *mut _
		))
	}

	pub fn type_parameter_constraint_by_index(
		&self,
		type_param: &Variable,
		index: u32,
	) -> Option<&Type> {
		rcall!(spReflectionGeneric_GetTypeParameterConstraintType(
			self,
			type_param as *const _ as *mut _,
			index
		) as Option<&Type>)
	}

	pub fn inner_decl(&self) -> &Decl {
		rcall!(spReflectionGeneric_GetInnerDecl(self) as &Decl)
	}

	pub fn inner_kind(&self) -> sys::SlangDeclKind {
		rcall!(spReflectionGeneric_GetInnerKind(self))
	}

	pub fn outer_generic_container(&self) -> &Generic {
		rcall!(spReflectionGeneric_GetOuterGenericContainer(self) as &Generic)
	}

	pub fn concrete_type(&self, type_param: &Variable) -> &Type {
		rcall!(spReflectionGeneric_GetConcreteType(self, type_param as *const _ as *mut _) as &Type)
	}

	pub fn concrete_int_val(&self, value_param: &Variable) -> i64 {
		rcall!(spReflectionGeneric_GetConcreteIntVal(
			self,
			value_param as *const _ as *mut _
		))
	}

	pub fn apply_specializations(&self, generic: &Generic) -> &Generic {
		rcall!(
			spReflectionGeneric_applySpecializations(self, generic as *const _ as *mut _)
				as &Generic
		)
	}
}
