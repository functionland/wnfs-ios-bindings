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
                url: "https://github.com/functionland/wnfs-ios/raw/main/bundle.zip",
                checksum: "4f5a6c3486c1292b13dc814948d32b95da531b5c0aacd7a86bfdcf5baeb10a6b"),
            .testTarget(
                name: "WnfsSwiftTests",
                dependencies: ["WnfsSwift"]),
        ]
)