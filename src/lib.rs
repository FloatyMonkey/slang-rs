//! Rust bindings for the Slang shader language compiler

pub mod reflection;

#[cfg(test)]
mod tests;

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr::{null, null_mut};

pub(crate) use shader_slang_sys as sys;

pub use sys::{
	SlangBindingType as BindingType, SlangCompileTarget as CompileTarget,
	SlangDebugInfoLevel as DebugInfoLevel, SlangDeclKind as DeclKind,
	SlangFloatingPointMode as FloatingPointMode, SlangImageFormat as ImageFormat,
	SlangLayoutRules as LayoutRules, SlangLineDirectiveMode as LineDirectiveMode,
	SlangMatrixLayoutMode as MatrixLayoutMode, SlangModifierID as ModifierID,
	SlangOptimizationLevel as OptimizationLevel, SlangParameterCategory as ParameterCategory,
	SlangReflectionGenericArg as GenericArg, SlangReflectionGenericArgType as GenericArgType,
	SlangResourceAccess as ResourceAccess, SlangResourceShape as ResourceShape,
	SlangScalarType as ScalarType, SlangSourceLanguage as SourceLanguage, SlangStage as Stage,
	SlangTypeKind as TypeKind, SlangUUID as UUID, slang_CompilerOptionName as CompilerOptionName,
	slang_Modifier as Modifier,
};

macro_rules! vcall {
	($self:expr, $method:ident($($args:expr),*)) => {
		unsafe { ($self.vtable().$method)($self.as_raw(), $($args),*) }
	};
}

const fn uuid(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> UUID {
	UUID {
		data1,
		data2,
		data3,
		data4,
	}
}

pub enum Error {
	Code(sys::SlangResult),
	Blob(Blob),
}

impl std::fmt::Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Code(code) => write!(f, "{}", code),
			Error::Blob(blob) => write!(f, "{}", blob.as_str().unwrap_or_default()),
		}
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(self, f)
	}
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}
impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn succeeded(result: sys::SlangResult) -> bool {
	result >= 0
}

fn result_from_blob(code: sys::SlangResult, blob: *mut sys::slang_IBlob) -> Result<()> {
	if code < 0 && !blob.is_null() {
		Err(Error::Blob(Blob(IUnknown(
			std::ptr::NonNull::new(blob as *mut _).unwrap(),
		))))
	} else if code < 0 {
		Err(Error::Code(code))
	} else {
		Ok(())
	}
}

#[derive(Clone, Copy)]
pub struct ProfileID(sys::SlangProfileID);

impl ProfileID {
	pub const UNKNOWN: ProfileID = ProfileID(sys::SlangProfileID_SlangProfileUnknown);

	pub fn is_unknown(&self) -> bool {
		self.0 == sys::SlangProfileID_SlangProfileUnknown
	}
}

#[derive(Clone, Copy)]
pub struct CapabilityID(sys::SlangCapabilityID);

impl CapabilityID {
	pub const UNKNOWN: CapabilityID = CapabilityID(sys::SlangCapabilityID_SlangCapabilityUnknown);

	pub fn is_unknown(&self) -> bool {
		self.0 == sys::SlangCapabilityID_SlangCapabilityUnknown
	}
}

unsafe trait Interface: Sized {
	type Vtable;
	const IID: UUID;

	#[inline(always)]
	unsafe fn vtable(&self) -> &Self::Vtable {
		unsafe { &**(self.as_raw() as *mut *mut Self::Vtable) }
	}

	#[inline(always)]
	unsafe fn as_raw<T>(&self) -> *mut T {
		unsafe { std::mem::transmute_copy(self) }
	}

	fn as_unknown(&self) -> &IUnknown {
		// SAFETY: It is always safe to treat an `Interface` as an `IUnknown`.
		unsafe { std::mem::transmute(self) }
	}
}

pub unsafe trait Downcast<T> {
	fn downcast(&self) -> &T;
}

#[repr(transparent)]
pub struct IUnknown(std::ptr::NonNull<std::ffi::c_void>);

unsafe impl Interface for IUnknown {
	type Vtable = sys::ISlangUnknown__bindgen_vtable;
	const IID: UUID = uuid(
		0x00000000,
		0x0000,
		0x0000,
		[0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
	);
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
pub struct Blob(IUnknown);

unsafe impl Interface for Blob {
	type Vtable = sys::IBlobVtable;
	const IID: UUID = uuid(
		0x8ba5fb08,
		0x5195,
		0x40e2,
		[0xac, 0x58, 0x0d, 0x98, 0x9c, 0x3a, 0x01, 0x02],
	);
}

impl Blob {
	pub fn as_slice(&self) -> &[u8] {
		let ptr = vcall!(self, getBufferPointer());
		let size = vcall!(self, getBufferSize());
		unsafe { std::slice::from_raw_parts(ptr as *const u8, size) }
	}

	pub fn as_str(&self) -> std::result::Result<&str, std::str::Utf8Error> {
		std::str::from_utf8(self.as_slice())
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct GlobalSession(IUnknown);

unsafe impl Interface for GlobalSession {
	type Vtable = sys::IGlobalSessionVtable;
	const IID: UUID = uuid(
		0xc140b5fd,
		0x0c78,
		0x452e,
		[0xba, 0x7c, 0x1a, 0x1e, 0x70, 0xc7, 0xf7, 0x1c],
	);
}

impl GlobalSession {
	pub fn new() -> Option<GlobalSession> {
		let mut global_session = null_mut();
		unsafe { sys::slang_createGlobalSession(sys::SLANG_API_VERSION as _, &mut global_session) };
		Some(GlobalSession(IUnknown(std::ptr::NonNull::new(
			global_session as *mut _,
		)?)))
	}

	pub fn new_without_core_module() -> Option<GlobalSession> {
		let mut global_session = null_mut();
		unsafe {
			sys::slang_createGlobalSessionWithoutCoreModule(
				sys::SLANG_API_VERSION as _,
				&mut global_session,
			)
		};
		Some(GlobalSession(IUnknown(std::ptr::NonNull::new(
			global_session as *mut _,
		)?)))
	}

	pub fn create_session(&self, desc: &SessionDesc) -> Option<Session> {
		let mut session = null_mut();
		vcall!(self, createSession(&**desc, &mut session));
		Some(Session(IUnknown(std::ptr::NonNull::new(
			session as *mut _,
		)?)))
	}

	pub fn find_profile(&self, name: &str) -> ProfileID {
		let name = CString::new(name).unwrap();
		ProfileID(vcall!(self, findProfile(name.as_ptr())))
	}

	pub fn find_capability(&self, name: &str) -> CapabilityID {
		let name = CString::new(name).unwrap();
		CapabilityID(vcall!(self, findCapability(name.as_ptr())))
	}

	pub fn build_tag_string(&self) -> &str {
		let tag = vcall!(self, getBuildTagString());
		unsafe { CStr::from_ptr(tag).to_str().unwrap() }
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Session(IUnknown);

unsafe impl Interface for Session {
	type Vtable = sys::ISessionVtable;
	const IID: UUID = uuid(
		0x67618701,
		0xd116,
		0x468f,
		[0xab, 0x3b, 0x47, 0x4b, 0xed, 0xce, 0x0e, 0x3d],
	);
}

impl Session {
	pub fn load_module(&self, name: &str) -> Result<Module> {
		let name = CString::new(name).unwrap();
		let mut diagnostics = null_mut();

		let module = vcall!(self, loadModule(name.as_ptr(), &mut diagnostics));

		if module.is_null() {
			let blob = Blob(IUnknown(
				std::ptr::NonNull::new(diagnostics as *mut _).unwrap(),
			));
			Err(Error::Blob(blob))
		} else {
			let module = Module(IUnknown(std::ptr::NonNull::new(module as *mut _).unwrap()));
			unsafe { (module.as_unknown().vtable().ISlangUnknown_addRef)(module.as_raw()) };
			Ok(module)
		}
	}

	pub fn load_module_from_source_string(
		&self,
		module_name: &str,
		path: &str,
		source: &str,
	) -> Result<Module> {
		let module_name = CString::new(module_name).unwrap();
		let path = CString::new(path).unwrap();
		let source = CString::new(source).unwrap();
		let mut diagnostics = null_mut();

		let module = vcall!(
			self,
			loadModuleFromSourceString(
				module_name.as_ptr(),
				path.as_ptr(),
				source.as_ptr(),
				&mut diagnostics
			)
		);

		if module.is_null() {
			let blob = Blob(IUnknown(
				std::ptr::NonNull::new(diagnostics as *mut _).unwrap(),
			));
			Err(Error::Blob(blob))
		} else {
			let module = Module(IUnknown(std::ptr::NonNull::new(module as *mut _).unwrap()));
			unsafe { (module.as_unknown().vtable().ISlangUnknown_addRef)(module.as_raw()) };
			Ok(module)
		}
	}

	pub fn load_module_from_ir_blob(
		&self,
		module_name: &str,
		path: &str,
		ir_blob: &Blob,
	) -> Result<Module> {
		let module_name = CString::new(module_name).unwrap();
		let path = CString::new(path).unwrap();
		let mut diagnostics = null_mut();

		let module = vcall!(
			self,
			loadModuleFromIRBlob(
				module_name.as_ptr(),
				path.as_ptr(),
				ir_blob.as_raw(),
				&mut diagnostics
			)
		);

		if module.is_null() {
			let blob = Blob(IUnknown(
				std::ptr::NonNull::new(diagnostics as *mut _).unwrap(),
			));
			Err(Error::Blob(blob))
		} else {
			let module = Module(IUnknown(std::ptr::NonNull::new(module as *mut _).unwrap()));
			unsafe { (module.as_unknown().vtable().ISlangUnknown_addRef)(module.as_raw()) };
			Ok(module)
		}
	}

	pub fn create_composite_component_type(
		&self,
		components: &[ComponentType],
	) -> Result<ComponentType> {
		let mut composite_component_type = null_mut();
		let mut diagnostics = null_mut();

		result_from_blob(
			vcall!(
				self,
				createCompositeComponentType(
					components.as_ptr() as _,
					components.len() as _,
					&mut composite_component_type,
					&mut diagnostics
				)
			),
			diagnostics,
		)?;

		Ok(ComponentType(IUnknown(
			std::ptr::NonNull::new(composite_component_type as *mut _).unwrap(),
		)))
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Metadata(IUnknown);

unsafe impl Interface for Metadata {
	type Vtable = sys::IMetadataVtable;
	const IID: UUID = uuid(
		0x8044a8a3,
		0xddc0,
		0x4b7f,
		[0xaf, 0x8e, 0x2, 0x6e, 0x90, 0x5d, 0x73, 0x32],
	);
}

impl Metadata {
	pub fn is_parameter_location_used(
		&self,
		category: ParameterCategory,
		space_index: u64,
		register_index: u64,
	) -> Option<bool> {
		let mut used = false;
		let result = vcall!(
			self,
			isParameterLocationUsed(category, space_index, register_index, &mut used)
		);
		succeeded(result).then(|| used)
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ComponentType(IUnknown);

unsafe impl Interface for ComponentType {
	type Vtable = sys::IComponentTypeVtable;
	const IID: UUID = uuid(
		0x5bc42be8,
		0x5c50,
		0x4929,
		[0x9e, 0x5e, 0xd1, 0x5e, 0x7c, 0x24, 0x01, 0x5f],
	);
}

impl ComponentType {
	pub fn layout(&self, target: i64) -> Result<&reflection::Shader> {
		let mut diagnostics = null_mut();
		let ptr = vcall!(self, getLayout(target, &mut diagnostics));

		if ptr.is_null() {
			Err(Error::Blob(Blob(IUnknown(
				std::ptr::NonNull::new(diagnostics as *mut _).unwrap(),
			))))
		} else {
			Ok(unsafe { &*(ptr as *const _) })
		}
	}

	pub fn link(&self) -> Result<ComponentType> {
		let mut linked_component_type = null_mut();
		let mut diagnostics = null_mut();

		result_from_blob(
			vcall!(self, link(&mut linked_component_type, &mut diagnostics)),
			diagnostics,
		)?;

		Ok(ComponentType(IUnknown(
			std::ptr::NonNull::new(linked_component_type as *mut _).unwrap(),
		)))
	}

	pub fn target_code(&self, target: i64) -> Result<Blob> {
		let mut code = null_mut();
		let mut diagnostics = null_mut();

		result_from_blob(
			vcall!(self, getTargetCode(target, &mut code, &mut diagnostics)),
			diagnostics,
		)?;

		Ok(Blob(IUnknown(
			std::ptr::NonNull::new(code as *mut _).unwrap(),
		)))
	}

	pub fn entry_point_code(&self, index: i64, target: i64) -> Result<Blob> {
		let mut code = null_mut();
		let mut diagnostics = null_mut();

		result_from_blob(
			vcall!(
				self,
				getEntryPointCode(index, target, &mut code, &mut diagnostics)
			),
			diagnostics,
		)?;

		Ok(Blob(IUnknown(
			std::ptr::NonNull::new(code as *mut _).unwrap(),
		)))
	}

	pub fn target_metadata(&self, target_index: i64) -> Result<Metadata> {
		let mut metadata = null_mut();
		let mut diagnostics = null_mut();

		result_from_blob(
			vcall!(
				self,
				getTargetMetadata(target_index, &mut metadata, &mut diagnostics)
			),
			diagnostics,
		)?;

		Ok(Metadata(IUnknown(
			std::ptr::NonNull::new(metadata as *mut _).unwrap(),
		)))
	}

	pub fn entry_point_metadata(
		&self,
		entry_point_index: i64,
		target_index: i64,
	) -> Result<Metadata> {
		let mut metadata = null_mut();
		let mut diagnostics = null_mut();

		result_from_blob(
			vcall!(
				self,
				getEntryPointMetadata(
					entry_point_index,
					target_index,
					&mut metadata,
					&mut diagnostics
				)
			),
			diagnostics,
		)?;

		Ok(Metadata(IUnknown(
			std::ptr::NonNull::new(metadata as *mut _).unwrap(),
		)))
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct EntryPoint(IUnknown);

unsafe impl Interface for EntryPoint {
	type Vtable = sys::IEntryPointVtable;
	const IID: UUID = uuid(
		0x8f241361,
		0xf5bd,
		0x4ca0,
		[0xa3, 0xac, 0x02, 0xf7, 0xfa, 0x24, 0x02, 0xb8],
	);
}

unsafe impl Downcast<ComponentType> for EntryPoint {
	fn downcast(&self) -> &ComponentType {
		unsafe { std::mem::transmute(self) }
	}
}

impl EntryPoint {
	pub fn function_reflection(&self) -> &reflection::Function {
		let ptr = vcall!(self, getFunctionReflection());
		unsafe { &*(ptr as *const _) }
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct TypeConformance(IUnknown);

unsafe impl Interface for TypeConformance {
	type Vtable = sys::ITypeConformanceVtable;
	const IID: UUID = uuid(
		0x73eb3147,
		0xe544,
		0x41b5,
		[0xb8, 0xf0, 0xa2, 0x44, 0xdf, 0x21, 0x94, 0x0b],
	);
}

unsafe impl Downcast<ComponentType> for TypeConformance {
	fn downcast(&self) -> &ComponentType {
		unsafe { std::mem::transmute(self) }
	}
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Module(IUnknown);

unsafe impl Interface for Module {
	type Vtable = sys::IModuleVtable;
	const IID: UUID = uuid(
		0x0c720e64,
		0x8722,
		0x4d31,
		[0x89, 0x90, 0x63, 0x8a, 0x98, 0xb1, 0xc2, 0x79],
	);
}

unsafe impl Downcast<ComponentType> for Module {
	fn downcast(&self) -> &ComponentType {
		unsafe { std::mem::transmute(self) }
	}
}

impl Module {
	pub fn find_entry_point_by_name(&self, name: &str) -> Option<EntryPoint> {
		let name = CString::new(name).unwrap();
		let mut entry_point = null_mut();
		vcall!(self, findEntryPointByName(name.as_ptr(), &mut entry_point));
		Some(EntryPoint(IUnknown(std::ptr::NonNull::new(
			entry_point as *mut _,
		)?)))
	}

	pub fn entry_point_count(&self) -> u32 {
		vcall!(self, getDefinedEntryPointCount()) as _
	}

	pub fn entry_point_by_index(&self, index: u32) -> Option<EntryPoint> {
		let mut entry_point = null_mut();
		vcall!(self, getDefinedEntryPoint(index as _, &mut entry_point));
		Some(EntryPoint(IUnknown(std::ptr::NonNull::new(
			entry_point as *mut _,
		)?)))
	}

	pub fn entry_points(&self) -> impl ExactSizeIterator<Item = EntryPoint> {
		(0..self.entry_point_count()).map(|i| self.entry_point_by_index(i).unwrap())
	}

	pub fn name(&self) -> &str {
		let name = vcall!(self, getName());
		unsafe { CStr::from_ptr(name).to_str().unwrap() }
	}

	pub fn file_path(&self) -> &str {
		let path = vcall!(self, getFilePath());
		unsafe { CStr::from_ptr(path).to_str().unwrap() }
	}

	pub fn unique_identity(&self) -> &str {
		let identity = vcall!(self, getUniqueIdentity());
		unsafe { CStr::from_ptr(identity).to_str().unwrap() }
	}

	pub fn dependency_file_count(&self) -> i32 {
		vcall!(self, getDependencyFileCount()) as i32
	}

	pub fn dependency_file_path(&self, index: i32) -> &str {
		let path = vcall!(self, getDependencyFilePath(index as i32));
		unsafe { CStr::from_ptr(path).to_str().unwrap() }
	}

	pub fn dependency_file_paths(&self) -> impl ExactSizeIterator<Item = &str> {
		(0..self.dependency_file_count()).map(|i| self.dependency_file_path(i))
	}

	pub fn module_reflection(&self) -> &reflection::Decl {
		let ptr = vcall!(self, getModuleReflection());
		unsafe { &*(ptr as *const _) }
	}
}

#[repr(transparent)]
pub struct TargetDesc<'a> {
	inner: sys::slang_TargetDesc,
	_phantom: PhantomData<&'a ()>,
}

impl std::ops::Deref for TargetDesc<'_> {
	type Target = sys::slang_TargetDesc;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl Default for TargetDesc<'_> {
	fn default() -> Self {
		Self {
			inner: sys::slang_TargetDesc {
				structureSize: std::mem::size_of::<sys::slang_TargetDesc>(),
				..unsafe { std::mem::zeroed() }
			},
			_phantom: PhantomData,
		}
	}
}

impl<'a> TargetDesc<'a> {
	pub fn format(mut self, format: CompileTarget) -> Self {
		self.inner.format = format;
		self
	}

	pub fn profile(mut self, profile: ProfileID) -> Self {
		self.inner.profile = profile.0;
		self
	}

	pub fn options(mut self, options: &'a CompilerOptions) -> Self {
		self.inner.compilerOptionEntries = options.options.as_ptr() as _;
		self.inner.compilerOptionEntryCount = options.options.len() as _;
		self
	}
}

#[repr(transparent)]
pub struct SessionDesc<'a> {
	inner: sys::slang_SessionDesc,
	_phantom: PhantomData<&'a ()>,
}

impl std::ops::Deref for SessionDesc<'_> {
	type Target = sys::slang_SessionDesc;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl Default for SessionDesc<'_> {
	fn default() -> Self {
		Self {
			inner: sys::slang_SessionDesc {
				structureSize: std::mem::size_of::<sys::slang_SessionDesc>(),
				..unsafe { std::mem::zeroed() }
			},
			_phantom: PhantomData,
		}
	}
}

impl<'a> SessionDesc<'a> {
	pub fn targets(mut self, targets: &'a [TargetDesc]) -> Self {
		self.inner.targets = targets.as_ptr() as _;
		self.inner.targetCount = targets.len() as _;
		self
	}

	pub fn search_paths(mut self, paths: &'a [*const i8]) -> Self {
		self.inner.searchPaths = paths.as_ptr();
		self.inner.searchPathCount = paths.len() as _;
		self
	}

	pub fn options(mut self, options: &'a CompilerOptions) -> Self {
		self.inner.compilerOptionEntries = options.options.as_ptr() as _;
		self.inner.compilerOptionEntryCount = options.options.len() as _;
		self
	}
}

macro_rules! option {
	($name:ident, $func:ident($p_name:ident: $p_type:ident)) => {
		#[inline(always)]
		pub fn $func(self, $p_name: $p_type) -> Self {
			self.push_ints(CompilerOptionName::$name, $p_name as _, 0)
		}
	};

	($name:ident, $func:ident($p_name:ident: &str)) => {
		#[inline(always)]
		pub fn $func(self, $p_name: &str) -> Self {
			self.push_str1(CompilerOptionName::$name, $p_name)
		}
	};

	($name:ident, $func:ident($p_name1:ident: &str, $p_name2:ident: &str)) => {
		#[inline(always)]
		pub fn $func(self, $p_name1: &str, $p_name2: &str) -> Self {
			self.push_str2(CompilerOptionName::$name, $p_name1, $p_name2)
		}
	};
}

#[derive(Default)]
pub struct CompilerOptions {
	strings: Vec<CString>,
	options: Vec<sys::slang_CompilerOptionEntry>,
}

impl CompilerOptions {
	fn push_ints(mut self, name: CompilerOptionName, i0: i32, i1: i32) -> Self {
		self.options.push(sys::slang_CompilerOptionEntry {
			name,
			value: sys::slang_CompilerOptionValue {
				kind: sys::slang_CompilerOptionValueKind::Int,
				intValue0: i0,
				intValue1: i1,
				stringValue0: null(),
				stringValue1: null(),
			},
		});

		self
	}

	fn push_strings(mut self, name: CompilerOptionName, s0: *const i8, s1: *const i8) -> Self {
		self.options.push(sys::slang_CompilerOptionEntry {
			name,
			value: sys::slang_CompilerOptionValue {
				kind: sys::slang_CompilerOptionValueKind::String,
				intValue0: 0,
				intValue1: 0,
				stringValue0: s0,
				stringValue1: s1,
			},
		});

		self
	}

	fn push_str1(mut self, name: CompilerOptionName, s0: &str) -> Self {
		let s0 = CString::new(s0).unwrap();
		let s0_ptr = s0.as_ptr();
		self.strings.push(s0);

		self.push_strings(name, s0_ptr, null())
	}

	fn push_str2(mut self, name: CompilerOptionName, s0: &str, s1: &str) -> Self {
		let s0 = CString::new(s0).unwrap();
		let s0_ptr = s0.as_ptr();
		self.strings.push(s0);

		let s1 = CString::new(s1).unwrap();
		let s1_ptr = s1.as_ptr();
		self.strings.push(s1);

		self.push_strings(name, s0_ptr, s1_ptr)
	}
}

impl CompilerOptions {
	option!(MacroDefine, macro_define(key: &str, value: &str));
	option!(Include, include(path: &str));
	option!(Language, language(language: SourceLanguage));
	option!(MatrixLayoutColumn, matrix_layout_column(enable: bool));
	option!(MatrixLayoutRow, matrix_layout_row(enable: bool));

	#[inline(always)]
	pub fn profile(self, profile: ProfileID) -> Self {
		self.push_ints(CompilerOptionName::Profile, profile.0 as _, 0)
	}

	option!(Stage, stage(stage: Stage));
	option!(Target, target(target: CompileTarget));
	option!(WarningsAsErrors, warnings_as_errors(warning_codes: &str));
	option!(DisableWarnings, disable_warnings(warning_codes: &str));
	option!(EnableWarning, enable_warning(warning_code: &str));
	option!(DisableWarning, disable_warning(warning_code: &str));
	option!(ReportDownstreamTime, report_downstream_time(enable: bool));
	option!(ReportPerfBenchmark, report_perf_benchmark(enable: bool));
	option!(SkipSPIRVValidation, skip_spirv_validation(enable: bool));

	// Target
	#[inline(always)]
	pub fn capability(self, capability: CapabilityID) -> Self {
		self.push_ints(CompilerOptionName::Capability, capability.0 as _, 0)
	}

	option!(DefaultImageFormatUnknown, default_image_format_unknown(enable: bool));
	option!(DisableDynamicDispatch, disable_dynamic_dispatch(enable: bool));
	option!(DisableSpecialization, disable_specialization(enable: bool));
	option!(FloatingPointMode, floating_point_mode(mode: FloatingPointMode));
	option!(DebugInformation, debug_information(level: DebugInfoLevel));
	option!(LineDirectiveMode, line_directive_mode(mode: LineDirectiveMode));
	option!(Optimization, optimization(level: OptimizationLevel));
	option!(Obfuscate, obfuscate(enable: bool));
	option!(VulkanUseEntryPointName, vulkan_use_entry_point_name(enable: bool));
	option!(GLSLForceScalarLayout, glsl_force_scalar_layout(enable: bool));
	option!(EmitSpirvDirectly, emit_spirv_directly(enable: bool));

	// Debugging
	option!(NoCodeGen, no_code_gen(enable: bool));

	// Experimental
	option!(NoMangle, no_mangle(enable: bool));
	option!(ValidateUniformity, validate_uniformity(enable: bool));
}
