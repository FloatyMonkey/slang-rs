use crate as slang;
use slang::Downcast;

#[test]
fn compile() {
	let global_session = slang::GlobalSession::new().unwrap();

	let search_path = std::ffi::CString::new("shaders").unwrap();

	// All compiler options are available through this builder.
	let session_options = slang::CompilerOptions::default()
		.optimization(slang::OptimizationLevel::High)
		.matrix_layout_row(true);

	let target_desc = slang::TargetDesc::default()
		.format(slang::CompileTarget::Spirv)
		.profile(global_session.find_profile("glsl_450"));

	let targets = [target_desc];
	let search_paths = [search_path.as_ptr()];

	let session_desc = slang::SessionDesc::default()
		.targets(&targets)
		.search_paths(&search_paths)
		.options(&session_options);

	let session = global_session.create_session(&session_desc).unwrap();
	let module = session.load_module("test.slang").unwrap();
	let entry_point = module.find_entry_point_by_name("main").unwrap();

	let program = session
		.create_composite_component_type(&[
			module.downcast().clone(),
			entry_point.downcast().clone(),
		])
		.unwrap();

	let linked_program = program.link().unwrap();

	// Entry point to the reflection API.
	let reflection = linked_program.layout(0).unwrap();
	assert_eq!(reflection.entry_point_count(), 1);
	assert_eq!(reflection.parameter_count(), 3);

	let shader_bytecode = linked_program.entry_point_code(0, 0).unwrap();
	assert_ne!(shader_bytecode.as_slice().len(), 0);
}
