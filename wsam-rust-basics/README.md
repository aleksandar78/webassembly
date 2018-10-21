# WebAssembly samples with Rust

## How to

### Installation
To install Rust and other auxiliary packages [rustup](https://sh.rustup.rs) is recommended tool.

For Mac OSX rustup can be installed with brew:
```shell
brew install rustup
```

To get all necessary staff:
```shell
rustup-init
```

To set rust version for WebAssembly nightly version is requires:
```shell
rustup default nightly
```

To set rust target for WebAssembly (no platform for WebAssembly - that's why is unknown-unknown);
```shell
rustup target wasm32-unknown-unknown
```

Rust package manager is Cargo. It has been install with rustup tool.
Next step is install of webassembly pack:
```shell
cargo install wasm-pack
```

Another useful tool is wasm-gc. This tool removes all unused imports, functions and other stuff
from WebAssembly modules.
```shell
cargo install wasm-gc
```

To be able to develop web server is required.
```shell
cargo install https
```

### Development

Create util lib
```shell
cargo new --lib utils
```

Build WebAssembly module
```shell
wasm-pack build
```



