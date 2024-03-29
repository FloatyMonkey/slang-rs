#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{c_char, c_int, c_void};

// The vtables below are manually implemented since bindgen does not yet support
// generating vtables for types with base classes, a critical part of COM interfaces.

// Based on Slang version 2024.0.10

#[repr(C)]
pub struct IGlobalSessionVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub createSession: unsafe extern "stdcall" fn(*mut c_void, desc: *const slang_SessionDesc, outSession: *mut *mut slang_ISession) -> SlangResult,
	pub findProfile: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char) -> SlangProfileID,
	pub setDownstreamCompilerPath: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, path: *const c_char),
	#[deprecated( note = "Use setLanguagePrelude instead")]
	pub setDownstreamCompilerPrelude: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, preludeText: *const c_char),
	#[deprecated( note = "Use getLanguagePrelude instead")]
	pub getDownstreamCompilerPrelude: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, outPrelude: *mut *mut ISlangBlob),
	pub getBuildTagString: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	pub setDefaultDownstreamCompiler: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage, defaultCompiler: SlangPassThrough) -> SlangResult,
	pub getDefaultDownstreamCompiler: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage) -> SlangPassThrough,
	pub setLanguagePrelude: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage, preludeText: *const c_char),
	pub getLanguagePrelude: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage, outPrelude: *mut *mut ISlangBlob),
	pub createCompileRequest: unsafe extern "stdcall" fn(*mut c_void, *mut *mut c_void) -> SlangResult,
	pub addBuiltins: unsafe extern "stdcall" fn(*mut c_void, sourcePath: *const c_char, sourceString: *const c_char),
	pub setSharedLibraryLoader: unsafe extern "stdcall" fn(*mut c_void, loader: *mut ISlangSharedLibraryLoader),
	pub getSharedLibraryLoader: unsafe extern "stdcall" fn(*mut c_void) -> *mut ISlangSharedLibraryLoader,
	pub checkCompileTargetSupport: unsafe extern "stdcall" fn(*mut c_void, target: SlangCompileTarget) -> SlangResult,
	pub checkPassThroughSupport: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough) -> SlangResult,
	pub compileStdLib: unsafe extern "stdcall" fn(*mut c_void, flags: slang_CompileStdLibFlags) -> SlangResult,
	pub loadStdLib: unsafe extern "stdcall" fn(*mut c_void, stdLib: *const c_void, stdLibSizeInBytes: usize) -> SlangResult,
	pub saveStdLib: unsafe extern "stdcall" fn(*mut c_void, archiveType: SlangArchiveType, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub findCapability: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char) -> SlangCapabilityID,
	pub setDownstreamCompilerForTransition: unsafe extern "stdcall" fn(*mut c_void, source: SlangCompileTarget, target: SlangCompileTarget, compiler: SlangPassThrough),
	pub getDownstreamCompilerForTransition: unsafe extern "stdcall" fn(*mut c_void, source: SlangCompileTarget, target: SlangCompileTarget) -> SlangPassThrough,
	pub getCompilerElapsedTime: unsafe extern "stdcall" fn(*mut c_void, outTotalTime: *mut f64, outDownstreamTime: *mut f64),
	pub setSPIRVCoreGrammar: unsafe extern "stdcall" fn(*mut c_void, jsonPath: *const c_char) -> SlangResult,
	pub parseCommandLineArguments: unsafe extern "stdcall" fn(*mut c_void, argc: c_int, argv: *const *const c_char, outSessionDesc: *mut slang_SessionDesc, outAuxAllocation: *mut *mut ISlangUnknown) -> SlangResult,
}

#[repr(C)]
pub struct ICompileRequestVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub setFileSystem: unsafe extern "stdcall" fn(*mut c_void, fileSystem: *mut ISlangFileSystem),
	pub setCompileFlags: unsafe extern "stdcall" fn(*mut c_void, flags: SlangCompileFlags),
	pub getCompileFlags: unsafe extern "stdcall" fn(*mut c_void) -> SlangCompileFlags,
	pub setDumpIntermediates: unsafe extern "stdcall" fn(*mut c_void, enable: c_int),
	pub setDumpIntermediatePrefix: unsafe extern "stdcall" fn(*mut c_void, prefix: *const c_char),
	pub setLineDirectiveMode: unsafe extern "stdcall" fn(*mut c_void, mode: SlangLineDirectiveMode),
	pub setCodeGenTarget: unsafe extern "stdcall" fn(*mut c_void, target: SlangCompileTarget),
	pub addCodeGenTarget: unsafe extern "stdcall" fn(*mut c_void, target: SlangCompileTarget) -> c_int,
	pub setTargetProfile: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, profile: SlangProfileID),
	pub setTargetFlags: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, flags: SlangTargetFlags),
	pub setTargetFloatingPointMode: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, mode: SlangFloatingPointMode),
	#[deprecated( note = "Use setMatrixLayoutMode instead")]
	pub setTargetMatrixLayoutMode: unsafe extern "stdcall" fn(*mut c_void, target: c_int, mode: SlangMatrixLayoutMode),
	pub setMatrixLayoutMode: unsafe extern "stdcall" fn(*mut c_void, mode: SlangMatrixLayoutMode),
	pub setDebugInfoLevel: unsafe extern "stdcall" fn(*mut c_void, level: SlangDebugInfoLevel),
	pub setOptimizationLevel: unsafe extern "stdcall" fn(*mut c_void, level: SlangOptimizationLevel),
	pub setOutputContainerFormat: unsafe extern "stdcall" fn(*mut c_void, format: SlangContainerFormat),
	pub setPassThrough: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough),
	pub setDiagnosticCallback: unsafe extern "stdcall" fn(*mut c_void, callback: SlangDiagnosticCallback, userData: *const c_void),
	pub setWriter: unsafe extern "stdcall" fn(*mut c_void, channel: SlangWriterChannel, writer: *mut ISlangWriter),
	pub getWriter: unsafe extern "stdcall" fn(*mut c_void, channel: SlangWriterChannel) -> *mut ISlangWriter,
	pub addSearchPath: unsafe extern "stdcall" fn(*mut c_void, searchDir: *const c_char),
	pub addPreprocessorDefine: unsafe extern "stdcall" fn(*mut c_void, key: *const c_char, value: *const c_char),
	pub processCommandLineArguments: unsafe extern "stdcall" fn(*mut c_void, args: *const *const c_char, argCount: c_int) -> SlangResult,
	pub addTranslationUnit: unsafe extern "stdcall" fn(*mut c_void, language: SlangSourceLanguage, name: *const c_char) -> c_int,
	pub setDefaultModuleName: unsafe extern "stdcall" fn(*mut c_void, defaultModuleName: *const c_char),
	pub addTranslationUnitPreprocessorDefine: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, key: *const c_char, value: *const c_char),
	pub addTranslationUnitSourceFile: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, path: *const c_char),
	pub addTranslationUnitSourceString: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, path: *const c_char, source: *const c_char),
	pub addLibraryReference: unsafe extern "stdcall" fn(*mut c_void, basePath: *const c_char, libData: *const c_void, libDataSize: usize) -> SlangResult,
	pub addTranslationUnitSourceStringSpan: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, path: *const c_char, sourceBegin: *const c_char, sourceEnd: *const c_char),
	pub addTranslationUnitSourceBlob: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, path: *const c_char, sourceBlob: *mut ISlangBlob),
	pub addEntryPoint: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, name: *const c_char, stage: SlangStage) -> c_int,
	pub addEntryPointEx: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, name: *const c_char, stage: SlangStage, genericArgCount: c_int, genericArgs: *const *const c_char) -> c_int,
	pub setGlobalGenericArgs: unsafe extern "stdcall" fn(*mut c_void, genericArgCount: c_int, genericArgs: *const *const c_char) -> SlangResult,
	pub setTypeNameForGlobalExistentialTypeParam: unsafe extern "stdcall" fn(*mut c_void, slotIndex: c_int, typeName: *const c_char) -> SlangResult,
	pub setTypeNameForEntryPointExistentialTypeParam: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, slotIndex: c_int, typeName: *const c_char) -> SlangResult,
	pub setAllowGLSLInput: unsafe extern "stdcall" fn(*mut c_void, value: bool),
	pub compile: unsafe extern "stdcall" fn(*mut c_void) -> SlangResult,
	pub getDiagnosticOutput: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	pub getDiagnosticOutputBlob: unsafe extern "stdcall" fn(*mut c_void, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub getDependencyFileCount: unsafe extern "stdcall" fn(*mut c_void) -> c_int,
	pub getDependencyFilePath: unsafe extern "stdcall" fn(*mut c_void, index: c_int) -> *const c_char,
	pub getTranslationUnitCount: unsafe extern "stdcall" fn(*mut c_void) -> c_int,
	pub getEntryPointSource: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int) -> *const c_char,
	pub getEntryPointCode: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, outSize: *mut usize) -> *const c_void,
	pub getEntryPointCodeBlob: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, targetIndex: c_int, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub getEntryPointHostCallable: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, targetIndex: c_int, outSharedLibrary: *mut *mut ISlangSharedLibrary) -> SlangResult,
	pub getTargetCodeBlob: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub getTargetHostCallable: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, outSharedLibrary: *mut *mut ISlangSharedLibrary) -> SlangResult,
	pub getCompileRequestCode: unsafe extern "stdcall" fn(*mut c_void, outSize: *mut usize) -> *const c_void,
	pub getCompileRequestResultAsFileSystem: unsafe extern "stdcall" fn(*mut c_void) -> *mut ISlangMutableFileSystem,
	pub getContainerCode: unsafe extern "stdcall" fn(*mut c_void, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub loadRepro: unsafe extern "stdcall" fn(*mut c_void, fileSystem: *mut ISlangFileSystem, data: *const c_void, size: usize) -> SlangResult,
	pub saveRepro: unsafe extern "stdcall" fn(*mut c_void, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub enableReproCapture: unsafe extern "stdcall" fn(*mut c_void) -> SlangResult,
	pub getProgram: unsafe extern "stdcall" fn(*mut c_void, outProgram: *mut *mut slang_IComponentType) -> SlangResult,
	pub getEntryPoint: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, outEntryPoint: *mut *mut slang_IComponentType) -> SlangResult,
	pub getModule: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, outModule: *mut *mut slang_IModule) -> SlangResult,
	pub getSession: unsafe extern "stdcall" fn(*mut c_void, outSession: *mut *mut slang_ISession) -> SlangResult,
	pub getReflection: unsafe extern "stdcall" fn(*mut c_void) -> *mut SlangReflection,
	pub setCommandLineCompilerMode: unsafe extern "stdcall" fn(*mut c_void),
	pub addTargetCapability: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, capability: SlangCapabilityID) -> SlangResult,
	pub getProgramWithEntryPoints: unsafe extern "stdcall" fn(*mut c_void, outProgram: *mut *mut slang_IComponentType) -> SlangResult,
	pub isParameterLocationUsed: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, targetIndex: c_int, category: SlangParameterCategory, spaceIndex: SlangUInt, registerIndex: SlangUInt, outUsed: *mut bool) -> SlangResult,
	pub setTargetLineDirectiveMode: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, mode: SlangLineDirectiveMode),
	pub setTargetForceGLSLScalarBufferLayout: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, forceScalarLayout: bool),
	pub overrideDiagnosticSeverity: unsafe extern "stdcall" fn(*mut c_void, messageID: SlangInt, overrideSeverity: SlangSeverity),
	pub getDiagnosticFlags: unsafe extern "stdcall" fn(*mut c_void) -> SlangDiagnosticFlags,
	pub setDiagnosticFlags: unsafe extern "stdcall" fn(*mut c_void, flags: SlangDiagnosticFlags),
	pub setDebugInfoFormat: unsafe extern "stdcall" fn(*mut c_void, debugFormat: SlangDebugInfoFormat),
	pub setEnableEffectAnnotations: unsafe extern "stdcall" fn(*mut c_void, value: bool),
	pub setReportDownstreamTime: unsafe extern "stdcall" fn(*mut c_void, value: bool),
	pub setReportPerfBenchmark: unsafe extern "stdcall" fn(*mut c_void, value: bool),
	pub setSkipSPIRVValidation: unsafe extern "stdcall" fn(*mut c_void, value: bool),
}
