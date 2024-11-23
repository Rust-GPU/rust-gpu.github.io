import React from "react";
import Snippet from "@site/src/components/Snippet";
import RustKernelSource from "!!raw-loader!../code/crates/gpu/tiling_1d/src/lib.rs";
import RustIsomorphicSource from "!!raw-loader!../code/crates/shared/isomorphic/src/lib.rs";
import RustIsomorphicCargoToml from "!!raw-loader!../code/crates/shared/isomorphic/Cargo.toml";
import RustWgpuBackend from "!!raw-loader!../code/crates/cpu/matmul/src/backends/wgpu.rs";
import RustCpuBackendSource from "!!raw-loader!../code/crates/cpu/matmul/src/backends/cpu.rs";

export const RustPartySettings: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="3,9,11" hash="47bb656">
    {RustKernelSource}
  </Snippet>
);

export const RustIsomorphic: React.FC = () => (
  <Snippet language="rust" className="text-xs">
    {RustIsomorphicSource}
  </Snippet>
);

export const RustIsomorphicGlam: React.FC = () => (
  <Snippet language="rust" lines="15-19" hash="a3dbf2f" className="text-xs">
    {RustIsomorphicSource}
  </Snippet>
);

export const RustIsomorphicDeps: React.FC = () => (
  <Snippet
    language="rust"
    lines="9-20"
    hash="72c14d7"
    className="text-xs"
    title="Cargo.toml"
  >
    {RustIsomorphicCargoToml}
  </Snippet>
);

export const RustWgpuDimensions: React.FC = () => (
  <Snippet
    language="rust"
    lines="108-118"
    hash="cbb5295"
    className="text-xs"
    title="Creating the Dimensions struct on the CPU and writing it to the GPU"
  >
    {RustWgpuBackend}
  </Snippet>
);

export const RustCpuBackendHarness: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="30-79" hash="7ad7cab">
    {RustCpuBackendSource}
  </Snippet>
);

export const RustCpuBackendTest: React.FC = () => (
  <Snippet language="rust" className="text-xs" lines="174-194" hash="7ad7cab">
    {RustCpuBackendSource}
  </Snippet>
);
