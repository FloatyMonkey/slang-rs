# Slang Rust Bindings

Rust bindings for the [Slang](https://github.com/shader-slang/slang/) shader language compiler. In contrast to existing bindings, these internally use Slang's COM/C++ API because the old C API is soon to be deprecated.

Currently mostly reflects the needs of our own [engine](https://github.com/FloatyMonkey/engine) but issues and pull requests are more than welcome.

## Example

```rust
let global_session = slang::GlobalSession::new().unwrap();

let search_path = std::ffi::CString::new("shaders/directory").unwrap();

// All compiler options are available through this builder.
let session_options = slang::OptionsBuilder::new()
	.optimization(slang::OptimizationLevel::High)
	.matrix_layout_row(true);

let target_desc = slang::TargetDescBuilder::new()
	.format(slang::CompileTarget::Dxil)
	.profile(self.global_session.find_profile("sm_6_5"));

let session_desc = slang::SessionDescBuilder::new()
	.targets(&[*target_desc])
	.search_paths(&[include_path.as_ptr()])
	.options(&session_options);

let session = self.global_session.create_session(&session_desc).unwrap();

let module = session.load_module("filename.slang").unwrap();

let entry_point = module.find_entry_point_by_name("main").unwrap();

let program = session.create_composite_component_type(&[
	module.downcast(), entry_point.downcast(),
]);

let linked_program = program.link();

let shader_bytecode = linked_program.get_entry_point_code(0, 0);
```

## Installation

Add the following to the `[dependencies]` section of your `Cargo.toml`:

```toml
slang = { git = "https://github.com/FloatyMonkey/slang-rs.git" }
```

Set the `SLANG_DIR` environment variable to the path of your Slang installation. Download the latest release from their [releases page](https://github.com/shader-slang/slang/releases). Copy `slang.dll` to your executable's directory.

To compile to DXIL bytecode you need the Microsoft DirectXShaderCompiler. Download the latest release from their [releases page](https://github.com/microsoft/DirectXShaderCompiler/releases). Copy `dxil.dll` and `dxcompiler.dll` to your executable's directory.

## Credits

Maintained by Lauro Oyen ([@laurooyen](https://github.com/laurooyen)).

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
