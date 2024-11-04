# Test project showing wasm-opt errors

Rust 1.82.0 generates wasm code that causes wasm-opt to fail with the message
`parse exception: invalid code after misc prefix: 17`

The same code works if compiled with Rust 1.81.0.

## Building with trunk:

```plaintext
$ trunk build --release

...

   Compiling wasm_test_2024 v0.1.0
    Finished `release` profile [optimized] target(s) in 11.78s
[parse exception: invalid code after misc prefix: 17 (at 0:20646)]
Fatal: error parsing wasm (try --debug for more info)
2024-11-04T23:06:47.941140Z ERROR ❌ error
error from build pipeline

Caused by:
    0: HTML build pipeline failed (1 errors), showing first
    1: error from asset pipeline
    2: running wasm-opt
    3: wasm-opt call to executable '/home/___/.cache/trunk/wasm-opt-version_116/bin/wasm-opt' with args: '["--output=.../wasm_test_2024_b/target/wasm-opt/release/wasm_test_2024_bg.wasm", "-Oz", ".../wasm_test_2024_b/dist/.stage/wasm_test_2024-616af1a08af380ae_bg.wasm"]' returned a bad status: exit status: 1
2024-11-04T23:06:47.941182Z ERROR error from build pipeline
2024-11-04T23:06:47.941184Z  INFO   1: HTML build pipeline failed (1 errors), showing first
2024-11-04T23:06:47.941186Z  INFO   2: error from asset pipeline
2024-11-04T23:06:47.941187Z  INFO   3: running wasm-opt
2024-11-04T23:06:47.941189Z  INFO   4: wasm-opt call to executable '/home/___/.cache/trunk/wasm-opt-version_116/bin/wasm-opt' with args: '["--output=.../wasm_test_2024_b/target/wasm-opt/release/wasm_test_2024_bg.wasm", "-Oz", ".../wasm_test_2024_b/dist/.stage/wasm_test_2024-616af1a08af380ae_bg.wasm"]' returned a bad status: exit status: 1
```

## Building with cargo + wasm-bindgen + wasm-opt:

```plaintext
$ cargo build --target=wasm32-unknown-unknown --release --bin wasm_test_2024
$ wasm-bindgen --target=web --out-dir=target/wasm-bindgen/release --out-name=wasm_test_2024 target/wasm32-unknown-unknown/release/wasm_test_2024.wasm --no-typescript
$ wasm-opt -Oz target/wasm-bindgen/release/wasm_test_2024_bg.wasm --output wasm-opt.out
[parse exception: invalid code after misc prefix: 17 (at 0:20646)]
Fatal: error parsing wasm (try --debug for more info)
```
