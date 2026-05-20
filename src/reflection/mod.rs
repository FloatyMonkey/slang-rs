mod decl;
mod entry_point;
mod function;
mod generic;
mod shader;
mod ty;
mod type_layout;
mod type_parameter;
mod user_attribute;
mod variable;
mod variable_layout;

pub use decl::Decl;
pub use entry_point::EntryPoint;
pub use function::Function;
pub use generic::Generic;
pub use shader::Shader;
pub use ty::Type;
pub use type_layout::TypeLayout;
pub use type_parameter::TypeParameter;
pub use user_attribute::UserAttribute;
pub use variable::Variable;
pub use variable_layout::VariableLayout;

use super::{Modifier, sys};

pub fn compute_string_hash(string: &str) -> u32 {
	rcall!(spComputeStringHash(string, string.len()))
}

macro_rules! rcall {
	($f:ident($s:ident $(,$arg:expr)*)) => {
		unsafe { sys::$f($s as *const _ as *mut _ $(,$arg)*) }
	};

	($f:ident($s:ident $(,$arg:expr)*) as Option<&str>) => {
		unsafe {
			let ptr = sys::$f($s as *const _ as *mut _ $(,$arg)*);
			(!ptr.is_null()).then(|| std::ffi::CStr::from_ptr(ptr).to_str().ok()).flatten()
		}
	};

	($f:ident($s:ident $(,$arg:expr)*) as Option<&$cast:ty>) => {
		unsafe {
			let ptr = sys::$f($s as *const _ as *mut _ $(,$arg)*);
			super::ref_from_ptr::<_, $cast>(ptr)
		}
	};
}

pub(super) use rcall;

/// Trait to associate wrapper types with their underlying system types.
/// This ensures conversions from raw pointers to wrapper types, as performed by the rcall! macro, are type-safe.
pub(super) unsafe trait Wrapper {
	type SysType;
}

unsafe impl Wrapper for Decl {
	type SysType = sys::SlangReflectionDecl;
}
unsafe impl Wrapper for EntryPoint {
	type SysType = sys::SlangReflectionEntryPoint;
}
unsafe impl Wrapper for Function {
	type SysType = sys::SlangReflectionFunction;
}
unsafe impl Wrapper for Generic {
	type SysType = sys::SlangReflectionGeneric;
}
unsafe impl Wrapper for Modifier {
	type SysType = sys::SlangReflectionModifier;
}
unsafe impl Wrapper for Shader {
	type SysType = sys::SlangReflection;
}
unsafe impl Wrapper for Type {
	type SysType = sys::SlangReflectionType;
}
unsafe impl Wrapper for TypeLayout {
	type SysType = sys::SlangReflectionTypeLayout;
}
unsafe impl Wrapper for TypeParameter {
	type SysType = sys::SlangReflectionTypeParameter;
}
unsafe impl Wrapper for UserAttribute {
	type SysType = sys::SlangReflectionUserAttribute;
}
unsafe impl Wrapper for Variable {
	type SysType = sys::SlangReflectionVariable;
}
unsafe impl Wrapper for VariableLayout {
	type SysType = sys::SlangReflectionVariableLayout;
}

pub(super) unsafe fn ref_from_ptr<'a, S, W>(ptr: *mut S) -> Option<&'a W>
where
	W: Wrapper<SysType = S>,
{
	(!ptr.is_null()).then(|| unsafe { &*(ptr as *const W) })
}
