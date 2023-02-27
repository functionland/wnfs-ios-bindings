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
        dependencies: [
            // Dependencies declare other packages that this package depends on.
            .package(url: "https://github.com/swift-libp2p/swift-cid.git", .upToNextMajor(from: "0.0.1")),
        ],
        targets: [
            .target(
                name: "WnfsSwift",
                dependencies: ["Wnfs", .product(name: "CID", package: "swift-cid"),]),
            .binaryTarget(
                name: "Wnfs",
                // You can use local path for faster development
                // path: "../build/Wnfs.xcframework"),
                url: "https://github.com/hhio618/wnfs-build-xcframework/raw/main/bundle.zip",
                checksum: "d0197cca1dfd4bd7fd84e1438b87876215d42d4a80dd0e716d0b509ddb1d7bb0"),
           
            .testTarget(
                name: "WnfsSwiftTests",
                dependencies: ["WnfsSwift"]),
        ]
)
