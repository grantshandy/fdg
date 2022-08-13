all:
	wasm-pack build --target web --out-name fdg-web --out-dir examples

serve: all
	python3 -m http.server -d examples