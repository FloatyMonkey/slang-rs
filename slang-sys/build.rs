extern crate bindgen;

use std::env;

fn main() {
	println!("cargo:rerun-if-env-changed=SLANG_DIR");
	println!("cargo:rerun-if-env-changed=SLANG_INCLUDE_DIR");
	println!("cargo:rerun-if-env-changed=SLANG_LIB_DIR");
	println!("cargo:rerun-if-env-changed=VULKAN_SDK");

	let include_dir = if let Ok(dir) = env::var("SLANG_INCLUDE_DIR") {
		dir
	} else if let Ok(dir) = env::var("SLANG_DIR") {
		format!("{dir}/include")
	} else if let Ok(dir) = env::var("VULKAN_SDK") {
		format!("{dir}/include/slang")
	} else {
		panic!("The environment variable SLANG_INCLUDE_DIR, SLANG_DIR, or VULKAN_SDK must be set");
	};

	let lib_dir = if let Ok(dir) = env::var("SLANG_LIB_DIR") {
		dir
	} else if let Ok(dir) = env::var("SLANG_DIR") {
		format!("{dir}/lib")
	} else if let Ok(dir) = env::var("VULKAN_SDK") {
		format!("{dir}/lib")
	} else {
		panic!("The environment variable SLANG_LIB_DIR, SLANG_DIR, or VULKAN_SDK must be set");
	};

	if !lib_dir.is_empty() {
		println!("cargo:rustc-link-search=native={lib_dir}");
	}

	println!("cargo:rustc-link-lib=dylib=slang");

	let out_dir = env::var("OUT_DIR").expect("Couldn't determine output directory.");

	bindgen::builder()
		.header(format!("{include_dir}/slang.h").as_str())
		.clang_arg("-v")
		.clang_arg("-xc++")
		.clang_arg("-std=c++17")
		.allowlist_function("spReflection.*")
		.allowlist_function("spComputeStringHash")
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
		.write_to_file(format!("{out_dir}/bindings.rs").as_str())
		.expect("Couldn't write bindings.");
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

	#[cfg(feature = "serde")]
	fn add_derives(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
		if info.name.starts_with("Slang") && info.kind == bindgen::callbacks::TypeKind::Enum {
			return vec!["serde::Serialize".into(), "serde::Deserialize".into()];
		}
		vec![]
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
