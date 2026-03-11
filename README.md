# GNOME Application Launcher (Rust + GTK4)

A blisteringly fast, minimalistic, and native application launcher for Linux built with Rust, GTK4, and libadwaita. It acts as a lightweight replacement for heavy extensions by utilizing the `gio` system index and running as a persistent DBus daemon in the background to ensure instantaneous startup times natively under Wayland.

## Features

- **Blazing Fast**: Written in Rust.
- **Persistent Daemon**: Runs continuously in the background using `gapplication` to eliminate start-up delays.
- **Fuzzy Search**: Implements `skim` algorithms to instantly match your query against system applications.
- **Native Wayland Design**: Built cleanly on top of GTK4 and Libadwaita for a seamless GNOME experience.
- **Keyboard Friendly**: Navigate with Arrow keys and launch with Enter.

## Installation

### From Pre-built RPM
If you are on an RPM-based distribution (like Fedora), you can install the launcher directly via the generated package:
```bash
sudo rpm -i target/generate-rpm/launcher-0.1.0-1.x86_64.rpm
```

### Build from Source
Ensure you have the GTK4 and libadwaita development headers installed on your system.
```bash
# On Fedora:
sudo dnf install rust cargo gtk4-devel libadwaita-devel

# Clone or navigate to the repository
cargo build --release
```

## Setup & Keyboard Shortcut (Wayland)

Since modern Wayland enforces strict security, global hotkeys aren't supported generically for raw executing binaries. The launcher embraces the "GNOME Way" using D-Bus toggling.

1. Go to **GNOME Settings** -> **Keyboard** -> **Keyboard Shortcuts** -> **View and Customize Shortcuts** -> **Custom Shortcuts**.
2. Click the `+` button.
3. Set the fields:
   - **Name**: Application Launcher
   - **Command**: `launcher` (or `/home/user/code/Launcher/target/release/launcher` if built manually without installing)
   - **Shortcut**: Set to your desired combo (e.g., `Ctrl+Space`).

When you invoke the shortcut:
- **First time**: The daemon boots silently in the background and presents the search bar.
- **Consecutive times**: Hitting the shortcut instantly pings the background daemon to toggle the window. No heavy loading required.

## Building the RPM yourself
If you wish to distribute this launcher:
```bash
# 1. Install cargo-generate-rpm
cargo install cargo-generate-rpm

# 2. Build the release and package it
cargo build --release
cargo generate-rpm
```
You will find the generated `.rpm` inside `target/generate-rpm/`.
