all: clean test gen-c-header add-rust-targets x86_64-apple-ios\
 aarch64-apple-ios x86_64-apple-darwin xcode-build bundle

# test:
# 	cargo test
gen-c-header: 
	cargo install cbindgen && cbindgen --lang C -o include/wnfsbindings.h .

add-rust-targets:
	rustup target add x86_64-apple-ios aarch64-apple-ios x86_64-apple-darwin

aarch64-apple-ios:
	cargo build --release --target aarch64-apple-ios

x86_64-apple-ios:
	cargo build --release --target x86_64-apple-ios

x86_64-apple-darwin:
	cargo build --release --target x86_64-apple-darwin

xcode-build:
	xcodebuild -create-xcframework \
	-library ./target/x86_64-apple-ios/release/libwnfsbindings.a \
	-headers ./include/ \
	-library ./target/aarch64-apple-ios/release/libwnfsbindings.a \
	-headers ./include/ \
	-library ./target/x86_64-apple-darwin/release/libwnfsbindings.a \
	-headers ./include/ \
	-output ./build/WnfsBindings.xcframework

bundle:
	cd build && cp  ../LICENSE . &&\
	zip -r ./cocoapods-bundle.zip ./WnfsBindings.xcframework ./LICENSE && echo "$$(openssl dgst -sha256 ./cocoapods-bundle.zip)" > ./cocoapods-bundle.zip.sha256

clean:
	rm -rf build/*
