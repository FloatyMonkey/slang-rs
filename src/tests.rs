use crate as slang;
use crate::fs::FileSystemTrait;
use crate::{Blob, IUnknown, Interface};
use std::collections::HashMap;

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
		.create_composite_component_type(&[module.into(), entry_point.into()])
		.unwrap();

	let linked_program = program.link().unwrap();

	// Entry point to the reflection API.
	let reflection = linked_program.layout(0).unwrap();
	assert_eq!(reflection.entry_point_count(), 1);
	assert_eq!(reflection.parameter_count(), 3);

	let shader_bytecode = linked_program.entry_point_code(0, 0).unwrap();
	assert_ne!(shader_bytecode.as_slice().len(), 0);
}


struct TestFileSystem {
	files: HashMap<String, Vec<u8>>,
}

impl TestFileSystem {
	fn new(files: impl IntoIterator<Item = (String, Vec<u8>)>) -> Self {
		Self {
			files: files.into_iter().collect(),
		}
	}
}

impl FileSystemTrait for TestFileSystem {
	fn load_file(&self, path: &str) -> Option<Blob> {
		let data = self.files.get(path)?;
		
		Some(Blob::new(data))
	}
}

#[test]
fn test_file_system_with_slang_compilation() {
	let vertex_shader = r#"
		[shader("vertex")]
		void vsMain(
			float3 position : POSITION,
			float2 uv : TEXCOORD0,
			out float4 outPosition : SV_Position,
			out float2 outUv : TEXCOORD0
		) {
			outPosition = float4(position, 1.0);
			outUv = uv;
		}
	"#.as_bytes().to_vec();
	
	let test_fs = TestFileSystem::new([("shader.slang".to_string(), vertex_shader)]);
	
	let global_session = slang::GlobalSession::new().unwrap();
	
	let target_desc = slang::TargetDesc::default()
		.format(slang::CompileTarget::Spirv)
		.profile(global_session.find_profile("glsl_450"));
	
	let targets = [target_desc];
	
	let session_desc = slang::SessionDesc::default()
		.targets(&targets)
		.file_system(test_fs);
	
	let session = global_session.create_session(&session_desc).unwrap();
	
	let module = session.load_module("shader.slang").unwrap();
	
	let module_name = module.name();
	assert!(module_name.contains("shader"), "Module name should contain 'shader', got: {}", module_name);
	
	let entry_point = module.find_entry_point_by_name("vsMain").unwrap();
	
	let program = session
		.create_composite_component_type(&[module.into(), entry_point.into()])
		.unwrap();
	
	let linked_program = program.link().unwrap();
	
	let reflection = linked_program.layout(0).unwrap();
	assert_eq!(reflection.entry_point_count(), 1);
	
	let shader_bytecode = linked_program.entry_point_code(0, 0).unwrap();
	assert_ne!(shader_bytecode.as_slice().len(), 0, "Shader bytecode should not be empty");
}

#[test]
fn test_file_system_with_includes() {
	let common = r#"
		float4 multiplyColor(float4 color, float factor) {
			return color * factor;
		}
	"#.as_bytes().to_vec();
	
	let main = r#"
		#include "common.slang"
		
		[shader("fragment")]
		void fsMain(
			float4 position : SV_Position,
			float2 uv : TEXCOORD0,
			out float4 outColor : SV_Target
		) {
			float4 baseColor = float4(uv, 0.0, 1.0);
			outColor = multiplyColor(baseColor, 2.0);
		}
	"#.as_bytes().to_vec();
	
	let fs = TestFileSystem::new([
		("common.slang".to_string(), common),
		("main.slang".to_string(), main),
	]);
	
	let global_session = slang::GlobalSession::new().unwrap();
	
	let target_desc = slang::TargetDesc::default()
		.format(slang::CompileTarget::Spirv)
		.profile(global_session.find_profile("glsl_450"));
	
	let targets = [target_desc];
	
	let session_desc = slang::SessionDesc::default()
		.targets(&targets)
		.file_system(fs);
	
	let session = global_session.create_session(&session_desc).unwrap();
	
	let module = session.load_module("main.slang").unwrap();
	
	let module_name = module.name();
	assert!(module_name.contains("main"), "Module name should contain 'main', got: {}", module_name);
	
	// Find the entry point
	let entry_point = module.find_entry_point_by_name("fsMain").unwrap();
	
	// Create and link the program
	let program = session
		.create_composite_component_type(&[module.into(), entry_point.into()])
		.unwrap();
	
	let linked_program = program.link().unwrap();
	
	// Get reflection and verify it works
	let reflection = linked_program.layout(0).unwrap();
	assert_eq!(reflection.entry_point_count(), 1);
	
	// Get the compiled bytecode and verify it was generated
	let shader_bytecode = linked_program.entry_point_code(0, 0).unwrap();
	assert_ne!(shader_bytecode.as_slice().len(), 0, "Shader bytecode should not be empty");
}

#[test]
fn test_file_system_error_handling() {
	let empty_fs = TestFileSystem::new(std::iter::empty());
	
	let global_session = slang::GlobalSession::new().unwrap();
	let target_desc = slang::TargetDesc::default()
		.format(slang::CompileTarget::Spirv)
		.profile(global_session.find_profile("glsl_450"));
	
	let targets = [target_desc];
	
	let session_desc = slang::SessionDesc::default()
		.targets(&targets)
		.file_system(empty_fs);
	
	let session = global_session.create_session(&session_desc).unwrap();
	
	let result = session.load_module("nonexistent.slang");
	assert!(result.is_err(), "Loading non-existent module should fail");
}
