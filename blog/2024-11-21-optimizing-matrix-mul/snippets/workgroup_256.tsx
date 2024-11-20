import React from "react";
import Snippet from "@site/src/components/Snippet";
import RustKernelSource from "!!raw-loader!../code/crates/gpu/workgroup_256/src/lib.rs";
import VariantsSource from "!!raw-loader!../code/crates/cpu/matmul/src/variants.rs";
import WgpuBackendSource from "!!raw-loader!../code/crates/cpu/matmul/src/backends/wgpu.rs";

export const RustWorkgroup256Workgroup: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="7">
    {RustKernelSource}
  </Snippet>
);

export const RustWorkgroup256WorkgroupCount: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    lines="51-64"
    title="Calculating how many workgroup dispatches are needed on the CPU"
  >
    {VariantsSource}
  </Snippet>
);

export const RustWorkgroup256WgpuDispatch: React.FC = () => (
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
