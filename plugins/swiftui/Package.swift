// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "UIBlueprintSwiftUI",
    platforms: [.macOS(.v14)],
    products: [.library(name: "UIBlueprintSwiftUI", targets: ["UIBlueprintSwiftUI"])],
    targets: [
        .target(name: "UIBlueprintSwiftUI"),
        .testTarget(name: "UIBlueprintSwiftUITests", dependencies: ["UIBlueprintSwiftUI"])
    ]
)
