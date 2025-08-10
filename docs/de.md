# 🌟 Felty-Oi

<p align="center">
  <img src="/assets/header_256.png" alt="Felty-Oi Logo" width="128" height="128">
</p>

# Felty Oï

<p align="center">
  <a href="https://www.rust-lang.org/" title="Entwickelt mit Rust">
    <img src="https://img.shields.io/badge/Rust-1-blue?logo=rust&logoColor=white" alt="Rust">
  </a>
  <a href="https://github.com/Aqui-oi/Felty-oi/blob/main/LICENSE" title="MPL-2.0 Lizenz">
    <img src="https://img.shields.io/badge/License-MPL--2.0-blue?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/Aqui-oi/Felty-oi/stargazers" title="Sterne anzeigen">
    <img src="https://img.shields.io/github/stars/Aqui-oi/Felty-oi?style=flat-square" alt="GitHub Stars">
  </a>
</p>

**Felty Oï** ist eine schnelle, intuitive und intelligente grafische Dateiübertragungsanwendung für mehrere Plattformen.  
Sie ermöglicht das Verschieben oder Kopieren ganzer Ordner mit **Ausschlussregeln** (nach Erweiterung, Name oder Ordner) und strebt bald **ferne Verbindungen** über Host/Port an.

Entwickelt in **Rust** mit dem modernen [Dioxus](https://dioxuslabs.com) Framework, bietet es eine flüssige, mehrsprachige und ästhetische Benutzeroberfläche (unterstützt GTK, TailwindCSS, benutzerdefinierte Icons...).

## 🌍 Verfügbare Dokumentation

- 🇫🇷 [Französisch](/README.md)
- 🇬🇧 [Englisch](/docs/en.md)
- 🇪🇸 [Spanisch](/docs/es.md)
- 🇩🇪 [Deutsch](/docs/de.md)

## 🚀 Hauptfunktionen

- 🔄 Lokale Datei- und Ordnerübertragung
- ⚙️ Erweitertes Ausschlusssystem:
  - Nach **Erweiterung** (`.tmp`, `.log`, etc.)
  - Nach **Dateiname**
  - Nach **Ordner**
- 🖱️ Moderne und intuitive Benutzeroberfläche (Dioxus Desktop + TailwindCSS)
- 📊 Fortschrittsbalken, geschätzte Zeit und Benachrichtigungen
- 🌐 Mehrsprachige Anwendung (EN, FR, ES, DE)
- 🌙 Integriertes Dunkles Thema
- 💾 Nativer Build für **Linux**, **Windows**, **macOS**

## 🖥️ Screenshot

> *(Oberfläche)*

<p align="center">
  <img src="assets/file.png" alt="Screenshot 1" width="600">
</p>

> *(Oberfläche mit Filtermöglichkeit)*

<p align="center">
  <img src="assets/file_extension.png" alt="Screenshot 2" width="600">
</p>

> *(Übrigens, an der Oberfläche)*

<p align="center">
  <img src="assets/about.png" alt="Screenshot 3" width="600">
</p>

## 📦 Installation

### ✅ Voraussetzungen

- [Rust](https://rust-lang.org)
- [Dioxus CLI](https://github.com/DioxusLabs/cli): `cargo install dioxus-cli`
- GTK (für Linux): `sudo apt install libgtk-3-dev libglib2.0-dev`

### 🔧 Build

#### 🔹 Entwicklungsmodus

```bash
dx serve
```

#### 🔹 Produktivmodus

```bash
dx build --platform desktop --release
```

> *(Das finale Binary wird in: `target/dx/felty-oi/release/{windows|linux|macos}/felty-oi` gefunden)*

#### 🧠 Demnächst
- 📡 Übertragung zu einem Remote-Server (Host, Port…)
- 🧩 Automatische Konflikt-Erkennung und -Lösung
- 📋 Transferhistorie
- 🔐 Integritätsprüfung (SHA-256-Hash)
- 📁 Drag & Drop-Unterstützung

### 📝 Mitwirken
Möchtest du bei der Verbesserung der Anwendung helfen?  
Öffne Pull-Anfragen und Vorschläge! Schicke eine Idee über [issues](https://github.com/Aqui-oi/Felty-oi/issues/1).

### 🧑‍💻 Autor
Projekt betrieben von [`Zyouax`](https://github.com/zyouax)  
Gemacht mit ❤️ in Rust 🦀
