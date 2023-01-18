// swift-tools-version:5.3
import PackageDescription
import Foundation
let package = Package(
        name: "WnfsSwift",
        platforms: [
            .iOS(.v13), 
            .macOS(.v11)
        ],
        products: [
            .library(
                name: "WnfsSwift",
                targets: ["WnfsSwift"]),
            .library(
                name: "Wnfs",
                targets: ["Wnfs"]),
        ],
        targets: [
            .target(
                name: "WnfsSwift",
                dependencies: ["Wnfs"]),
            .binaryTarget(
                name: "Wnfs",
                url: "https://github.com/hhio618/wnfs-build-xcframework/raw/main/bundle.zip",
                checksum: "85e83f29d6b21c65d42d5fe08e82bbe31f98e69a41eaaa7502154ef13bc2c02e"),
            .testTarget(
                name: "WnfsSwiftTests",
                dependencies: ["WnfsSwift"]),
        ]
)
