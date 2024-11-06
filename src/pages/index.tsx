import React, { useEffect, useState, useRef } from "react";
import clsx from "clsx";
import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import HomeLayout from "@site/src/components/HomeLayout";
import Heading from "@theme/Heading";
import Navbar from "@theme/Navbar";
import CodeBlock from "@theme/CodeBlock";
import {
  FcMultipleDevices,
  FcLike,
  FcPuzzle,
  FcParallelTasks,
  FcInTransit,
} from "react-icons/fc";

interface HeroCodeProps {
  className?: string;
}

const HeroCode: React.FC<HeroCodeProps> = ({ className = "" }) => {
  const rustCode = `\
use glam::UVec3;
use spirv_std::spirv;

enum Outcome {
  Fizz,
  Buzz,
  FizzBuzz,
}

trait Game {
    fn fizzbuzz(&self) -> Option<Outcome>;
}

impl Game for u32 {
    fn fizzbuzz(&self) -> Option<Outcome> {
        match (self % 3 == 0, self % 5 == 0) {
            (true, true) => Some(Outcome::FizzBuzz),
            (true, false) => Some(Outcome::Fizz),
            (false, true) => Some(Outcome::Buzz),
            _ => None,
        }
    }
}

#[spirv(compute(threads(64)))]
pub fn main(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer)] output: &mut [Option<Outcome>; 64],
) {
    let index = id.x as u32;
    output[index] = index.fizzbuzz();
}\
`;
  return (
    <CodeBlock
      language="rust"
      className={clsx(
        "max-w-[50em] lg:max-w-[80em] overflow-hidden leading-tight w-full text-left",
        "text-[8px] lg:text-[11px]",
        "font-mono",
        className
      )}
    >
      {rustCode}
    </CodeBlock>
  );
};

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className="flex flex-col w-full max-w-[1200px] justify-center mx-auto">
      <div className="flex flex-col flex-grow items-center justify-center w-full">
        {/* Row 1 */}
        <div className="flex flex-col lg:flex-row w-full gap-8 items-center">
          {/* Left Column */}
          <div className="flex flex-col items-center justify-center text-center w-full lg:w-1/2">
            <h1 className="heading-text text-5xl font-bold font-text my-0">
              {siteConfig.title}
            </h1>
            <h3 className="title-text mx-4 lg:mx-0 text-3xl mt-[1em] font-text font-medium">
              {siteConfig.tagline}
            </h3>
          </div>

          {/* Right Column */}
          <div className="flex items-center justify-center w-full lg:w-1/2 h-full mx-2 lg:ml-0 lg:mr-8 overflow-hidden">
            <HeroCode />
          </div>
        </div>
      </div>

      {/* Row 2 (Learn More at the Bottom) */}
      <div className="flex justify-center w-full mt-auto mb-4">
        <h4 className="text-md font-text font-normal cursor-pointer">
          <a
            id="learn-more"
            onClick={() =>
              document
                .getElementById("features")
                ?.scrollIntoView({ behavior: "smooth" })
            }
          >
            Learn More ⇣
          </a>
        </h4>
      </div>
    </header>
  );
}

interface FeatureSectionProps {
  className?: string;
  children: React.ReactNode;
}

const FeatureSection: React.FC<FeatureSectionProps> = ({
  className = "",
  children,
}) => {
  return (
    <div className={clsx("ml-8 pl-4 border-l-2 border-[#e5e7eb]", className)}>
      {children}
    </div>
  );
};

function Features() {
  const { siteConfig } = useDocusaurusContext();

  return (
    <div
      id="features"
      className={`flex flex-col justify-center pb-12 pt-12 px-[2em] max-w-[1200px] w-full`}
    >
      <div className="w-full text-4xl mb-8 font-text text-center">
        Finally, a modern language for GPUs
      </div>

      <div className="w-full justify-center">
        <p>
          Rust GPU makes it possible to write and run GPU software in Rust,
          leveraging the language's powerful safety and concurrency features to
          enhance performance and reliability. With Rust GPU, you can seamlessly
          develop for both CPU and GPU using a unified codebase while benefiting
          from Rust's existing ecosystem.
        </p>
        <h2 id="multi-vendor">
          <FcMultipleDevices className="inline-block  mb-1 mr-2" /> Multi-Vendor
        </h2>
        <FeatureSection>
          <p>
            The Rust GPU compiler backend emits code compatible with{" "}
            <a href="https://www.vulkan.org">Vulkan</a>, ensuring your code runs
            across a{" "}
            <a href="https://vulkan.gpuinfo.org/">
              wide range of devices and vendors
            </a>
            .
          </p>
          <p>
            If instead you wish to stick to the NVIDIA-only ecosystem, stay
            tuned as the{" "}
            <a href="https://github.com/rust-gpu/rust-cuda">Rust CUDA</a>{" "}
            project is in the process of being rebooted and possibly integrated
            with Rust GPU.
          </p>
        </FeatureSection>
        <h2 id="modern-and-unified">
          <FcLike className="inline-block mb-1 mr-2" />
          Modern and Unified
        </h2>
        <FeatureSection>
          <p>
            There is no longer a need to learn a GPU-specific programming
            language. You can write both CPU and GPU code in Rust, leveraging
            your existing Rust knowledge and maintaining a consistent
            development experience. Furthermore, the{" "}
            <em>same code can run on both the CPU and GPU</em>, with divergent
            behavior gated behind <code>cfg</code> attributes and macros where
            necessary.
          </p>
          <p>
            Even if your current codebase isn't written in Rust, choosing Rust
            for the GPU parts gives you more widely applicable skills in one of
            the{" "}
            <a href="https://octoverse.github.com/2022/top-programming-languages">
              fastest-growing languages on GitHub
            </a>
            . Rust is also one of the{" "}
            <a href="https://github.blog/developer-skills/programming-languages-and-frameworks/why-rust-is-the-most-admired-language-among-developers/">
              most admired programming languages
            </a>{" "}
            while GPU-specific languages are considered a necessary evil.
          </p>
        </FeatureSection>
        <h2 id="fearless-concurrency">
          <FcParallelTasks className="inline-block mb-1 mr-2" />
          Fearless Concurrency
        </h2>
        <FeatureSection>
          <p>
            Rust's ownership model and type system guarantee memory safety,
            minimizing bugs and undefined behavior. Rust's borrow checker also
            enables fearless concurrency, which is essential for maximizing
            performance on massively parallel GPUs.
          </p>
        </FeatureSection>

        <h2 id="powerful-abstractions">
          <FcPuzzle className="inline-block mb-1 mr-2" />
          Powerful Abstractions
        </h2>
        <FeatureSection>
          <p>
            Programming in GPU-specific languages can often feel like{" "}
            <a href="https://xol.io/blah/death-to-shading-languages/">
              taking a step back to the 90s
            </a>
            , where primitive tools and sharp edges abound. Because of this,
            code written for GPUs is simplistic with low cyclomatic complexity.
            Rust has an expressive type system and zero-cost abstractions that
            enable writing high-level, reusable code without sacrificing
            performance. This approach leads to more maintainable and
            sophisticated GPU programs, streamlining the development process and
            enhancing productivity.
          </p>
        </FeatureSection>

        <h2 id="existing-ecosystem">
          <FcInTransit className="inline-block mb-1 mr-2" />
          Existing Ecosystem
        </h2>
        <FeatureSection>
          <p>
            The state-of-the-art for sharing GPU code is copy and pasting. With
            Rust GPU we are excited to bring the excellent <code>cargo</code>{" "}
            and crates.io ecosystem to GPU programming and provide some sanity.
          </p>
          <p>
            Rust's <code>no_std</code> ecosystem offers a wide array of
            libraries that can be used in environments without the standard
            library. Traditionally this has meant embedded devices, but a lot of
            the same assumptions apply to GPUs! As a consequence, you can reuse{" "}
            <a href="https://crates.io/categories/no-std::no-alloc">
              existing <code>no_std</code> libraries from crates.io
            </a>{" "}
            in your GPU code{" "}
            <em>without the authors explicitly adding GPU support</em>. This is
            uniquely enabled by Rust GPU's implementation choices and Rust's{" "}
            <a href="https://without.boats/blog/the-registers-of-rust/">
              registers
            </a>
            . Sharing and reusing code from the greater Rust ecosystem is a
            superpower when writing GPU programs that will massively compound
            over time.
          </p>
        </FeatureSection>
      </div>
    </div>
  );
}

export default function Home(): JSX.Element {
  const { siteConfig } = useDocusaurusContext();
  return (
    <HomeLayout description={siteConfig.tagline} hero={<HomepageHeader />}>
      <Features />
    </HomeLayout>
  );
}
