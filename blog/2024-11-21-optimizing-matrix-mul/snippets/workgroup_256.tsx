import React from "react";
import Snippet from "@site/src/components/Snippet";
import RustKernelSource from "!!raw-loader!../code/crates/gpu/workgroup_256/src/lib.rs";
import VariantsSource from "!!raw-loader!../code/crates/cpu/matmul/src/variants.rs";

export const RustWorkgroup256Workgroup: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="7" hash="56b3ae8">
    {RustKernelSource}
  </Snippet>
);

export const RustWorkgroup256WorkgroupCount: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    lines="51-65"
    hash="8abb43d"
    title="Calculating how many workgroup dispatches are needed on the CPU"
  >
    {VariantsSource}
  </Snippet>
);
