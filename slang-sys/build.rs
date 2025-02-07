extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
	println!("cargo:rerun-if-env-changed=SLANG_DIR");
	println!("cargo:rerun-if-env-changed=VULKAN_SDK");
	let mut include_file = PathBuf::from("include");
	let slang_dir = if let Ok(slang_dir) = env::var("SLANG_DIR").map(PathBuf::from) {
		include_file = include_file.join("slang.h");
		slang_dir
	} else if let Ok(vulkan_sdk_dir) = env::var("VULKAN_SDK").map(PathBuf::from) {
		include_file = include_file.join("slang/slang.h");
		vulkan_sdk_dir
	} else {
		panic!("Environment `SLANG_DIR` should be set to the directory of a Slang installation, or `VULKAN_SDK` should be set to the directory of the Vulkan SKD installation.");
	};

	let out_dir = env::var("OUT_DIR")
		.map(PathBuf::from)
		.expect("Couldn't determine output directory.");

	link_libraries(&slang_dir);

	bindgen::builder()
		.header(slang_dir.join(include_file).to_str().unwrap())
		.clang_arg("-v")
		.clang_arg("-xc++")
		.clang_arg("-std=c++17")
		.allowlist_function("spReflection.*")
		.allowlist_function("slang_.*")
		.allowlist_type("slang.*")
		.allowlist_var("SLANG_.*")
		.with_codegen_config(
			bindgen::CodegenConfig::FUNCTIONS
				| bindgen::CodegenConfig::TYPES
				| bindgen::CodegenConfig::VARS,
		)
		.parse_callbacks(Box::new(ParseCallback {}))
		.default_enum_style(bindgen::EnumVariation::Rust {
			non_exhaustive: false,
		})
		.constified_enum("SlangProfileID")
		.constified_enum("SlangCapabilityID")
		.vtable_generation(true)
		.layout_tests(false)
		.derive_copy(true)
		.generate()
		.expect("Couldn't generate bindings.")
		.write_to_file(out_dir.join("bindings.rs"))
		.expect("Couldn't write bindings.");
}

fn link_libraries(slang_dir: &Path) {
	let lib_dir = slang_dir.join("lib");

	if !lib_dir.is_dir() {
		panic!("Couldn't find the `lib` subdirectory in the Slang installation directory.")
	}

	println!("cargo:rustc-link-search=native={}", lib_dir.display());
	println!("cargo:rustc-link-lib=dylib=slang");
}

#[derive(Debug)]
struct ParseCallback {}

impl bindgen::callbacks::ParseCallbacks for ParseCallback {
	fn enum_variant_name(
		&self,
		enum_name: Option<&str>,
		original_variant_name: &str,
		_variant_value: bindgen::callbacks::EnumVariantValue,
	) -> Option<String> {
		let enum_name = enum_name?;

		// Map enum names to the part of their variant names that needs to be trimmed.
		// When an enum name is not in this map the code below will try to trim the enum name itself.
		let mut map = std::collections::HashMap::new();
		map.insert("SlangMatrixLayoutMode", "SlangMatrixLayout");
		map.insert("SlangCompileTarget", "Slang");

		let trim = map.get(enum_name).unwrap_or(&enum_name);
		let new_variant_name = pascal_case_from_snake_case(original_variant_name);
		let new_variant_name = new_variant_name.trim_start_matches(trim);
		Some(new_variant_name.to_string())
	}
}

/// Converts `snake_case` or `SNAKE_CASE` to `PascalCase`.
/// If the input is already in `PascalCase` it will be returned as is.
fn pascal_case_from_snake_case(snake_case: &str) -> String {
	let mut result = String::new();

	let should_lower = snake_case
		.chars()
		.filter(|c| c.is_alphabetic())
		.all(|c| c.is_uppercase());

	for part in snake_case.split('_') {
		for (i, c) in part.chars().enumerate() {
			if i == 0 {
				result.push(c.to_ascii_uppercase());
			} else if should_lower {
				result.push(c.to_ascii_lowercase());
			} else {
				result.push(c);
			}
		}
	}

	result
}
