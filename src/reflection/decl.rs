use super::{Function, Generic, Type, Variable, rcall};
use crate::{DeclKind, sys};

#[repr(transparent)]
pub struct Decl(sys::SlangReflectionDecl);

impl Decl {
	pub fn name(&self) -> Option<&str> {
		let name = rcall!(spReflectionDecl_getName(self));
		(!name.is_null()).then(|| unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() })
	}

	pub fn kind(&self) -> DeclKind {
		rcall!(spReflectionDecl_getKind(self))
	}

	pub fn child_count(&self) -> u32 {
		rcall!(spReflectionDecl_getChildrenCount(self))
	}

	pub fn child_by_index(&self, index: u32) -> Option<&Decl> {
		rcall!(spReflectionDecl_getChild(self, index) as Option<&Decl>)
	}

	pub fn children(&self) -> impl ExactSizeIterator<Item = &Decl> {
		(0..self.child_count()).map(|i| self.child_by_index(i).unwrap())
	}

	pub fn ty(&self) -> Option<&Type> {
		rcall!(spReflection_getTypeFromDecl(self) as Option<&Type>)
	}

	pub fn as_variable(&self) -> Option<&Variable> {
		rcall!(spReflectionDecl_castToVariable(self) as Option<&Variable>)
	}

	pub fn as_function(&self) -> Option<&Function> {
		rcall!(spReflectionDecl_castToFunction(self) as Option<&Function>)
	}

	pub fn as_generic(&self) -> Option<&Generic> {
		rcall!(spReflectionDecl_castToGeneric(self) as Option<&Generic>)
	}

	pub fn parent(&self) -> Option<&Decl> {
		rcall!(spReflectionDecl_getParent(self) as Option<&Decl>)
	}
}
