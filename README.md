# driftwm-settings

Complete GUI settings manager for [driftwm](https://github.com/malbiruk/driftwm) compositor.

![Version](https://img.shields.io/badge/version-1.0.0-blue)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)
![GTK](https://img.shields.io/badge/GTK-4-green)

## Features

- 🎨 **Native GTK4 interface** with sidebar navigation
- ⚙️ **All driftwm settings** - 13 configuration pages covering every option
- 💾 **Live editing** - changes update config structure in real-time
- 🔄 **Hot reload support** - driftwm automatically reloads config on save
- 📝 **TOML format** - reads/writes `~/.config/driftwm/config.toml`

## Settings Pages

### 1. General
- Modifier key (super/alt)
- Focus follows mouse

### 2. Keyboard
- Layout, variant, options, model
- Repeat rate and delay
- Layout independent bindings

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

### 7. Zoom
- Zoom step multiplier
- Fit padding

### 8. Snap
- Enable/disable snapping
- Gap, distance, break force
- Same edge snapping

### 9. Decorations
- Background and foreground colors
- Corner radius

### 10. Effects
- Blur radius and strength

### 11. Background
- Shader path (GLSL)
- Tile path (PNG/JPG)

### 12. Autostart
- Multi-line editor for startup commands

### 13. Output
- (Reserved for future multi-monitor settings)

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
│   ├── main.rs           # UI and page implementations (1343 lines)
│   ├── config.rs         # TOML config structures (259 lines)
│   ├── config_helpers.rs # Config initialization helpers (41 lines)
│   └── ui_helpers.rs     # UI widget helpers (47 lines)
├── Cargo.toml
├── Makefile
├── README.md
└── driftwm-settings.desktop
```

**Total**: 1690 lines of Rust code

## Configuration

The app reads and writes to:
```
~/.config/driftwm/config.toml
```

Changes are saved when you click the "Save Configuration" button. driftwm will automatically reload the config within 1 second (hot reload).

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

This is a personal fork. For upstream driftwm, see [malbiruk/driftwm](https://github.com/malbiruk/driftwm).

## Credits

- Built for [driftwm](https://github.com/malbiruk/driftwm) by malbiruk
- GUI implementation by wwmaxik
