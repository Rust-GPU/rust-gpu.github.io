import React from "react";
import CodeBlock from "@theme/CodeBlock";
import Snippet from "@site/src/components/Snippet";
import RustKernelSource from "!!raw-loader!../code/crates/gpu/workgroup_2d/src/lib.rs";
import VariantsSource from "!!raw-loader!../code/crates/cpu/matmul/src/variants.rs";
import WgpuBackendSource from "!!raw-loader!../code/crates/cpu/matmul/src/backends/wgpu.rs";

export const RustWorkgroup2d: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    title="2D workgroup kernel with Rust GPU"
  >
    {RustKernelSource}
  </Snippet>
);

/*
export const RustWorkgroup2d: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    lines="7-8,15-16"
    title="2D workgroup kernel with Rust GPU"
  >
    {RustKernelSource}
  </Snippet>
);
*/

export const RustWorkgroup2dWorkgroup: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="7">
    {RustKernelSource}
  </Snippet>
);

export const RustWorkgroup2dWorkgroupCount: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    lines="82-94"
    title="Calculating how many workgroup dispatches are needed on the CPU"
  >
    {VariantsSource}
  </Snippet>
);

export const RustWorkgroup2dWgpuDispatch: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    lines="144,145,147"
    strip_leading_spaces
    title="Using wgpu on the CPU to dispatch to the GPU"
  >
    {WgpuBackendSource}
  </Snippet>
);
