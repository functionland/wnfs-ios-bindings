# Wnfs Swift bindings
This is home to the wnfs bindings library for the Apple mac/ios devices.
## Requirements
- An OSX environment
- Latest version of Golang and Gomobile installed
- Latest version of Rust installed
## How to build
Run `make`, this will create a `Wnfs.xcframework` directory in the project root file.

## Distribution of the xcframework
Please follow this doc from the apple documentations: https://developer.apple.com/documentation/xcode/distributing-binary-frameworks-as-swift-packages

## Testing the swift package
The swift package is located in the  `wnfs-swift-package` sub-directory. run the following commands to test the package:
```sh
$ cd wnfs-swift-package
$ swift build
$ swift test
```
