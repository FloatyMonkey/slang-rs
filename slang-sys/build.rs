extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
	let slang_dir = env::var("SLANG_DIR")
		.map(PathBuf::from)
		.expect("Environment variable `SLANG_DIR` should be set to the directory of a Slang installation. \
		This directory should contain `slang.h` and a `bin` subdirectory.");

	let out_dir = env::var("OUT_DIR")
		.map(PathBuf::from)
		.expect("Couldn't determine output directory.");

	link_libraries(&slang_dir);

	bindgen::builder()
		.header(slang_dir.join("slang.h").to_str().unwrap())
		.clang_arg("-v")
		.clang_arg("-xc++")
		.clang_arg("-std=c++14")
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
			non_exhaustive: true,
		})
		.vtable_generation(true)
		.layout_tests(false)
		.derive_copy(true)
		.generate()
		.expect("Couldn't generate bindings.")
		.write_to_file(out_dir.join("bindings.rs"))
		.expect("Couldn't write bindings.");
}

fn link_libraries(slang_dir: &Path) {
	let target_os = env::var("CARGO_CFG_TARGET_OS")
		.expect("Couldn't determine target OS.");

	let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
		.expect("Couldn't determine target architecture.");

	let target = match(&*target_os, &*target_arch) {
		("windows", "x86")     => "windows-x86",
		("windows", "x86_64")  => "windows-x64",
		("windows", "aarch64") => "windows-aarch64",
		("linux",   "x86_64")  => "linux-x64",
		("linux",   "aarch64") => "linux-aarch64",
		("macos",   "x86_64")  => "macosx-x64",

		(os, arch) => panic!("Unsupported OS or architecture: {os} {arch}")
	};

	let bin_dir = slang_dir.join(format!("bin/{target}/release"));

	if !bin_dir.is_dir() {
		panic!("
			Could not find the target-specific `bin` subdirectory (bin/{target}/release) in the Slang installation directory. \
			The Slang installation may not match the target this crate is being compiled for.
		")
	}

	println!("cargo:rustc-link-search=native={}", bin_dir.display());
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
fn pascal_case_from_snake_case(snake_case: &str) -> String {
	let mut result = String::new();
	let mut capitalize_next = true;

	for c in snake_case.chars() {
		if c == '_' {
			capitalize_next = true;
		} else {
			if capitalize_next {
				result.push(c.to_ascii_uppercase());
				capitalize_next = false;
			} else {
				result.push(c.to_ascii_lowercase());
			}
		}
	}

	result
}
