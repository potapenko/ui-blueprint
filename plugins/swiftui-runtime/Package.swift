// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "UIBlueprintSwiftUIRuntime",
    platforms: [.macOS(.v14)],
    products: [.library(name: "UIBlueprintSwiftUIRuntime", type: .static, targets: ["UIBlueprintSwiftUIRuntime"])],
    targets: [
        .target(name: "UIBlueprintSwiftUIRuntime"),
        .testTarget(name: "UIBlueprintSwiftUIRuntimeTests", dependencies: ["UIBlueprintSwiftUIRuntime"])
    ]
)
