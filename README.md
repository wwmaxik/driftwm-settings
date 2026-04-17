# driftwm-settings

Complete GUI settings manager for [driftwm](https://github.com/malbiruk/driftwm) compositor.

![CI](https://github.com/wwmaxik/driftwm-settings/workflows/CI/badge.svg)
![Version](https://img.shields.io/badge/version-0.2.0-blue)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)
![GTK](https://img.shields.io/badge/GTK-4-green)
![License](https://img.shields.io/badge/license-GPLv3-blue)

## Features

- 🎨 **Native GTK4 interface** with sidebar navigation
- ⚙️ **All driftwm settings** - 15+ configuration pages covering every option
- 💾 **Live editing** - changes update config structure in real-time
- 🔄 **Hot reload support** - driftwm automatically reloads config on save
- 📝 **TOML format** - reads/writes `~/.config/driftwm/config.toml`
- 🎭 **Interactive Shader Editor** - create custom animated backgrounds with visual controls
- 🪟 **Window Rules** - per-app blur, opacity, and decoration settings

## Settings Pages

### 1. General
- Modifier key (super/alt)
- Focus follows mouse

### 2. Keyboard
- Layout, variant, options, model
- Repeat rate and delay
- Layout independent keybindings
- Num Lock and Caps Lock startup state

### 3. Trackpad
- Tap to click, natural scroll, tap and drag
- Acceleration speed and profile
- Click method

### 4. Mouse
- Acceleration speed and profile
- Natural scroll

### 5. Cursor
- Theme and size
- Inactive opacity

### 6. Navigation
- Trackpad and mouse speed
- Friction and animation speed
- Nudge and pan step
- Edge pan settings

### 7. Zoom
- Zoom step multiplier
- Fit padding
- Reset on new window/activation

### 8. Snap
- Enable/disable snapping
- Gap, distance, break force
- Same edge snapping

### 9. Decorations
- Background and foreground colors
- Corner radius

### 10. Effects
- Blur radius (0-20 passes) and strength (0.1-5.0)
- 6 blur presets: None, Light, Default, Medium, Strong, Extreme

### 11. Window Rules
- Per-app blur, opacity, decoration settings
- Match by app_id and/or title (glob support)
- Widget mode (pinned windows)
- Dynamic add/remove rules

### 12. Backend
- Wait for frame completion
- Disable direct scanout
- NVIDIA environment variables guide

### 13. Background
- Shader path (GLSL)
- Tile path (PNG/JPG)
- Quick access to Shader Editor

### 14. Shader Editor ✨ NEW
- **Visual Mode**: Interactive controls for colors, animation, effects
  - 3 shader templates: Gradient, Animated Waves, Clouds
  - RGB color pickers for primary and secondary colors
  - Animation speed, pattern scale, complexity controls
  - Vignette and glow effects
- **Raw Mode**: Full GLSL code editor for advanced users
- Generate and save custom shaders
- Apply directly to background

### 15. Keybindings
- Custom keyboard shortcuts
- Add/remove bindings dynamically

### 16. Autostart
- Multi-line editor for startup commands

## Installation

### Requirements

- GTK4
- Rust 1.85+ (edition 2024)

### Build from source

```bash
cd ~/driftwmsettings
cargo build --release
```

### Install system-wide

```bash
sudo make install
```

This installs:
- Binary to `/usr/local/bin/driftwm-settings`
- Desktop entry to `/usr/local/share/applications/`

### Uninstall

```bash
sudo make uninstall
```

## Usage

### Launch from terminal

```bash
driftwm-settings
```

### Launch from application menu

Search for "driftwm Settings" in your application launcher.

### Development

```bash
cargo run
```

## Project Structure

```
driftwmsettings/
├── src/
│   ├── main.rs           # UI and page implementations
│   ├── config.rs         # TOML config structures
│   ├── config_helpers.rs # Config initialization helpers
│   ├── ui_helpers.rs     # UI widget helpers
│   └── shader_editor.rs  # Interactive shader editor
├── Cargo.toml
├── Makefile
├── README.md
└── driftwm-settings.desktop
```

## Configuration

The app reads and writes to:
```
~/.config/driftwm/config.toml
```

Changes are saved when you click the "Save" button. driftwm will automatically reload the config within 1 second (hot reload).

Custom shaders are saved to:
```
~/.config/driftwm/custom_shader.glsl
```

## Recent Changes (v0.2.0)

### New Features
- ✨ **Interactive Shader Editor** with Visual/Raw modes
- 🪟 **Window Rules** page for per-app settings
- 🎨 **Enhanced blur controls** with 6 presets
- 📏 **Better window sizing** - reduced default size to 900x650
- 🔄 **Scroll reset** - pages always start from top when switching
- 🔤 **Clearer labels** - "Layout independent keybindings" with tooltip

### Config Updates
- Added `num_lock` and `caps_lock` keyboard settings
- Added `reset_on_new_window` and `reset_on_activation` zoom settings
- Removed `force_legacy_drm` (now uses `SMITHAY_USE_LEGACY=1` env var)

### Bug Fixes
- Fixed scroll state persisting between sections (#1)
- Fixed default layout being too wide (#2)
- Improved "Layout independent" label clarity (#3)

## Screenshots

*(Coming soon)*

## Development

### Adding new settings

1. Add field to appropriate struct in `src/config.rs`
2. Add UI widget in corresponding `add_*_page()` function in `src/main.rs`
3. Connect widget signal to update config

### Code style

- Use `create_row()` for horizontal layouts
- Use `add_label()` for consistent label width
- Use `ensure_*()` helpers to initialize nested config structs
- Follow existing patterns for Switch, Entry, and SpinButton widgets

## License

GPL-3.0-or-later (same as driftwm)

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

- Fork the repository
- Create a feature branch
- Make your changes
- Run tests: `cargo fmt`, `cargo clippy`, `cargo build`
- Submit a pull request

See also: [Code of Conduct](CODE_OF_CONDUCT.md)

For upstream driftwm, see [malbiruk/driftwm](https://github.com/malbiruk/driftwm).

## Credits

- Built for [driftwm](https://github.com/malbiruk/driftwm) by malbiruk
- GUI implementation by wwmaxik
