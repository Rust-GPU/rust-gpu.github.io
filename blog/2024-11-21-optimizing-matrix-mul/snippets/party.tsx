import React from "react";
import Snippet from "@site/src/components/Snippet";
import RustKernelSource from "!!raw-loader!../code/crates/gpu/tiling_1d/src/lib.rs";
import RustIsomorphicSource from "!!raw-loader!../code/crates/shared/isomorphic/src/lib.rs";
import RustIsomorphicCargoToml from "!!raw-loader!../code/crates/shared/isomorphic/Cargo.toml";
import RustWgpuBackend from "!!raw-loader!../code/crates/cpu/matmul/src/backends/wgpu.rs";
import RustCpuBackendSource from "!!raw-loader!../code/crates/cpu/matmul/src/backends/cpu.rs";

export const RustPartySettings: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="3,9,11">
    {RustKernelSource}
  </Snippet>
);

export const RustIsomorphic: React.FC = () => (
  <Snippet language="rust" className="text-xs">
    {RustIsomorphicSource}
  </Snippet>
);

export const RustIsomorphicGlam: React.FC = () => (
  <Snippet language="rust" lines="15-19" className="text-xs">
    {RustIsomorphicSource}
  </Snippet>
);

export const RustIsomorphicDeps: React.FC = () => (
  <Snippet language="rust" lines="9-20" className="text-xs" title="Cargo.toml">
    {RustIsomorphicCargoToml}
  </Snippet>
);

export const RustWgpuDimensions: React.FC = () => (
  <Snippet
    language="rust"
    lines="98-111"
    className="text-xs"
    title="Creating the Dimensions struct on the CPU and writing it to the GPU"
  >
    {RustWgpuBackend}
  </Snippet>
);

export const RustCpuBackendHarness: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="30-72">
    {RustCpuBackendSource}
  </Snippet>
);

export const RustCpuBackendTest: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="155-172">
    {RustCpuBackendSource}
  </Snippet>
);
