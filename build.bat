cargo build --release --target wasm32-wasip1 --manifest-path ./packages/compiler/gcc/Cargo.toml
wasm-tools component new ./target/wasm32-wasip1/release/snippet_compiler_gcc.wasm -o snippet-compiler-gcc.wasm --adapt ./packages/wasi_snapshot_preview1.wasm
cargo build --release --target wasm32-wasip1 --manifest-path ./packages/language/c/Cargo.toml
wasm-tools component new ./target/wasm32-wasip1/release/snippet_language_c.wasm -o snippet-language-c.wasm --adapt ./packages/wasi_snapshot_preview1.wasm
cargo build --release --target wasm32-wasip1 --manifest-path ./packages/compiler/cl/Cargo.toml
wasm-tools component new ./target/wasm32-wasip1/release/snippet_compiler_cl.wasm -o snippet-compiler-cl.wasm --adapt ./packages/wasi_snapshot_preview1.wasm
cargo build --release --target wasm32-wasip1 --manifest-path ./packages/compiler/clang/Cargo.toml
wasm-tools component new ./target/wasm32-wasip1/release/snippet_compiler_clang.wasm -o snippet-compiler-clang.wasm --adapt ./packages/wasi_snapshot_preview1.wasm