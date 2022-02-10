# MacroGraph

A framework and application for node-based programming.

This project is split up into multiple parts to allow for multiple uses:

- Core: Rust library that performs the work of processing events and managing nodes. This can be embedded in your own codebase.
- Packages: Rust libraries that are loaded at runtime by the Core that expose custom functionality via Engines and node schemas. These are what make MacroGraph actually useful.
- Package API: Rust library that is shared by both the Core and packages which allows packages to communicate with the Core, without actually being aware of implementation details. It should contain as little code as possible since it is included with every package binary.
- App: A [Tauri]()-powered desktop application that allows for visual editing of node graphs. The web frontend communicates via Tauri's message passing with the Rust backend that runs the Core, among other things. I plan to separate the web frontend into its own package so that it can be used for both the desktop app and online, hence why the Core is communication protocol-agnostic.

## Running

1. Make sure you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [pnpm](https://pnpm.io/) installed.
2. Run `pnpm i`
3. Run `cargo build`
4. Run `pnpm tauri dev`

All the packages are loaded at runtime, including the OBS package. Currently, a package engine crashing will cause the entire app to crash, so either have OBS running with an OBS Websocket v5 alpha installed or remove the `"obs"` entry in [the app's main file](app/src-tauri/src/main.rs).
## [Figma Designs](https://www.figma.com/file/VO7zmohUtZSqC1eIyGUuN3/MacroGraph-Designs)

The app mostly follows these designs, highly based off of Unreal Engine's blueprints

## Acknowledgements

MacroGraph makes use of countless amounts of other peoples' work, and to acknowledge all of it would take an incredibly long time. However, I would like to make a few notable mentions that have heavily assisted in making MacroGraph possible.

- The Rust team & community: For creating and developing what is probably my favourite programming language
- The Tauri team: For creating an exceptional UI development library that uses Rust and doesn't hog CPU and RAM like Electron
- Aleph-Alpha and other ts-rs contributors: For allowing MacroGraph's frontend and backend to safely communicate
- Epic Games: For creating Unreal Engine Blueprints which inspired this project in the first place