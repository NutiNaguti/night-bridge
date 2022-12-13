#/bin/bash

rustup target add wasm32-unknown-unknown
cargo build -p bridge --target wasm32-unknown-unknown --release
cargo build -p ft --target wasm32-unknown-unknown --release
cargo build -p ft_erc20 --target wasm32-unknown-unknown --release
cargo build -p lite-node --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/lite_node.wasm ./out/lite_node.wasm 
cp ./target/wasm32-unknown-unknown/release/bridge.wasm ./out/bridge.wasm
cp ./target/wasm32-unknown-unknown/release/ft.wasm ./out/ft.wasm
cp ./target/wasm32-unknown-unknown/release/ft_erc20.wasm ./out/ft_erc20.wasm
