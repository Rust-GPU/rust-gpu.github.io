import React from "react";
import CodeBlock from "@theme/CodeBlock";
import Snippet from "@site/src/components/Snippet";
import RustKernelSource from "!!raw-loader!../code/crates/gpu/tiling_1d/src/lib.rs";

export const RustTiling1d: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    title="Tiling kernel with Rust GPU"
  >
    {RustKernelSource}
  </Snippet>
);
