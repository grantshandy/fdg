#!/bin/sh
rustup target add wasm32-unknown-unknown

cargo build --release --target wasm32-unknown-unknown --examples

cp target/wasm32-unknown-unknown/release/examples/view_nd_cube.wasm ./fdg-macroquad/examples/web-examples/
cp target/wasm32-unknown-unknown/release/examples/view_binary_tree.wasm ./fdg-macroquad/examples/web-examples/
cp target/wasm32-unknown-unknown/release/examples/view_cylinder.wasm ./fdg-macroquad/examples/web-examples/
cp target/wasm32-unknown-unknown/release/examples/view_json.wasm ./fdg-macroquad/examples/web-examples/
cp target/wasm32-unknown-unknown/release/examples/view_lattice.wasm ./fdg-macroquad/examples/web-examples/
cp target/wasm32-unknown-unknown/release/examples/view_ring.wasm ./fdg-macroquad/examples/web-examples/
cp target/wasm32-unknown-unknown/release/examples/view_sphere.wasm ./fdg-macroquad/examples/web-examples/
