# 🌟 Felty‑Oi

<p align="center">
  <img src="assets/header_256.png" alt="Felty‑Oi Logo" width="128" height="128">
</p>

<h1 align="center">Felty Oï</h1>

<p align="center">
  <a href="https://www.rust-lang.org/" title="Made with Rust">
    <img src="https://img.shields.io/badge/Rust-1-blue?logo=rust&logoColor=white" alt="Rust">
  </a>
  <a href="https://github.com/Aqui-oi/Felty-oi/blob/main/LICENSE" title="MPL-2.0 License">
    <img src="https://img.shields.io/badge/License-MPL--2.0-blue?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/Aqui-oi/Felty-oi/stargazers" title="Voir les étoiles">
    <img src="https://img.shields.io/github/stars/Aqui-oi/Felty-oi?style=flat-square" alt="GitHub Stars">
  </a>
</p>


**Felty Oï** est une application graphique multiplateforme de transfert de fichiers rapide, intuitive et intelligente.  
Elle permet de déplacer ou copier des dossiers entiers avec des **règles d’exclusion** (par extension, nom ou dossier) et vise bientôt la **connexion à distance** via hôte/port.

Développée en **Rust** avec le framework moderne [Dioxus](https://dioxuslabs.com), elle offre une interface fluide, multilingue et esthétique (supporte GTK, TailwindCSS, icônes personnalisées...).


## 🌍 Documentations disponibles

- 🇫🇷 [Français](/README.md)
- 🇬🇧 [English](/docs/en.md)
- 🇪🇸 [Español](/docs/es.md)
- 🇩🇪 [Deutsch](/docs/de.md)


## 🚀 Fonctionnalités principales

- 🔄 Transfert de fichiers et dossiers locaux
- ⚙️ Système d’exclusions avancé :
  - Par **extension** (`.tmp`, `.log`, etc.)
  - Par **nom de fichier**
  - Par **dossier**
- 🖱️ Interface moderne et intuitive (Dioxus Desktop + TailwindCSS)
- 📊 Barre de progression, temps estimé et notifications
- 🌐 Application multilingue (EN, FR, ES, DE)
- 🌙 Thème sombre intégré
- 💾 Build natif pour **Linux**, **Windows**, **macOS**


## 🖥️ Capture d'écran

> *(l’interface)*


## 📦 Installation

### ✅ Requis

- [Rust](https://rust-lang.org)
- [Dioxus CLI](https://github.com/DioxusLabs/cli) : `cargo install dioxus-cli`
- GTK (pour Linux) : `sudo apt install libgtk-3-dev libglib2.0-dev`

### 🔧 Build

#### 🔹 Mode Développement

```bash
dx serve
```

#### 🔹 Mode Production

```bash
dx build --platform desktop --release
```

> *(Le binaire final se trouvera dans : `target/dx/felty-oi/release/{windows|linux|macos}/felty-oi`)*


#### 🧠 À venir

-   📡 Transfert vers un serveur distant (host, port…)

-   🧩 Détection automatique de conflits et résolutions

-   📋 Historique des transferts

-   🔐 Vérification d’intégrité (hash SHA-256)

-   📁 Glisser-déposer (Drag & Drop)

## 📝 Contribuer

Vous souhaitez contribuer à l’amélioration de l’application ?

Ouvert aux PR et suggestions ! Propose une idée via [issues](https://github.com/Aqui-oi/Felty-oi/issues/1)


## 🧑‍💻 Auteur

Projet maintenu par [`Zyouax`](https://github.com/zyouax)

Made with ❤️ in Rust 🦀
