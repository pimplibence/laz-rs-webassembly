build:
	wasm-pack build --no-pack --no-typescript --release --out-name laz_rs_webassembly-lib --target web
	cp ./js/* ./pkg