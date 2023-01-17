all: gen-c-header add-rust-targets x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim lipo-ios xcode-build bundle

gen-c-header: 
	cbindgen --lang C -o include/wnfs.h .

add-rust-targets:
	rustup target add x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim

aarch64-apple-ios:
	cargo build --release --target aarch64-apple-ios

x86_64-apple-ios:
	cargo build --release --target x86_64-apple-ios

aarch64-apple-ios-sim:
	cargo build --release --target aarch64-apple-ios-sim

lipo-ios:
	lipo -create \
	target/x86_64-apple-ios/release/libwnfs.a \
	target/aarch64-apple-ios-sim/release/libwnfs.a \
	-output libwnfs_iossimulator.a

xcode-build:
	xcodebuild -create-xcframework \
	-library ./libwnfs_iossimulator.a \
	-headers ./include/ \
	-output Wnfs.xcframework
	-library ./target/aarch64-apple-ios/release/libwnfs.a \
	-headers ./include/ \


bundle:
	zip -r bundle.zip Wnfs.xcframework && openssl dgst -sha256 bundle.zip

build-fula:
	cd fula-ios-lib && gomobile build -bundleid fula.app -target=ios github.com/functionland/go-fula/mobile

clean:
	rm -rf libwnfs* && rm -rf bundle.zip && rm -rf Wnfs.xc*
