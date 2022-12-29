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
        ],
        targets: [
            .target(
                name: "WnfsSwift",
                dependencies: ["Wnfs"]),
            .binaryTarget(
                name: "Wnfs",
                url: "https://github.com/functionland/wnfs-ios/bundle.zip",
                checksum: ""),
            .testTarget(
                name: "WnfsSwiftTests",
                dependencies: ["WnfsSwift"]),
        ]
)