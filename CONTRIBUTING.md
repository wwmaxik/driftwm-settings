# Contributing to driftwm-settings

Thank you for your interest in contributing to driftwm-settings! This document provides guidelines and instructions for contributing.

## Getting Started

### Prerequisites

- Rust 1.85+ (edition 2024)
- GTK4 development libraries
- Git

### Setting Up Development Environment

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/driftwm-settings.git
   cd driftwm-settings
   ```

3. Install dependencies:
   
   **Fedora:**
   ```bash
   sudo dnf install rust cargo gtk4-devel gcc
   ```
   
   **Ubuntu/Debian:**
   ```bash
   sudo apt install libgtk-4-dev build-essential rustc cargo
   ```
   
   **Arch Linux:**
   ```bash
   sudo pacman -S rust gtk4 base-devel
   ```

4. Build and run:
   ```bash
   cargo build
   cargo run
   ```

## Development Workflow

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy before committing: `cargo clippy`
- Write self-documenting code with clear names
- Keep functions small and focused

### Project Structure

```
driftwm-settings/
├── src/
│   ├── main.rs           # UI implementation and page builders
│   ├── config.rs         # TOML config structures
│   ├── config_helpers.rs # Config initialization helpers
│   └── ui_helpers.rs     # UI widget helpers
├── style.css             # Minimal CSS styling
├── driftwm-settings.svg  # Application icon
└── driftwm-settings.desktop
```

### Making Changes

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the code style

3. Test your changes:
   ```bash
   cargo build
   cargo clippy
   cargo fmt
   cargo run
   ```

4. Commit with a clear message:
   ```bash
   git commit -m "Add feature: description of what you added"
   ```

5. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

6. Open a Pull Request on GitHub

### Commit Message Guidelines

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- First line should be 50 characters or less
- Reference issues and pull requests when relevant

Examples:
```
Add support for custom keyboard layouts
Fix crash when config file is missing
Update README with installation instructions
```

## Adding New Settings

To add a new setting to the UI:

1. Add the field to the appropriate struct in `src/config.rs`
2. Add UI widget in the corresponding `add_*_page()` function in `src/main.rs`
3. Connect the widget signal to update the config
4. Use helper functions from `ui_helpers.rs` for consistency

Example:
```rust
// In add_general_page()
let setting_box = create_row();
add_label(&setting_box, "Setting name:", 200);

let setting_switch = Switch::new();
setting_switch.set_active(config.borrow().your_setting.unwrap_or(false));

let config_clone = config.clone();
setting_switch.connect_state_set(move |_, state| {
    config_clone.borrow_mut().your_setting = Some(state);
    gtk4::glib::Propagation::Proceed
});

setting_box.append(&setting_switch);
page.append(&setting_box);
```

## Testing

Currently, the project focuses on manual testing:

1. Build and run the application
2. Test all settings pages
3. Verify config file is written correctly to `~/.config/driftwm/config.toml`
4. Check that driftwm reloads the config (hot reload)

## Continuous Integration

All pull requests are automatically tested via GitHub Actions:

- Code formatting check (`cargo fmt`)
- Linting with clippy (`cargo clippy`)
- Build on Ubuntu and Fedora
- Run tests (`cargo test`)

Make sure your code passes all checks before submitting a PR.

## Reporting Issues

When reporting issues, please include:

- Operating system and version
- GTK4 version
- Rust version (`rustc --version`)
- Steps to reproduce the issue
- Expected vs actual behavior
- Relevant logs or error messages

## Questions?

Feel free to open an issue for questions or discussions about contributing.

## License

By contributing to driftwm-settings, you agree that your contributions will be licensed under the GPLv3 license.
