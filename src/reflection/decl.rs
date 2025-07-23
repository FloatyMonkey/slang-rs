use super::{Function, Generic, Type, Variable, rcall};
use crate::{DeclKind, sys};

#[repr(transparent)]
pub struct Decl(sys::SlangReflectionDecl);

impl Decl {
	pub fn name(&self) -> &str {
		let name = rcall!(spReflectionDecl_getName(self));
		unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() }
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
		(0..self.child_count()).map(move |i| rcall!(spReflectionDecl_getChild(self, i) as &Decl))
	}

	pub fn ty(&self) -> &Type {
		rcall!(spReflection_getTypeFromDecl(self) as &Type)
	}

	pub fn as_variable(&self) -> &Variable {
		rcall!(spReflectionDecl_castToVariable(self) as &Variable)
	}

	pub fn as_function(&self) -> &Function {
		rcall!(spReflectionDecl_castToFunction(self) as &Function)
	}

	pub fn as_generic(&self) -> &Generic {
		rcall!(spReflectionDecl_castToGeneric(self) as &Generic)
	}

	pub fn parent(&self) -> &Decl {
		rcall!(spReflectionDecl_getParent(self) as &Decl)
	}
}
