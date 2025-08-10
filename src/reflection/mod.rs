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

use super::sys;

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
			(!ptr.is_null()).then(|| &*(ptr as *const $cast))
		}
	};
}

pub(super) use rcall;
