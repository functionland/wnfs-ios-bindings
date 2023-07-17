all: clean test gen-c-header add-rust-targets x86_64-apple-ios\
 aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-darwin\
  x86_64-apple-darwin lipo-ios lipo-darwin xcode-build bundle

test:
	cargo test
gen-c-header: 
	cargo install cbindgen && cbindgen --lang C -o include/wnfsbindings.h .

add-rust-targets:
	rustup target add x86_64-apple-ios aarch64-apple-darwin aarch64-apple-ios-sim aarch64-apple-ios x86_64-apple-darwin

aarch64-apple-ios:
	cargo build --release --target aarch64-apple-ios

aarch64-apple-darwin:
	cargo build --release --target aarch64-apple-darwin

aarch64-apple-ios-sim:
	cargo build --release --target aarch64-apple-ios-sim

x86_64-apple-ios:
	cargo build --release --target x86_64-apple-ios

x86_64-apple-darwin:
	cargo build --release --target x86_64-apple-darwin

lipo-ios:
	mkdir -p build && \
	lipo -create \
	target/x86_64-apple-ios/release/libwnfsbindings.a \
	target/aarch64-apple-ios-sim/release/libwnfsbindings.a \
	-output build/libwnfsbindings_iossimulator.a

lipo-darwin:
	mkdir -p build && \
	lipo -create \
	target/x86_64-apple-darwin/release/libwnfsbindings.a \
	target/aarch64-apple-darwin/release/libwnfsbindings.a \
	-output build/libwnfsbindings_darwin.a

xcode-build:
	xcodebuild -create-xcframework \
	-library ./build/libwnfsbindings_iossimulator.a \
	-headers ./include/ \
	-library ./target/aarch64-apple-ios/release/libwnfsbindings.a \
	-headers ./include/ \
	-library ./build/libwnfsbindings_darwin.a \
	-headers ./include/ \
	-output ./build/WnfsBindings.xcframework

bundle:
	cd build && cp  ../LICENSE . &&\
	zip -r ./cocoapods-bundle.zip ./WnfsBindings.xcframework ./LICENSE && echo "$$(openssl dgst -sha256 ./cocoapods-bundle.zip)" > ./cocoapods-bundle.zip.sha256

clean:
	rm -rf build/*
