import React from "react";
import CodeBlock from "@theme/CodeBlock";
import Snippet from "@site/src/components/Snippet";
import RustKernelSource from "!!raw-loader!../code/crates/gpu/naive/src/lib.rs";
import RustWorkgroupCount from "!!raw-loader!../code/crates/cpu/matmul/src/variants.rs";
import RustWgpuBackend from "!!raw-loader!../code/crates/cpu/matmul/src/backends/wgpu.rs";

export const WebGpuInputs: React.FC = () => (
  <CodeBlock language="wgsl" title="WGSL" className="text-xs">
    {`struct Dimensions {
  M: u32,
  K: u32,
  N: u32,
}

@group(0) @binding(0) var<uniform> dimensions: Dimensions;
@group(0) @binding(1) var<storage, read> a: array<f32>;
@group(0) @binding(2) var<storage, read> b: array<f32>;
@group(0) @binding(3) var<storage, read_write> result: array<f32>;
`}
  </CodeBlock>
);

export const WebGpuKernel: React.FC = () => (
  <CodeBlock language="wgsl" title="WGSL" className="text-xs">
    {" "}
    {`@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let index = global_id.x;
  let row = index / dimensions.N;
  let col = index % dimensions.N;

  if (index < dimensions.M * dimensions.N) {
    var sum = 0.0;
    for (var i: u32 = 0u; i < dimensions.K; i = i + 1u) {
      sum = sum + a[row * dimensions.K + i] * b[i * dimensions.N + col];
    }
    result[row * dimensions.N + col] = sum;
  }
}
`}
  </CodeBlock>
);

export const RustNaiveKernel: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    showLineNumbers
    title="Naive kernel with Rust GPU"
  >
    {RustKernelSource}
  </Snippet>
);

export const RustNaiveWorkgroupCount: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    lines="26-34"
    hash="8abb43d"
    title="Calculating on the CPU how many workgroup dispatches are needed"
  >
    {RustWorkgroupCount}
  </Snippet>
);

export const RustNaiveDispatch: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    lines="152,154"
    hash="cbb5295"
    strip_leading_spaces
    title="Using wgpu on the CPU to dispatch workgroups to the GPU"
  >
    {RustWgpuBackend}
  </Snippet>
);

export const RustNaiveWorkgroup: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="7" hash="7762339">
    {RustKernelSource}
  </Snippet>
);
