# 🌟 Felty-Oi

<p align="center">
  <img src="assets/header_256.png" alt="Felty-Oi Logo" width="128" height="128">
</p>

# Felty Oï

<p align="center">
  <a href="https://www.rust-lang.org/" title="Hecho con Rust">
    <img src="https://img.shields.io/badge/Rust-1-blue?logo=rust&logoColor=white" alt="Rust">
  </a>
  <a href="https://github.com/Aqui-oi/Felty-oi/blob/main/LICENSE" title="Licencia MPL-2.0">
    <img src="https://img.shields.io/badge/License-MPL--2.0-blue?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/Aqui-oi/Felty-oi/stargazers" title="Ver estrellas">
    <img src="https://img.shields.io/github/stars/Aqui-oi/Felty-oi?style=flat-square" alt="GitHub Stars">
  </a>
</p>

**Felty Oï** es una aplicación gráfica de transferencia de archivos rápida, intuitiva y inteligente para múltiples plataformas.  
Permite mover o copiar carpetas enteras con **reglas de exclusión** (por extensión, nombre o carpeta) y pronto buscará soportar **conexión remota** vía host/puerto.

Desarrollada en **Rust** con el moderno framework [Dioxus](https://dioxuslabs.com), ofrece una interfaz fluida, multilingüe y estética (soporta GTK, TailwindCSS, íconos personalizados...).

## 🌍 Documentación disponible

- 🇫🇷 [Francés](/README.md)
- 🇬🇧 [Inglés](/docs/en.md)
- 🇪🇸 [Español](/docs/es.md)
- 🇩🇪 [Alemán](/docs/de.md)

## 🚀 Funcionalidades principales

- 🔄 Transferencia de archivos y carpetas locales
- ⚙️ Sistema de exclusión avanzado:
  - Por **extensión** (`.tmp`, `.log`, etc.)
  - Por **nombre de archivo**
  - Por **carpeta**
- 🖱️ Interfaz moderna e intuitiva (Dioxus Desktop + TailwindCSS)
- 📊 Barra de progreso, tiempo estimado y notificaciones
- 🌐 Aplicación multilingüe (EN, FR, ES, DE)
- 🌙 Tema oscuro integrado
- 💾 Compilación nativa para **Linux**, **Windows**, **macOS**

## 🖥️ Captura de pantalla

> *(interfaz)*

## 📦 Instalación

### ✅ Requisitos

- [Rust](https://rust-lang.org)
- [Dioxus CLI](https://github.com/DioxusLabs/cli): `cargo install dioxus-cli`
- GTK (para Linux): `sudo apt install libgtk-3-dev libglib2.0-dev`

### 🔧 Compilación

#### 🔹 Modo Desarrollo

```bash
dx serve
```

#### 🔹 Modo Producción

```bash
dx build --platform desktop --release
```

> *(El binario final se encontrará en: `target/dx/felty-oi/release/{windows|linux|macos}/felty-oi`)*

#### 🧠 En progreso

-   📡 Transferir a un servidor remoto (host, puerto…)

-   🧩 Detección automática de conflictos y resolución

-   📋 Historial de transferencias

-   🔐 Verificación de integridad (hash SHA-256)

-   📁 Arrastrar y soltar (Drag & Drop)

### 📝 Contribuir

¿Quieres ayudar a mejorar la aplicación?  
Se abre a PRs y sugerencias! Envía una idea a través de [issues](https://github.com/Aqui-oi/Felty-oi/issues/1).

### 🧑‍💻 Autor

Proyecto mantenido por [`Zyouax`](https://github.com/zyouax)  
Hecho con ❤️ en Rust 🦀