// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "AgentUIMac",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(name: "AgentUIMac", targets: ["AgentUIMac"])
    ],
    targets: [
        .executableTarget(
            name: "AgentUIMac",
            path: ".",
            exclude: ["README.md"],
            resources: []
        )
    ]
)
