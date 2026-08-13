# ⚡ ezcurl

> A fast, keyboard-driven HTTP client for your terminal.

[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
![Status](https://img.shields.io/badge/status-alpha-orange)
[![CI](https://github.com/HugoNayl/ezcurl/actions/workflows/rust.yaml/badge.svg)](https://github.com/HugoNayl/ezcurl/actions/workflows/rust.yaml)
![Crates.io Version](https://img.shields.io/crates/v/ezcurl)

**ezcurl** is a lightweight TUI for building and sending HTTP requests without leaving your terminal.

It combines the simplicity of a graphical HTTP client with a fast, keyboard-first workflow inspired by Vim.

> [!WARNING]
> ezcurl is currently under active development.

## ✨ Features

- ⚡ Terminal-native and keyboard-first
- 🧭 Vim-inspired navigation
- ✏️ Normal / Insert modes
- 🌐 GET, POST, PUT, PATCH, DELETE, HEAD and OPTIONS
- 🔗 Interactive request editing
- 📥 Response viewer
- 🦀 Built with Rust, Ratatui and Reqwest

## 📦 Installation

```bash
cargo install --locked --git https://github.com/HugoNayl/ezcurl.git
```

## ⌨️ Keybindings

| Key             | Action       |
| --------------- | ------------ |
| `h` `j` `k` `l` | Navigate     |
| `Tab`           | Next panel   |
| `i`             | Insert mode  |
| `Esc`           | Normal mode  |
| `Ctrl+s`        | Send request |
| `q`             | Quit         |

## 🤝 Contributing

Contributions, issues and ideas are welcome.

For significant changes, please open an issue first to discuss the approach.

## 📄 License

Licensed under the [Apache License 2.0](LICENSE).
