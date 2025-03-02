// swift-tools-version: 6.0.2

import PackageDescription

let package = Package(
    name: "menu",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .executable(name: "menu", targets: ["MenuApp"]),
    ],
    targets: [
        // Swift target for the main application logic
        .executableTarget(
            name: "MenuApp",
            dependencies: ["RustBridge"], // Depend on the Objective-C/C target
            path: "menuBar",
            sources: ["AppDelegate.swift", "MonitorWidgetApp.swift"], // Only include Swift files
            swiftSettings: [
                .define("DEBUG", .when(configuration: .debug))
            ]
        ),
        // Objective-C/C target for the Rust bridge (private header)
        .target(
            name: "RustBridge",
            dependencies: [], // No dependencies unless you need Rust libraries here
            path: "menuBar", // Directory with Objective-C/C files
            sources: ["RustBridge.h", "RustBridge.m"], // Include both header and implementation
            cSettings: [
                .headerSearchPath(".") // Ensure headers in the same directory are found
            ]
        ),
    ]
)