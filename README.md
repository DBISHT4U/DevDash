<div align="center">

```
 ____             ____            _     
|  _ \  _____   _|  _ \  __ _ ___| |__  
| | | |/ _ \ \ / / | | |/ _` / __| '_ \ 
| |_| |  __/\ V /| |_| | (_| \__ \ | | |
|____/ \___| \_/ |____/ \__,_|___/_| |_|
```

<h3>💻 A beautiful terminal dashboard for developers</h3>

<p>
  <strong>System stats · Git info · Process monitor · All in one gorgeous TUI.</strong><br>
  Built with Rust + ratatui for real-time monitoring.
</p>

<p>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.75%2B-orange.svg?logo=rust" alt="Rust"></a>
  <img src="https://img.shields.io/badge/TUI-ratatui-cyan.svg" alt="ratatui">
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-lightgrey.svg" alt="Platform">
  <a href="https://github.com/DBISHT4U/DevDash"><img src="https://img.shields.io/github/repo-size/DBISHT4U/DevDash?color=purple" alt="Repo Size"></a>
</p>

<p>
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-features">Features</a> •
  <a href="#-installation">Installation</a> •
  <a href="#%EF%B8%8F-keybindings">Keybindings</a> •
  <a href="#-tabs">Tabs</a> •
  <a href="#-contributing">Contributing</a>
</p>

<br>

<img src="https://img.shields.io/badge/-%E2%9A%A1%20CPU%20Monitor-E74C3C?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%92%BE%20RAM%20%2F%20Swap-3498DB?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%92%BD%20Disk%20Usage-2ECC71?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%94%8B%20Battery-F39C12?style=for-the-badge" alt="">
<br>
<img src="https://img.shields.io/badge/-%F0%9F%94%80%20Git%20Status-9B59B6?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%93%8A%20Processes-E91E63?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%8C%90%20Network%20I%2FO-1ABC9C?style=for-the-badge" alt="">
<img src="https://img.shields.io/badge/-%F0%9F%93%88%20Sparklines-E67E22?style=for-the-badge" alt="">

</div>

---

## 🚀 Quick Start

```bash
# Clone & build
git clone https://github.com/DBISHT4U/DevDash.git
cd DevDash && cargo build --release

# Run
./target/release/devdash
```

---

<div align="center">
  <img src="demo.gif" alt="DevDash Demo" width="800">
</div>

---

## ✨ Features

| Feature | What It Shows |
|:--|:--|
| ⚡ **CPU Monitor** | Real-time CPU usage gauge + sparkline history (last 60 ticks) |
| 💾 **RAM / Swap** | Used vs total memory with color-coded gauges |
| 💽 **Disk Usage** | All mounted disks with usage bars |
| 🔋 **Battery** | Charge percentage + charging status |
| 🌐 **Network I/O** | Total bytes received / transmitted |
| 📊 **Top Processes** | 8 most CPU-hungry processes with PID, name, CPU%, memory |
| 🔀 **Git Status** | Branch, remote, commits, staged/modified/untracked, ahead/behind |
| 📜 **Commit Log** | Last 10 commits with hash, message, author, date |

---

## 📦 Installation

### Build from Source

```bash
git clone https://github.com/DBISHT4U/DevDash.git
cd DevDash
cargo build --release
./target/release/devdash
```

### Move to PATH (optional)

```bash
sudo cp target/release/devdash /usr/local/bin/
devdash   # run from anywhere
```

---

## ⌨️ Keybindings

| Key | Action |
|:--:|:--|
| `Tab` | Switch between System / Git tabs |
| `Shift+Tab` | Switch back |
| `r` | Force refresh all data |
| `q` / `Esc` | Quit |

---

## 📑 Tabs

<details>
<summary><b>⚡ System Tab</b> — Real-time hardware monitoring</summary>

Shows everything about your machine in one screen:

- **System Info Bar** — hostname, OS, uptime, CPU model, core count, network I/O, battery, process count
- **CPU Panel** — usage gauge + sparkline chart (60-tick history)
- **Memory Panel** — RAM gauge + swap gauge + sparkline chart
- **Disk Panel** — all mounted volumes with usage bars
- **Top Processes** — table of 8 most CPU-intensive processes

All gauges are color-coded:
- 🟢 Green: 0–50%
- 🟡 Yellow: 51–75%
- 🟠 Orange: 76–90%
- 🔴 Red: 91–100%
</details>

<details>
<summary><b>🔀 Git Tab</b> — Repository overview at a glance</summary>

Run `devdash` inside any git repository to see:

- **Branch** — current branch with icon (🔀 main/master, 🌿 feature branches)
- **Remote** — origin URL
- **Status** — staged, modified, untracked file counts
- **Ahead/Behind** — commits ahead/behind remote
- **Commit Count** — total commits on current branch
- **Recent Commits** — last 10 commits table with hash, message, author, relative date

Status color coding:
- 🟢 Green: clean working tree
- 🟡 Yellow: modified/staged files
- 🔵 Cyan: untracked files only
</details>

---

## 🏗️ Tech Stack

| Component | Library |
|:--|:--|
| TUI framework | [ratatui](https://github.com/ratatui/ratatui) |
| Terminal handling | [crossterm](https://github.com/crossterm-rs/crossterm) |
| System info | [sysinfo](https://github.com/GuillaumeGomez/sysinfo) |
| Battery | [battery](https://github.com/svartalf/rust-battery) |
| Git | Native `git` CLI commands |
| Time | [chrono](https://github.com/chronotope/chrono) |
| Username | [whoami](https://github.com/ardaku/whoami) |

---

## 🗺️ Roadmap

- [ ] GPU monitoring
- [ ] Docker container status
- [ ] Customizable layout / config file
- [ ] Theme support (Dracula, Nord, Solarized)
- [ ] Network tab (connections, bandwidth per process)
- [ ] Publish to crates.io
- [ ] Homebrew formula

---

## 🤝 Contributing

Contributions welcome! Ideas:

1. **Add a new tab** — Docker, Network, or custom widget
2. **Improve Git detection** — handle edge cases
3. **Add themes** — custom color schemes

---

## 📜 License

MIT — see [LICENSE](LICENSE)

---

<div align="center">

**Built with 🦀 Rust + ratatui**

<sub>⭐ Star if DevDash made your terminal beautiful</sub>

<br>

<a href="https://github.com/DBISHT4U/DevDash/stargazers"><img src="https://img.shields.io/github/stars/DBISHT4U/DevDash?style=social" alt="Stars"></a>
<a href="https://github.com/DBISHT4U/DevDash/network/members"><img src="https://img.shields.io/github/forks/DBISHT4U/DevDash?style=social" alt="Forks"></a>

</div>
