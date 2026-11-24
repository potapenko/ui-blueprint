// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "UIBlueprintSwiftUIRuntime",
    platforms: [.macOS(.v14)],
    products: [.library(name: "UIBlueprintSwiftUIRuntime", type: .static, targets: ["UIBlueprintSwiftUIRuntime"])],
    targets: [
        .target(name: "RuntimeUpdateScope", publicHeadersPath: "include",
                cSettings: [.define("UIBLUEPRINT_RUNTIME_DEBUG", .when(configuration: .debug))]),
        .target(name: "UIBlueprintSwiftUIRuntime", dependencies: ["RuntimeUpdateScope"]),
        .testTarget(name: "UIBlueprintSwiftUIRuntimeTests", dependencies: ["UIBlueprintSwiftUIRuntime", "RuntimeUpdateScope"])
    ]
)
