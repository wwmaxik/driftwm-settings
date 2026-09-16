# driftwm-settings

Fast, lightweight, pure Rust configuration utility for the [driftwm](https://github.com/malbiruk/driftwm) Wayland compositor, built with [iced](https://github.com/iced-rs/iced) 0.13+ and styled in **Catppuccin Mocha**.

![Version](https://img.shields.io/badge/version-0.5.0-blue)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)
![GUI](https://img.shields.io/badge/GUI-iced_0.13-purple)
![License](https://img.shields.io/badge/license-GPLv3-blue)

## Features

- 🦀 **Pure Rust & Lightweight** — Completely rewritten from legacy GTK4 to Iced 0.13. Fast startup, minimal memory footprint, zero GTK/C-binding overhead.
- 🎨 **Catppuccin Mocha Theme** — Beautiful, modern dark UI designed specifically for Wayland environments.
- 📝 **Lossless TOML Editing (`toml_edit`)** — Preserves your user comments, custom formatting, and unmanaged tables (keybindings, autostart) in `~/.config/driftwm/config.toml`.
- 🔍 **Config Validation & Diagnostics** — Integrated check via `driftwm --check-config` before saving, with built-in semantic fallback validation and helpful warning banners.
- 🪟 **Comprehensive driftwm 0.19+ Support**:
  - **General / Placement**: Window placement (`center`, `cursor`, `auto`), focus placement (center, edges, corners), mod keys, sloppy focus, navigation on close, session restore.
  - **Appearance & Deco**: Decoration mode (`client`, `minimal`, `none`), window spacing (`gap`), screen inset (`outer_gap`), corner radius, borders, colors, opacity, blur, SSD titlebar fonts.
  - **Background**: Mode switch (`default` dot-grid, `shader`, `tile`, `wallpaper`, `none`), file pickers, mirror-fold tiles, shader cache and transparent settings, presets.
  - **Bookmarks & Nav**: Named canvas coordinates (`[x, y]`), anchors, camera lerp factor, drift momentum, edge-pan thresholds, and zoom configuration.
  - **Input Devices**: Keyboard layout/repeat/options, trackpad tap/drag/natural-scroll/speed, mouse speed/profile.

## Installation

### Prerequisites

- Rust 1.85+ (stable)
- Linux Wayland / X11 development headers:
  ```bash
  # Debian / Ubuntu
  sudo apt install -y pkg-config libxkbcommon-dev libfontconfig1-dev
  # Arch Linux
  sudo pacman -S --needed pkg-config libxkbcommon fontconfig
  # Fedora
  sudo dnf install -y pkgconf-pkg-config libxkbcommon-devel fontconfig-devel
  ```

### Build & Run

```bash
cd driftwm-settings
cargo build --release
./target/release/driftwm-settings
```

### Install system-wide

```bash
sudo make install
```

## Project Structure

```
driftwm-settings/
├── src/
│   ├── main.rs          # Application entrypoint & Elm-style routing (iced 0.13)
│   ├── config.rs        # Driftwm 0.19+ schema & toml_edit lossless syncing
│   ├── theme.rs         # Catppuccin Mocha dark theme palette & widget styling
│   ├── validator.rs     # driftwm --check-config runner & syntax validator
│   └── views/
│       ├── mod.rs       # Tab navigation definitions
│       ├── general.rs   # Window placement, focus, mod_key & session
│       ├── appearance.rs# Decorations, gaps, borders, fonts & blur effects
│       ├── background.rs# Background mode, shaders, tiles & wallpapers
│       ├── bookmarks.rs # Bookmarks registry, anchors & pan dynamics
│       └── input.rs     # Keyboard, trackpad & mouse device settings
├── Cargo.toml
├── Makefile
└── driftwm-settings.desktop
```

## License

GPL-3.0-or-later (same as driftwm)
