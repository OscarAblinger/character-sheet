#!/bin/sh

# compile engine
cd ../jsbinding || exit 1
wasm-pack build --target web || exit 2

# copy compiled target into this directory
cd .. || exit 1
# cp engine/target/wasm32-unknown-unknown/release/engine.wasm renderer/engine.wasm
mkdir -p renderer/engine || exit 3
cp jsbinding/pkg/character_sheet_js*.js renderer/engine/ || exit 4
cp jsbinding/pkg/character_sheet_js*.wasm renderer/engine/ || exit 5

