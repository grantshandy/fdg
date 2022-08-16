all:
	wasm-pack build --target web --out-name fdg-wasm --out-dir pkg

serve: all
	python3 -m http.server