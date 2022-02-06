# MacroGraph

A framework and application for node-based programming.

This project is split up into multiple parts to allow for multiple uses:
- Core: The Rust code that performs the work of processing events and managing nodes. This can be used on its own and embedded in your own codebase.
- App: A [Tauri]()-powered desktop application that allows for visual editing of node graphs. The web frontend communicates via Tauri's message passing with the Rust backend that runs the Core, among other things. I plan to separate the web frontend into its own package so that it can be used for both the desktop app and online, hence why the Core is communication protocol-agnostic.

## Running

1. Make sure you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [pnpm](https://pnpm.io/) installed.
2. Run `pnpm i`
3. Run `cargo build`
4. Run `pnpm tauri dev`

All the packages are loaded at runtime, including the OBS package. Currently, a package engine crashing will cause the entire app to crash, so either have OBS running with an OBS Websocket v5 alpha installed or remove the `"obs"` entry in `app/src-tauri/src/main.rs`


## Figma Designs

The app mostly follows these designs, highly based off of Unreal Engine's blueprints

https://www.figma.com/file/VO7zmohUtZSqC1eIyGUuN3/MacroGraph-Designs?node-id=0%3A1
