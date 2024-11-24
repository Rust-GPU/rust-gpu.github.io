import React from "react";
import CodeBlock from "@theme/CodeBlock";
import Snippet from "@site/src/components/Snippet";
import RustKernelSource from "!!raw-loader!../code/crates/gpu/tiling_1d_loop/src/lib.rs";

export const RustTiling1dLoop: React.FC = () => (
  <Snippet
    language="rust"
    className="text-xs"
    title="Tiling kernel using loops with Rust GPU"
  >
    {RustKernelSource}
  </Snippet>
);
