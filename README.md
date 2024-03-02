# Slang Rust Bindings

Rust bindings for the [Slang](https://github.com/shader-slang/slang/) shader language compiler. In contrast to existing bindings, these internally use Slang's COM/C++ API because the old C API is soon to be deprecated.

Currently mostly reflects the needs of our own [engine](https://github.com/FloatyMonkey/engine) but issues and pull requests are more than welcome.

## Example

```rust
let session = slang::GlobalSession::new();

let mut compile_request = session.create_compile_request();

compile_request
	.set_codegen_target(slang::CompileTarget::Dxil)
	.set_target_profile(session.find_profile("sm_6_5"));

let entry_point = compile_request
	.add_translation_unit(slang::SourceLanguage::Slang, None)
	.add_source_file(filepath)
	.add_entry_point("main", slang::Stage::Compute);

let shader_bytecode = compile_request
	.compile()
	.expect("Shader compilation failed.")
	.get_entry_point_code(entry_point);
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
