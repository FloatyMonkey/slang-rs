StructuredBuffer<float> input_0;
StructuredBuffer<float> input_1;
RWStructuredBuffer<float> output;

[shader("compute")]
[numthreads(1, 1, 1)]
void main(uint3 thread_id : SV_DispatchThreadID) {
	let index = thread_id.x;
	output[index] = input_0[index] * input_1[index];
}
