all: test gen-c-header add-rust-targets x86_64-apple-ios x86_64-apple-darwin aarch64-apple-darwin\
 aarch64-apple-ios aarch64-apple-ios-sim lipo-ios xcode-build bundle

test:
	cargo test
gen-c-header: 
	cargo install cbindgen && cbindgen --lang C -o include/wnfs.h .

add-rust-targets:
	rustup target add x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-darwin aarch64-apple-darwin

aarch64-apple-ios:
	cargo build --release --target aarch64-apple-ios

x86_64-apple-ios:
	cargo build --release --target x86_64-apple-ios

aarch64-apple-ios-sim:
	cargo build --release --target aarch64-apple-ios-sim

x86_64-apple-darwin:
	cargo build --release --target x86_64-apple-darwin

aarch64-apple-darwin:
	cargo build --release --target aarch64-apple-darwin

lipo-ios:
	lipo -create \
	target/x86_64-apple-ios/release/libwnfsbindings.a \
	target/aarch64-apple-ios-sim/release/libwnfsbindings.a \
	-output build/libwnfsbindings_iossimulator.a

xcode-build:
	xcodebuild -create-xcframework \
	-library ./build/libwnfsbindings_iossimulator.a \
	-headers ./include/ \
	-library ./target/aarch64-apple-ios/release/libwnfsbindings.a \
	-headers ./include/ \
	-library ./target/x86_64-apple-darwin/release/libwnfsbindings.a \
	-headers ./include/ \
	-output ./build/WnfsBindings.xcframework

gomobile-install:
	go install golang.org/x/mobile/cmd/gomobile@latest

bundle:
	zip -r ./build/bundle.zip ./build/WnfsBindings.xcframework && echo "$$(openssl dgst -sha256 ./build/bundle.zip)" > ./build/bundle.zip.sha256

clean:
	rm -rf build/*
