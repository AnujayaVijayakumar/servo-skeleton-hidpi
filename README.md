sSlint + Servo Skeleton

--A Rust project showing how to integrate Servo and Slint  in one app, with:
Build a tidy workspace with a separation between UI and content processing.
Servo rendering bridged into Slint via OpenGL.

--This skeleton is a minimal demonstration of:
A Slint UI app with native controls.
A Servo-powered content process that renders web pages.
A "bridge" that shares the OpenGL context to display Servo’s rendered output in Slint.


--It’s split into separate crates for UI and web rendering logic.

$ Slint crate
Defines all UI layout and controls, which are responsible for the event loop and user interactions.
$ Web crate
Wraps Servo rendering and handles loading URLs. Exposes OpenGL texture for Slint to draw.

--This separation:
$ Makes testing easier
$ Supports future multi-process design
$ Keeps UI logic clean and Servo logic independent

 
--Project Structure
├── Cargo.toml              # Workspace root
├── ui/                     # Slint UI crate
│   ├── Cargo.toml
│   ├── src/
│   └── app.slint
└── web/                    # Servo embedding crate
    ├── Cargo.toml
    └── src/

--How it works

$ Servo renders web pages using OpenGL.
$ The framebuffer is captured into a texture.
$ Slint displays that texture inside your app using a CustomRenderer.

--Simple workflow  : 

User action → UI sends URL → Web loads URL in Servo → Servo renders → GL framebuffer → Texture → UI draws it

--Requirements

Rust version – 1.74 (2024 Edition)
uv version
choco version
Visual Studio 2022 

--Dependencies

Cargo. Toml in workspace root
[workspace]
members = ["ui", "web"]
resolver  = "3"
exclude  = ["servo"]  

[workspace.package]
edition = "2024"
Note : servo is excluded from the workspace root to avoid building it as a separate crate, since we’ve to integrate that with web crate.

$ ui/ Cargo.Toml
[package]
name = "ui"
version = "0.1.0"
edition = "2024"

[dependencies]
slint = "1.12"
gl = "0.14"

$ web/ Cargo.Toml 
[package]
name = "web"
version = "0.1.0"
edition = "2024"

[dependencies]
slint = "1.12"
gl = "0.14"
glutin = "0.13.1"
embedder_traits = { path = "../servo/components/shared/embedder" }
libservo = { path = "../servo/components/servo" }
