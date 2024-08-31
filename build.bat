
cargo build --release --target wasm32-wasip1 --manifest-path ./packages/compiler/c/Cargo.toml
wasm-tools component new .\target\wasm32-wasip1\release\snippet_compiler_c.wasm -o snippet_compiler_c.wasm --adapt .\packages\wasi_snapshot_preview1.wasm
cargo build --release --target wasm32-wasip1 --manifest-path ./packages/language/c/Cargo.toml
wasm-tools component new .\target\wasm32-wasip1\release\snippet_language_c.wasm -o snippet_language_c.wasm --adapt .\packages\wasi_snapshot_preview1.wasm