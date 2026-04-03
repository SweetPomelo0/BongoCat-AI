# BongoCat AI

<p align="right">
  English | <a href="./README.zh-CN.md">简体中文</a>
</p>

<p align="center">
  <b>A local-first AI desktop companion with transparent Markdown memory.</b><br>
  It reacts to your keyboard, mouse, and controller, supports chat, and keeps persistent companion memory in editable local files.
</p>

<p align="center">
  <a href="https://github.com/SweetPomelo0/BongoCat-AI/releases"><img alt="Release" src="https://img.shields.io/github/v/release/SweetPomelo0/BongoCat-AI?style=flat-square"></a>
  <a href="./LICENSE"><img alt="License" src="https://img.shields.io/github/license/SweetPomelo0/BongoCat-AI?style=flat-square"></a>
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=flat-square">
  <img alt="Stack" src="https://img.shields.io/badge/Tauri%202-Vue%203-Rust-8b5cf6?style=flat-square">
</p>

<p align="center">
  <a href="#quick-start">Quick Start</a> ·
  <a href="https://github.com/SweetPomelo0/BongoCat-AI/releases">Download</a> ·
  <a href="#highlights">Highlights</a> ·
  <a href="#local-memory">Local Memory</a> ·
  <a href="./.github/CONTRIBUTING.md">Contributing</a>
</p>

> Based on [ayangweb/BongoCat](https://github.com/ayangweb/BongoCat), this fork pushes the project toward a more companion-like direction: not just a cute desktop pet, but a pet that can chat, remember, and stay with you over time.

---

## Why BongoCat AI

Most AI products still live inside a chat box.

**BongoCat AI** takes a different direction: a local-first AI desktop companion with a persistent memory layer that stays close to the desktop instead of hiding behind a web app.

It keeps the original BongoCat feeling — lightweight, cute, responsive — and extends it with:

- real-time keyboard, mouse, and gamepad reactions
- a dedicated companion chat window
- editable Markdown memory with persistent context
- desktop-native controls like tray, startup, always-on-top, and window behavior settings

---

## Highlights

### Feels like a desktop pet

- Live2D desktop pet rendering with transparent background
- Real-time reactions to keyboard, mouse, and controller input
- Window size, opacity, radius, position, and click-through controls
- Tray controls and desktop-friendly behavior for long-running use

### Adds AI without turning into a generic chatbot

- Dedicated companion chat window
- Configurable provider, model, base URL, and API key
- Streaming-style replies and multi-turn context
- Designed around companion UX instead of pure tool-style interaction

### Local memory, not black-box memory

- Memory lives in editable local Markdown files
- `PERSONA.md` for character and tone
- `MEMORY.md` for durable memory and stable preferences
- `daily/YYYY-MM-DD.md` for recent context and short-term notes
- Inspired by the same transparent, local-first memory direction that made projects like OpenClaw resonate, without hiding state behind a closed memory layer

### Built for desktop use

- Tauri 2 + Vue 3 + TypeScript + Rust + Live2D
- Native tray, startup, shortcut, and window integration
- Windows, macOS, and Linux (X11) support
- Distributed through GitHub Releases

---

## Screenshots

| macOS                                                                                                     | Windows                                                                                                   | Linux                                                                                                       |
| --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| ![](https://raw.githubusercontent.com/ayangweb/BongoCat/refs/heads/master/docs/images/screenshot-mac.jpg) | ![](https://raw.githubusercontent.com/ayangweb/BongoCat/refs/heads/master/docs/images/screenshot-win.jpg) | ![](https://raw.githubusercontent.com/ayangweb/BongoCat/refs/heads/master/docs/images/screenshot-linux.jpg) |

---

## Quick Start

1. Download the latest release:
   - [GitHub Releases](https://github.com/SweetPomelo0/BongoCat-AI/releases)
2. If you need package names for your platform:
   - [Download Guide](./.github/DOWNLOAD_GUIDE.md)
3. Launch the app and try:
   - the desktop pet window
   - input reactions
   - tray controls
   - chat window and AI settings

If you want a more personal setup, configure your model, AI provider, and memory settings in Preferences.

---

## What makes this different from the original BongoCat

The original project already delivers a polished desktop pet with Live2D rendering and input reactions.

This fork adds a second layer on top of that foundation:

- companion chat, not just animation
- persistent memory context, not just one-off replies
- editable local memory files, not invisible black-box storage

The goal is simple:
turn a desktop pet into a local-first AI companion layer rather than just a themed chat UI.

---

## Local Memory

BongoCat AI uses a local-first Markdown memory workspace to keep companion context persistent, transparent, and user-editable.

Current memory files are stored like this:

```text
<appData>/memory/
├── PERSONA.md
├── MEMORY.md
└── daily/
    └── YYYY-MM-DD.md
```

### What each file does

- `PERSONA.md`: personality, tone, and character direction
- `MEMORY.md`: long-term facts and stable preferences
- `daily/YYYY-MM-DD.md`: day-specific context and temporary notes

### Design principles

- **Markdown is the source of truth**
- **Core memory stays local and visible**
- **Users can inspect and edit what is remembered**

---

## Run from Source

### Requirements

- [Rust](https://v2.tauri.app/start/prerequisites/)
- [Node.js](https://nodejs.org/en/)
- [pnpm](https://pnpm.io/)

### Install dependencies

```bash
pnpm install
```

### Start development

```bash
pnpm tauri dev
```

### Build the app

```bash
pnpm tauri build
```

---

## Roadmap

- [x] Live2D desktop pet with multi-input reactions
- [x] Chat window and base AI settings
- [x] Local Markdown memory MVP
- [x] Transparent memory file access from settings
- [ ] More robust structured memory extraction
- [ ] Better retrieval from historical daily notes
- [ ] Emotion and proactive behavior systems
- [ ] Richer environment awareness

---

## Who this is for

- developers who want a desktop pet with real interaction
- users who like cute UI but still want practical AI features
- people interested in local-first memory instead of opaque AI state
- builders exploring the overlap between desktop pets, agents, and companion UX

---

## Related Project

- [ayangweb/BongoCat](https://github.com/ayangweb/BongoCat)

---

## Contributing

If you like the idea of combining desktop pets, AI, and local memory, contributions are welcome.

- Read the [Contributing Guide](./.github/CONTRIBUTING.md)
- Open an issue or pull request
- Share ideas for models, interactions, and companion behavior

---

## License

This project is based on the upstream MIT-licensed project and remains compatible with that license. See [LICENSE](./LICENSE).
