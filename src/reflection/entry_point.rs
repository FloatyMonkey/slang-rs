use super::{Function, TypeLayout, VariableLayout, rcall};
use crate::Stage;
use slang_sys as sys;

#[repr(transparent)]
pub struct EntryPoint(sys::SlangReflectionEntryPoint);

impl EntryPoint {
	pub fn name(&self) -> &str {
		let name = rcall!(spReflectionEntryPoint_getName(self));
		unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() }
	}

	pub fn name_override(&self) -> Option<&str> {
		let name = rcall!(spReflectionEntryPoint_getNameOverride(self));
		(!name.is_null()).then(|| unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() })
	}

	pub fn parameter_count(&self) -> u32 {
		rcall!(spReflectionEntryPoint_getParameterCount(self))
	}

	pub fn parameter_by_index(&self, index: u32) -> Option<&VariableLayout> {
		rcall!(spReflectionEntryPoint_getParameterByIndex(self, index) as Option<&VariableLayout>)
	}

	pub fn parameters(&self) -> impl ExactSizeIterator<Item = &VariableLayout> {
		(0..self.parameter_count()).map(move |i| {
			rcall!(spReflectionEntryPoint_getParameterByIndex(self, i) as &VariableLayout)
		})
	}

	pub fn function(&self) -> &Function {
		rcall!(spReflectionEntryPoint_getFunction(self) as &Function)
	}

	pub fn stage(&self) -> Stage {
		rcall!(spReflectionEntryPoint_getStage(self))
	}

	pub fn compute_thread_group_size(&self) -> [u64; 3] {
		let mut out_size = [0; 3];
		rcall!(spReflectionEntryPoint_getComputeThreadGroupSize(
			self,
			3,
			&mut out_size as *mut u64
		));
		out_size
	}

	pub fn compute_wave_size(&self) -> u64 {
		let mut out_size = 0;
		rcall!(spReflectionEntryPoint_getComputeWaveSize(
			self,
			&mut out_size as *mut u64
		));
		out_size
	}

	pub fn uses_any_sample_rate_input(&self) -> bool {
		rcall!(spReflectionEntryPoint_usesAnySampleRateInput(self)) != 0
	}

	pub fn var_layout(&self) -> &VariableLayout {
		rcall!(spReflectionEntryPoint_getVarLayout(self) as &VariableLayout)
	}

	pub fn type_layout(&self) -> &TypeLayout {
		self.var_layout().type_layout()
	}

	pub fn result_var_layout(&self) -> &VariableLayout {
		rcall!(spReflectionEntryPoint_getResultVarLayout(self) as &VariableLayout)
	}

	pub fn has_default_constant_buffer(&self) -> bool {
		rcall!(spReflectionEntryPoint_hasDefaultConstantBuffer(self)) != 0
	}
}
