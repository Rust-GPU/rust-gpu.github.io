import React from "react";
import Snippet from "@site/src/components/Snippet";
import RustKernelSource from "!!raw-loader!../code/crates/gpu/tiling_2d/src/lib.rs";

export const RustTiling2dSimd: React.FC = () => (
    <Snippet language="rust" className="text-xs" title="2D tiling kernel with Rust GPU">
      {RustKernelSource}
    </Snippet>
  );
