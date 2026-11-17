<div align="center">

# 💻 DevDash

### A beautiful terminal dashboard for developers

<p>
  <strong>System stats · Git info · Process monitor · All in one gorgeous TUI</strong>
</p>

<p>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.75%2B-orange.svg?logo=rust" alt="Rust"></a>
  <a href="https://crates.io/crates/devdash"><img src="https://img.shields.io/crates/v/devdash.svg" alt="Crates.io"></a>
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-lightgrey.svg" alt="Platform">
</p>

</div>

---

<div align="center">
  <img src="demo.gif" alt="DevDash Demo" width="800">
</div>

## ✨ Features

- **⚡ System Tab** — CPU usage with sparkline history, RAM/Swap gauges, disk usage, battery status, network I/O, top processes
- **🔀 Git Tab** — Branch, remote, commit count, staged/modified/untracked files, ahead/behind, recent commit log
- **🎨 Beautiful** — Rounded borders, color-coded gauges, responsive layout
- **⚡ Fast** — Built with Rust + ratatui, updates every second
- **📦 Single binary** — No runtime dependencies

## 🚀 Install

```bash
cargo install devdash
```

Or build from source:

```bash
git clone https://github.com/DBISHT4U/devdash.git
cd devdash && cargo build --release
./target/release/devdash
```

## ⌨️ Keybindings

| Key | Action |
|-----|--------|
| `Tab` | Switch between System / Git tabs |
| `r` | Force refresh all data |
| `q` / `Esc` | Quit |

## 📜 License

MIT

---

<div align="center">
<b>Built with 🦀 Rust + ratatui</b>
</div>
