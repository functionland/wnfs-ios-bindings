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
                checksum: "db8323fdf12a34c44c55acd9f1da37a36c74c8790e49158f1238eebeda4bee03"),
            .testTarget(
                name: "WnfsSwiftTests",
                dependencies: ["WnfsSwift"]),
        ]
)
