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
                // path: "../build/WnfsBindings.xcframework"),
                url: "https://github.com/hhio618/wnfs-build-xcframework/raw/main/bundle.zip",
                checksum: "018bc7fb2ee3218beb5a0c89c880d33552887426537a3c352b9faf7642aa6f9f"),
           
            .testTarget(
                name: "WnfsSwiftTests",
                dependencies: ["WnfsSwift"]),
        ]
)
