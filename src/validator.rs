use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationStatus {
    Success { warnings: Vec<String> },
    Warning { warnings: Vec<String> },
    Error { message: String },
}

pub struct ConfigValidator;

impl ConfigValidator {
    /// Locate driftwm executable if available
    pub fn find_driftwm_binary() -> Option<PathBuf> {
        // 1. Check PATH
        if let Some(path_var) = std::env::var_os("PATH") {
            for dir in std::env::split_paths(&path_var) {
                let candidate = dir.join("driftwm");
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }

        // 2. Check common paths
        let candidates = [
            "/usr/local/bin/driftwm",
            "/usr/bin/driftwm",
        ];

        for c in &candidates {
            let p = PathBuf::from(c);
            if p.is_file() {
                return Some(p);
            }
        }

        // 3. Check dev repo target
        if let Some(home) = dirs::home_dir() {
            let release_p = home.join("driftwm/target/release/driftwm");
            if release_p.is_file() {
                return Some(release_p);
            }
            let debug_p = home.join("driftwm/target/debug/driftwm");
            if debug_p.is_file() {
                return Some(debug_p);
            }
        }

        None
    }

    /// Validate the config file at `path`, or validate raw toml string
    pub fn validate_file(path: &Path) -> ValidationStatus {
        if let Some(binary) = Self::find_driftwm_binary() {
            // Run: driftwm --config <path> --check-config
            let output = Command::new(&binary)
                .arg("--config")
                .arg(path)
                .arg("--check-config")
                .output();

            match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();

                    if out.status.success() {
                        let mut warnings = Vec::new();
                        for line in stdout.lines().chain(stderr.lines()) {
                            let line = line.trim();
                            if line.contains("warning") || line.contains("WARN") {
                                warnings.push(line.to_string());
                            }
                        }

                        if warnings.is_empty() {
                            ValidationStatus::Success { warnings: vec![] }
                        } else {
                            ValidationStatus::Warning { warnings }
                        }
                    } else {
                        let msg = if !stderr.is_empty() {
                            stderr
                        } else if !stdout.is_empty() {
                            stdout
                        } else {
                            "driftwm --check-config failed with non-zero status".to_string()
                        };
                        ValidationStatus::Error { message: msg }
                    }
                }
                Err(_e) => {
                    // Fall back to built-in validation
                    Self::validate_builtin(path)
                }
            }
        } else {
            // Built-in validation
            Self::validate_builtin(path)
        }
    }

    /// Built-in semantic & syntactic validation
    pub fn validate_builtin(path: &Path) -> ValidationStatus {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                return ValidationStatus::Error {
                    message: format!("Cannot read file {}: {}", path.display(), e),
                };
            }
        };

        // 1. Check TOML syntax
        let toml_val: toml::Value = match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                return ValidationStatus::Error {
                    message: format!("TOML syntax error: {}", e),
                };
            }
        };

        let mut warnings = Vec::new();

        // 2. Validate known key types & bounds
        if let Some(table) = toml_val.as_table() {
            if let Some(mk) = table.get("mod_key").and_then(|v| v.as_str())
                && !["super", "alt", "mod3"].contains(&mk)
            {
                warnings.push(format!("Unknown mod_key '{}': expected 'super', 'alt', or 'mod3'", mk));
            }

            if let Some(wp) = table.get("window_placement").and_then(|v| v.as_str())
                && !["center", "cursor", "auto"].contains(&wp)
            {
                warnings.push(format!("Unknown window_placement '{}': expected 'center', 'cursor', or 'auto'", wp));
            }

            if let Some(fp) = table.get("focus_placement").and_then(|v| v.as_str()) {
                let valid_fp = [
                    "center", "top", "bottom", "left", "right",
                    "top-left", "top-right", "bottom-left", "bottom-right",
                ];
                if !valid_fp.contains(&fp) {
                    warnings.push(format!("Unknown focus_placement '{}'", fp));
                }
            }

            if let Some(bg) = table.get("background").and_then(|v| v.as_table())
                && let Some(kind) = bg.get("type").and_then(|v| v.as_str())
            {
                let valid_bg = ["default", "shader", "tile", "wallpaper", "none"];
                if !valid_bg.contains(&kind) {
                    warnings.push(format!("Unknown [background] type '{}'", kind));
                }
                if matches!(kind, "shader" | "tile" | "wallpaper") && !bg.contains_key("path") {
                    warnings.push(format!("[background] type = '{}' requires a 'path' field", kind));
                }
            }

            if let Some(deco) = table.get("decorations").and_then(|v| v.as_table())
                && let Some(dm) = deco.get("default_mode").and_then(|v| v.as_str())
                && !["client", "minimal", "none"].contains(&dm)
            {
                warnings.push(format!("Unknown decorations.default_mode '{}': expected 'client', 'minimal', or 'none'", dm));
            }
        }

        if warnings.is_empty() {
            ValidationStatus::Success { warnings: vec![] }
        } else {
            ValidationStatus::Warning { warnings }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_default_config() {
        let path = std::env::temp_dir().join("driftwm_test_valid.toml");
        let content = crate::config::DriftwmConfig::create_default_document().to_string();
        std::fs::write(&path, content).unwrap();

        let status = ConfigValidator::validate_builtin(&path);
        let _ = std::fs::remove_file(&path);
        assert_eq!(status, ValidationStatus::Success { warnings: vec![] });
    }

    #[test]
    fn test_syntax_error_reported() {
        let path = std::env::temp_dir().join("driftwm_test_bad_syntax.toml");
        std::fs::write(&path, b"mod_key = \n [broken").unwrap();

        let status = ConfigValidator::validate_builtin(&path);
        let _ = std::fs::remove_file(&path);
        match status {
            ValidationStatus::Error { message } => {
                assert!(message.contains("TOML syntax error"));
            }
            _ => panic!("Expected syntax error"),
        }
    }

    #[test]
    fn test_warning_for_invalid_mod_key() {
        let path = std::env::temp_dir().join("driftwm_test_bad_mod.toml");
        std::fs::write(&path, b"mod_key = \"hyper\"\n").unwrap();

        let status = ConfigValidator::validate_builtin(&path);
        let _ = std::fs::remove_file(&path);
        match status {
            ValidationStatus::Warning { warnings } => {
                assert!(warnings.iter().any(|w| w.contains("Unknown mod_key 'hyper'")));
            }
            _ => panic!("Expected warning for invalid mod_key"),
        }
    }
}
