use super::{Decl, Type, TypeParameter, Variable, rcall};
use crate::{DeclKind, sys};

#[repr(transparent)]
pub struct Generic(sys::SlangReflectionGeneric);

impl Generic {
	pub fn as_decl(&self) -> Option<&Decl> {
		rcall!(spReflectionGeneric_asDecl(self) as Option<&Decl>)
	}

	pub fn name(&self) -> Option<&str> {
		rcall!(spReflectionGeneric_GetName(self) as Option<&str>)
	}

	pub fn type_parameter_count(&self) -> u32 {
		rcall!(spReflectionGeneric_GetTypeParameterCount(self))
	}

	pub fn type_parameter_by_index(&self, index: u32) -> Option<&TypeParameter> {
		rcall!(spReflectionGeneric_GetTypeParameter(self, index) as Option<&TypeParameter>)
	}

	pub fn type_parameters(&self) -> impl ExactSizeIterator<Item = &TypeParameter> {
		(0..self.type_parameter_count()).map(|i| self.type_parameter_by_index(i).unwrap())
	}

	pub fn value_parameter_count(&self) -> u32 {
		rcall!(spReflectionGeneric_GetValueParameterCount(self))
	}

	pub fn value_parameter_by_index(&self, index: u32) -> Option<&Variable> {
		rcall!(spReflectionGeneric_GetValueParameter(self, index) as Option<&Variable>)
	}

	pub fn value_parameters(&self) -> impl ExactSizeIterator<Item = &Variable> {
		(0..self.value_parameter_count()).map(|i| self.value_parameter_by_index(i).unwrap())
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

	pub fn inner_decl(&self) -> Option<&Decl> {
		rcall!(spReflectionGeneric_GetInnerDecl(self) as Option<&Decl>)
	}

	pub fn inner_kind(&self) -> DeclKind {
		rcall!(spReflectionGeneric_GetInnerKind(self))
	}

	pub fn outer_generic_container(&self) -> Option<&Generic> {
		rcall!(spReflectionGeneric_GetOuterGenericContainer(self) as Option<&Generic>)
	}

	pub fn concrete_type(&self, type_param: &Variable) -> Option<&Type> {
		rcall!(
			spReflectionGeneric_GetConcreteType(self, type_param as *const _ as *mut _)
				as Option<&Type>
		)
	}

	pub fn concrete_int_val(&self, value_param: &Variable) -> i64 {
		rcall!(spReflectionGeneric_GetConcreteIntVal(
			self,
			value_param as *const _ as *mut _
		))
	}

	pub fn apply_specializations(&self, generic: &Generic) -> Option<&Generic> {
		rcall!(
			spReflectionGeneric_applySpecializations(self, generic as *const _ as *mut _)
				as Option<&Generic>
		)
	}
}
