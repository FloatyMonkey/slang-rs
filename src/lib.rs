use slang_sys as sys;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::slice;

pub use sys::SlangUUID as UUID;
pub use sys::SlangCompileTarget as CompileTarget;
pub use sys::SlangMatrixLayoutMode as MatrixLayoutMode;
pub use sys::SlangOptimizationLevel as OptimizationLevel;
pub use sys::SlangSourceLanguage as SourceLanguage;
pub use sys::SlangStage as Stage;
pub use sys::SlangProfileID as ProfileID;

macro_rules! vcall {
	($self:expr, $method:ident($($args:expr),*)) => {
		unsafe { ($self.vtable().$method)($self.as_raw(), $($args),*) }
	};
}

const fn uuid(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> UUID {
	UUID { data1, data2, data3, data4 }
}

unsafe trait Interface: Sized {
	type Vtable;
	const IID: UUID;

	#[inline(always)]
	unsafe fn vtable(&self) -> &Self::Vtable {
		&**(self.as_raw() as *mut *mut Self::Vtable)
	}

	#[inline(always)]
	unsafe fn as_raw<T>(&self) -> *mut T {
		std::mem::transmute_copy(self)
	}
}

#[repr(transparent)]
pub struct IUnknown(std::ptr::NonNull<std::ffi::c_void>);

unsafe impl Interface for IUnknown {
	type Vtable = sys::ISlangUnknown__bindgen_vtable;
	const IID: UUID = uuid(0x00000000, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);
}

impl Clone for IUnknown {
	fn clone(&self) -> Self {
		vcall!(self, ISlangUnknown_addRef());
		Self(self.0)
	}
}

impl Drop for IUnknown {
	fn drop(&mut self) {
		vcall!(self, ISlangUnknown_release());
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct GlobalSession(IUnknown);

unsafe impl Interface for GlobalSession {
	type Vtable = sys::IGlobalSessionVtable;
	const IID: UUID = uuid(0xc140b5fd, 0xc78, 0x452e, [0xba, 0x7c, 0x1a, 0x1e, 0x70, 0xc7, 0xf7, 0x1c]);
}

impl GlobalSession {
	pub fn new() -> GlobalSession {
		unsafe {
			let mut global_session = std::ptr::null_mut();
			sys::slang_createGlobalSession(sys::SLANG_API_VERSION as _, &mut global_session);
			GlobalSession(IUnknown(std::ptr::NonNull::new(global_session as *mut _).unwrap()))
		}
	}

	pub fn new_without_std_lib() -> GlobalSession {
		unsafe {
			let mut global_session = std::ptr::null_mut();
			sys::slang_createGlobalSessionWithoutStdLib(sys::SLANG_API_VERSION as _, &mut global_session);
			GlobalSession(IUnknown(std::ptr::NonNull::new(global_session as *mut _).unwrap()))
		}
	}

	pub fn create_compile_request(&self) -> CompileRequest {
		let mut compile_request = std::ptr::null_mut();
		vcall!(self, createCompileRequest(&mut compile_request));
		CompileRequest(IUnknown(std::ptr::NonNull::new(compile_request).unwrap()))
	}

	pub fn find_profile(&self, name: &str) -> ProfileID {
		let name = CString::new(name).unwrap();
		vcall!(self, findProfile(name.as_ptr()))
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct CompileRequest(IUnknown);

unsafe impl Interface for CompileRequest {
	type Vtable = sys::ICompileRequestVtable;
	const IID: UUID = uuid(0x96d33993, 0x317c, 0x4db5, [0xaf, 0xd8, 0x66, 0x6e, 0xe7, 0x72, 0x48, 0xe2]);
}

impl CompileRequest {
	pub fn set_codegen_target(&mut self, target: CompileTarget) -> &mut Self {
		vcall!(self, setCodeGenTarget(target));
		self
	}

	pub fn set_matrix_layout_mode(&mut self, layout: MatrixLayoutMode) -> &mut Self {
		vcall!(self, setMatrixLayoutMode(layout));
		self
	}

	pub fn set_optimization_level(&mut self, level: OptimizationLevel) -> &mut Self {
		vcall!(self, setOptimizationLevel(level));
		self
	}

	pub fn set_target_profile(&mut self, profile: ProfileID) ->&mut Self {
		vcall!(self, setTargetProfile(0, profile));
		self
	}

	pub fn add_preprocessor_define(&mut self, key: &str, value: &str) -> &mut Self {
		let key = CString::new(key).unwrap();
		let value = CString::new(value).unwrap();
		vcall!(self, addPreprocessorDefine(key.as_ptr(), value.as_ptr()));
		self
	}

	pub fn add_search_path(&mut self, path: impl AsRef<Path>) -> &mut Self {
		let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
		vcall!(self, addSearchPath(path.as_ptr()));
		self
	}

	pub fn add_translation_unit(&mut self, source_language: SourceLanguage, name: Option<&str>) -> TranslationUnit {
		let name = CString::new(name.unwrap_or("")).unwrap();
		let index = vcall!(self, addTranslationUnit(source_language, name.as_ptr()));

		TranslationUnit {
			request: self,
			index,
		}
	}

	pub fn compile(self) -> Result<CompiledRequest, CompilationErrors> {
		let r = vcall!(self, compile());

		if r < 0 {
			let out = vcall!(self, getDiagnosticOutput());
			let errors = unsafe { CStr::from_ptr(out).to_str().unwrap().to_string() };

			Err(CompilationErrors { errors })
		} else {
			Ok(CompiledRequest { request: self })
		}
	}
}

pub struct TranslationUnit<'a> {
	request: &'a mut CompileRequest,
	index: i32,
}

impl<'a> TranslationUnit<'a> {
	pub fn add_preprocessor_define(&mut self, key: &str, value: &str) -> &mut Self {
		let key = CString::new(key).unwrap();
		let value = CString::new(value).unwrap();
		vcall!(self.request, addTranslationUnitPreprocessorDefine(self.index, key.as_ptr(), value.as_ptr()));
		self
	}

	pub fn add_source_file(&mut self, path: impl AsRef<Path>) -> &mut Self {
		let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
		vcall!(self.request, addTranslationUnitSourceFile(self.index, path.as_ptr()));
		self
	}

	pub fn add_source_string(&mut self, path: impl AsRef<Path>, source: &str) -> &mut Self {
		let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
		let source = CString::new(source).unwrap();
		vcall!(self.request, addTranslationUnitSourceString(self.index, path.as_ptr(), source.as_ptr()));
		self
	}

	pub fn add_entry_point(&mut self, name: &str, stage: Stage) -> EntryPointIndex {
		let name = CString::new(name).unwrap();
		let index = vcall!(self.request, addEntryPoint(self.index, name.as_ptr(), stage));
		EntryPointIndex(index)
	}
}

pub struct CompiledRequest {
	request: CompileRequest,
}

impl CompiledRequest {
	pub fn get_entry_point_code(&self, index: EntryPointIndex) -> &[u8] {
		let mut out_size = 0;
		let ptr = vcall!(self.request, getEntryPointCode(index.0, &mut out_size));
		unsafe { slice::from_raw_parts(ptr as *const u8, out_size) }
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EntryPointIndex(pub i32);

pub struct CompilationErrors {
	errors: String,
}

impl std::fmt::Debug for CompilationErrors {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("\n")?;
		f.write_str(&self.errors)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn compiles() {}
}
