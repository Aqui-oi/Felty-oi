# 🌟 Felty-Oi

<p align="center">
  <img src="/assets/header_256.png" alt="Felty-Oi Logo" width="128" height="128">
</p>

# Felty Oï

<p align="center">
  <a href="https://www.rust-lang.org/" title="Made with Rust">
    <img src="https://img.shields.io/badge/Rust-1-blue?logo=rust&logoColor=white" alt="Rust">
  </a>
  <a href="https://github.com/Aqui-oi/Felty-oi/blob/main/LICENSE" title="MPL-2.0 License">
    <img src="https://img.shields.io/badge/License-MPL--2.0-blue?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/Aqui-oi/Felty-oi/stargazers" title="View stars">
    <img src="https://img.shields.io/github/stars/Aqui-oi/Felty-oi?style=flat-square" alt="GitHub Stars">
  </a>
</p>

**Felty Oï** is a fast, intuitive, and smart multiplatform graphical file transfer application.  
It allows moving or copying entire folders with **exclusion rules** (by extension, name, or folder) and aims soon to support **remote connection** via host/port.

Developed in **Rust** with the modern [Dioxus](https://dioxuslabs.com) framework, it offers a smooth, multilingual, and aesthetic interface (supports GTK, TailwindCSS, custom icons...).

## 🌍 Available Documentation

- 🇫🇷 [Français](/README.md)
- 🇬🇧 [English](/docs/en.md)
- 🇪🇸 [Español](/docs/es.md)
- 🇩🇪 [Deutsch](/docs/de.md)

## 🚀 Main Features

- 🔄 Local file and folder transfer
- ⚙️ Advanced exclusion system:
  - By **extension** (`.tmp`, `.log`, etc.)
  - By **filename**
  - By **folder**
- 🖱️ Modern and intuitive interface (Dioxus Desktop + TailwindCSS)
- 📊 Progress bar, estimated time, and notifications
- 🌐 Multilingual application (EN, FR, ES, DE)
- 🌙 Built-in dark theme
- 💾 Native build for **Linux**, **Windows**, **macOS**

## 🖥️ Screenshot

> *(the interface)*

<p align="center">
  <img src="assets/file.png" alt="Screenshot 1" width="600">
</p>

> *(the interface with the filtering option)*

<p align="center">
  <img src="assets/file_extension.png" alt="Screenshot 2" width="600">
</p>

> *(About the interface)*

<p align="center">
  <img src="assets/about.png" alt="Screenshot 3" width="600">
</p>

## 📦 Installation

### ✅ Requirements

- [Rust](https://rust-lang.org)
- [Dioxus CLI](https://github.com/DioxusLabs/cli) : `cargo install dioxus-cli`
- GTK (for Linux) : `sudo apt install libgtk-3-dev libglib2.0-dev`

### 🔧 Build

#### 🔹 Development Mode

```bash
dx serve
```

#### 🔹 Production Mode

```bash
dx build --platform desktop --release
```

> *(The final binary will be in: `target/dx/felty-oi/release/{windows|linux|macos}/felty-oi`)*


#### 🧠 Upcoming
- 📡 Transfer to a remote server (host, port…)
- 🧩 Automatic conflict detection and resolution
- 📋 Transfer history
- 🔐 Integrity check (SHA-256 hash)
- 📁 Drag & Drop support

### 📝 Contribute
Want to help improve the application?  
Open to PRs and suggestions! Submit an idea via [issues](https://github.com/Aqui-oi/Felty-oi/issues/1).

### 🧑‍💻 Author
Project maintained by [`Zyouax`](https://github.com/zyouax)  
Made with ❤️ in Rust 🦀
