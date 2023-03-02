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
                name: "WnfsBindings",
                targets: ["WnfsBindings"]),
        ],
        dependencies: [
            // Dependencies declare other packages that this package depends on.
            .package(url: "https://github.com/swift-libp2p/swift-cid.git", .upToNextMajor(from: "0.0.1")),
        ],
        targets: [
            .target(
                name: "WnfsSwift",
                dependencies: ["WnfsBindings", .product(name: "CID", package: "swift-cid"),]),
            .binaryTarget(
                name: "WnfsBindings",
                // You can use local path for faster development
                // path: "../build/Wnfs.xcframework"),
                url: "https://github.com/hhio618/wnfs-build-xcframework/raw/main/bundle.zip",
                checksum: "14375e7b41c9e7fa553294042611b9d67b83fb27a796306ab4e83a66d35d1764"),
           
            .testTarget(
                name: "WnfsSwiftTests",
                dependencies: ["WnfsSwift"]),
        ]
)
