# driftwm-settings

GUI settings manager for [driftwm](https://github.com/malbiruk/driftwm) compositor.

## Features

- 🎨 Native GTK4 interface
- ⚙️ Edit common driftwm settings without touching TOML
- 💾 Reads and writes `~/.config/driftwm/config.toml`
- 🔄 Hot reload support (driftwm automatically reloads config)

## Settings Supported

- **Modifier Key**: Choose between `super` or `alt`
- **Focus**: Focus follows mouse toggle
- **Trackpad**: Tap to click, natural scroll
- **Cursor**: Theme customization

## Installation

### Build from source

```bash
cargo build --release
sudo cp target/release/driftwm-settings /usr/local/bin/
```

### Run

```bash
driftwm-settings
```

## Requirements

- GTK4
- Rust 1.85+ (edition 2024)

## Development

```bash
cargo run
```

## License

Same as driftwm (GPL-3.0-or-later)
