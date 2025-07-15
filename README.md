✅Slint + Servo Skeleton
This project demonstrates how to integrate Slint and Servo in a single Rust application. It showcases a clear separation between UI and content rendering while sharing OpenGL context to embed web content within a native UI.

✅Overview
The skeleton is designed to:

Build a structured workspace separating the UI from web rendering logic.

Render web content using Servo and pass it into Slint using OpenGL.

Provide a minimal, modular foundation suitable for further extension.

✅Key Features
A Slint-based UI with native controls

Servo-powered web rendering using OpenGL

A bridge to display Servo’s rendered output inside Slint as a texture

Separated crates for UI logic and content processing

✅Architecture
The project is split into two Rust crates:

📁 ui/ (Slint UI Crate)
Defines the user interface with .slint files

Handles the event loop and user interactions

Receives and displays the rendered texture from the web crate

📁 web/ (Servo Embedding Crate)
Embeds Servo and manages web page loading

Renders output into an OpenGL framebuffer

Exposes a texture that the UI crate can render

------This separation improves testability, modularity, and future support for multi-process architectures.

✅Project Structure
servo-skeleton/
├── Cargo.toml              # Workspace root (excludes Servo)
├── ui/                     # Slint UI crate
│   ├── Cargo.toml
│   ├── src/
│   └── app.slint
└── web/                    # Servo integration crate
    ├── Cargo.toml
    └── src/
Workspace Configuration

/***Workspace Cargo.toml***/
[workspace]
members = ["ui", "web"]
resolver = "3"
exclude = ["servo"]
[workspace.package]
edition = "2024"
Dependencies

/***ui/Cargo.toml***/

[package]
name = "ui"
version = "0.1.0"
edition = "2024"
[dependencies]
slint = "1.12"
gl = "0.14"

/***web/Cargo.toml***/

[package]
name = "web"
version = "0.1.0"
edition = "2024"
[dependencies]
slint = "1.12"
gl = "0.14"
glutin = "0.13.1"
embedder_traits = { path = "../servo/components/shared/embedder" }
libservo         = { path = "../servo/components/servo" }

## ✅ Share in GitHub Issue

Once it's live, go back to [issue #8735](https://github.com/slint-ui/slint/issues/8735) and comment:

> Hi! I had trouble pushing a full PR due to size limits, so I created a clean minimal demo repo instead:
>
> 👉 (https://github.com/AnujayaVijayakumar/servo-skeleton-hidpi/tree/master)


