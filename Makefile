all: gen-c-header add-rust-targets x86_64-apple-ios x86_64-apple-darwin aarch64-apple-darwin\
 aarch64-apple-ios aarch64-apple-ios-sim lipo-ios xcode-build bundle

gen-c-header: 
	cargo install cbindgen && cbindgen --lang C -o build/include/wnfs.h .

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
	target/x86_64-apple-ios/release/libwnfs.a \
	target/aarch64-apple-ios-sim/release/libwnfs.a \
	-output build/libwnfs_iossimulator.a

xcode-build:
	xcodebuild -create-xcframework \
	-library ./build/libwnfs_iossimulator.a \
	-headers ./build/include/ \
	-library ./target/aarch64-apple-ios/release/libwnfs.a \
	-headers ./build/include/ \
	-library ./target/x86_64-apple-darwin/release/libwnfs.a \
	-headers ./build/include/ \
	-output ./build/Wnfs.xcframework

gomobile-install:
	go install golang.org/x/mobile/cmd/gomobile@latest

bundle:
	zip -r bundle.zip ./build/Wnfs.xcframework && echo "$$(openssl dgst -sha256 bundle.zip)" > ./build/bundle.zip.sha256

clean:
	rm -rf libwnfs* && rm -rf bundle.zip && rm -rf Wnfs.xc*
