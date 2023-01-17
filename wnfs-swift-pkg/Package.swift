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
                checksum: "390f4df8af3ff9567c051a3891c1a430331e5221d89ae9b0c0185c93a9243519"),
            .testTarget(
                name: "WnfsSwiftTests",
                dependencies: ["WnfsSwift"]),
        ]
)
