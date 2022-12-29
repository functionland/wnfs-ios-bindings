gen-c-header: 
	cbindgen --lang C -o include/wnfs.h .

add-rust-target-mac:
	rustup target add x86_64-apple-darwin aarch64-apple-darwin

add-rust-target-ios:
	rustup target add aarch64-apple-darwin x86_64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios-macabi aarch64-apple-ios-macabi

add-rust-targets: add-rust-ios add-rust-mac

aarch64-apple-ios:
	cargo build --release --target aarch64-apple-ios

x86_64-apple-darwin:
	cargo build --release --target x86_64-apple-darwins

aarch64-apple-darwin:
	cargo build --release --target aarch64-apple-darwin

x86_64-apple-ios:
	cargo build --release --target x86_64-apple-ios

aarch64-apple-ios-sim:
	cargo build --release --target aarch64-apple-ios-sim

x86_64-apple-ios-macabi:
	cargo build --release --target x86_64-apple-ios-macabi

aarch64-apple-ios-macabi:
	cargo build --release --target aarch64-apple-ios-macabi

lipo-macos:
	lipo -create \
	target/x86_64-apple-darwin/release/libwnfs.a \
	target/aarch64-apple-darwin/release/libwnfs.a \
	-output libwnfs_macos.a

lipo-ios:
	lipo -create \
	target/x86_64-apple-ios/release/libwnfs.a \
	target/aarch64-apple-ios-sim/release/libwnfs.a \
	-output libwnfs_iossimulator.a

lipo-macabi:
	lipo -create \
	target/x86_64-apple-ios-macabi/release/libwnfs.a \
	target/aarch64-apple-ios-macabi/release/libwnfs.a \
	-output libwnfs_maccatalyst.a

xcode-build:
	xcodebuild -create-xcframework \
	-library ./libwnfs_macos.a \
	-headers ./include/ \
	-library ./libwnfs_iossimulator.a \
	-headers ./include/ \
	-library ./libwnfs_maccatalyst.a \
	-headers ./include/ \
	-library ./target/aarch64-apple-ios/release/libwnfs.a \
	-headers ./include/ \
	-output Wnfs.xcframework

bundle:
	zip -r bundle.zip Wnfs.xcframework && openssl dgst -sha256 bundle.zip
