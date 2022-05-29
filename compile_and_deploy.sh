#!bin/bash

cargo build /home/russellbest/Final/marketplace --target wasm32-unknown-unknown --release
near deploy --wasmFile /home/russellbest/Final/marketplace/target/wasm32-unknown-unknown/release/marketplace.wasm --accountId=marketplace.awry.testnet