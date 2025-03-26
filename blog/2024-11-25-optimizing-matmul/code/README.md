The Rust code that accompanies the blog post.

You will find:

1. A binary (`blog`) that you can run with `cargo run`. Use `RUST_LOG=blog=info cargo run --release` to see output while running.
2. Benchmarks that you can run with `cargo bench`
3. GPU shaders/kernels written in Rust
4. CPU code that takes the shaders and runs it on the GPU (via `wgpu`) or the CPU with a
   simulated harness
5. Some tests that you can run with `cargo test`

A good place to start to get the lay of the land is the workspace's `Cargo.toml` in this
directory.

**Any changes to these files should ensure that the blog post is still correct as it
uses line numbers to embed code snippets.**
