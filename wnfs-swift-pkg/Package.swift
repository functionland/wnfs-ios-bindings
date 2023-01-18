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
                path: "../Wnfs.xcframework"),
            .testTarget(
                name: "WnfsSwiftTests",
                dependencies: ["WnfsSwift"]),
        ]
)
