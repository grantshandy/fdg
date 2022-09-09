all:
	wasm-pack build --target web --out-name fdg-wasm --out-dir examples/pkg

serve: all
	python3 -m http.server