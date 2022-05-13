all:
	rustup target add wasm32-unknown-unknown
	cd ..
	cargo build --release --target wasm32-unknown-unknown --examples
	cp ../target/wasm32-unknown-unknown/release/examples/4d_cube.wasm .
	cp ../target/wasm32-unknown-unknown/release/examples/binary_tree.wasm .
	cp ../target/wasm32-unknown-unknown/release/examples/cylinder.wasm .
	cp ../target/wasm32-unknown-unknown/release/examples/json.wasm .
	cp ../target/wasm32-unknown-unknown/release/examples/lattice.wasm .
	cp ../target/wasm32-unknown-unknown/release/examples/ring.wasm .
	cp ../target/wasm32-unknown-unknown/release/examples/sphere.wasm .
	cp ../target/wasm32-unknown-unknown/release/examples/empty.wasm .

