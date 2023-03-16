cargo build --release
rm ./wasm-out -rf
wasm-bindgen --out-dir ./wasm-out/ --target web ./target/wasm32-unknown-unknown/release/shifting_chamber.wasm
cp ./assets ./wasm-out -r
cp ./wasm-assets/index.html ./wasm-out
cd wasm-out && zip -r build.zip *