// swift-tools-version: 6.0
import PackageDescription

// Tag-pinned source entry point. Nested packages remain usable by local owners.
let package = Package(
    name: "UIBlueprint",
    platforms: [.macOS(.v14)],
    products: [
        .library(name: "UIBlueprintSwiftUI", targets: ["UIBlueprintSwiftUI"]),
        .library(name: "UIBlueprintSwiftUIRuntime", type: .static, targets: ["UIBlueprintSwiftUIRuntime"])
    ],
    targets: [
        .target(name: "UIBlueprintSwiftUI", path: "plugins/swiftui/Sources/UIBlueprintSwiftUI"),
        .target(name: "RuntimeUpdateScope", path: "plugins/swiftui-runtime/Sources/RuntimeUpdateScope",
                publicHeadersPath: "include",
                cSettings: [.define("UIBLUEPRINT_RUNTIME_DEBUG", .when(configuration: .debug))]),
        .target(name: "UIBlueprintSwiftUIRuntime", dependencies: ["RuntimeUpdateScope"],
                path: "plugins/swiftui-runtime/Sources/UIBlueprintSwiftUIRuntime"),
        .testTarget(name: "UIBlueprintSwiftUITests", dependencies: ["UIBlueprintSwiftUI"],
                    path: "plugins/swiftui/Tests/UIBlueprintSwiftUITests"),
        .testTarget(name: "UIBlueprintSwiftUIRuntimeTests", dependencies: ["UIBlueprintSwiftUIRuntime", "RuntimeUpdateScope"],
                    path: "plugins/swiftui-runtime/Tests/UIBlueprintSwiftUIRuntimeTests")
    ]
)
