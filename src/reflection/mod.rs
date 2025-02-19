mod decl;
mod entry_point;
mod function;
mod generic;
mod shader;
mod ty;
mod type_parameter;
mod user_attribute;
mod variable;

pub use decl::Decl;
pub use entry_point::EntryPoint;
pub use function::Function;
pub use generic::Generic;
pub use shader::{compute_string_hash, Shader};
pub use ty::{Type, TypeLayout};
pub use type_parameter::TypeParameter;
pub use user_attribute::UserAttribute;
pub use variable::{Variable, VariableLayout};

macro_rules! rcall {
	($f:ident($s:ident $(,$arg:expr)*)) => {
		unsafe { sys::$f($s as *const _ as *mut _ $(,$arg)*) }
	};

	($f:ident($s:ident $(,$arg:expr)*) as Option<&$cast:ty>) => {
		unsafe {
			let ptr = sys::$f($s as *const _ as *mut _ $(,$arg)*);
			(!ptr.is_null()).then(|| &*(ptr as *const $cast))
		}
	};

	($f:ident($s:ident $(,$arg:expr)*) as &$cast:ty) => {
		unsafe { &*(sys::$f($s as *const _ as *mut _ $(,$arg)*) as *const $cast) }
	};
}

pub(super) use rcall;
