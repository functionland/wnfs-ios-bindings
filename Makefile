all: clean test gen-c-header add-rust-targets x86_64-apple-ios\
 aarch64-apple-ios x86_64-apple-darwin lipo-ios xcode-build bundles

test:
	cargo test
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

lipo-ios:
	mkdir -p build && \
	lipo -create \
	target/x86_64-apple-ios/release/libwnfsbindings.a \
	target/aarch64-apple-ios/release/libwnfsbindings.a \
	-output build/libwnfsbindings_ios.a

xcode-build:
	xcodebuild -create-xcframework \
	-library ./target/x86_64-apple-ios/release/libwnfsbindings.a \
	-headers ./include/ \
	-library ./target/aarch64-apple-ios/release/libwnfsbindings.a \
	-headers ./include/ \
	-library ./target/x86_64-apple-darwin/release/libwnfsbindings.a \
	-headers ./include/ \
	-output ./build/WnfsBindings.xcframework

gomobile-install:
	go install golang.org/x/mobile/cmd/gomobile@latest

bundles:
	cp -r include build/ && cd build &&\
	zip -r ./swift-bundle.zip ./WnfsBindings.xcframework && echo "$$(openssl dgst -sha256 ./swift-bundle.zip)" > ./swift-bundle.zip.sha256 &&\
	zip -r ./cocoapods-bundle.zip ./include ./libwnfsbindings_ios.a && echo "$$(openssl dgst -sha256 ./cocoapods-bundle.zip)" > ./cocoapods-bundle.zip.sha256

clean:
	rm -rf build/*
