use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub ask_confirmation: bool,
    #[serde(default)]
    pub quiet: bool,
}

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".config");
    path.push("c2f");
    path.push("config.toml");
    path
}

pub fn load_config() -> Config {
    let config_path = get_config_path();

    if !config_path.exists() {
        // create config dir if not exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).ok();
        }

        // default config
        let default_config = Config::default();
        let toml_string = toml::to_string(&default_config).unwrap();
        fs::write(&config_path, toml_string).ok();

        return default_config;
    }

    let contents = fs::read_to_string(&config_path).unwrap_or_default();
    toml::from_str(&contents).unwrap_or_default()
}

pub fn parse_config_from_str(contents: &str) -> Config {
    toml::from_str(contents).unwrap_or_default()
}

pub fn determine_action(filename: &str, append: bool) -> &'static str {
    let exists = std::path::Path::new(filename).exists();
    if append {
        "append to"
    } else if exists {
        "overwrite"
    } else {
        "create"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.ask_confirmation, false);
        assert_eq!(config.quiet, false);
    }

    #[test]
    fn test_parse_config_from_str() {
        let toml_str = r#"
            ask_confirmation = true
            quiet = true
        "#;
        let config = parse_config_from_str(toml_str);
        assert_eq!(config.ask_confirmation, true);
        assert_eq!(config.quiet, true);
    }

    #[test]
    fn test_parse_partial_config() {
        let toml_str = r#"
            ask_confirmation = true
        "#;
        let config = parse_config_from_str(toml_str);
        assert_eq!(config.ask_confirmation, true);
        assert_eq!(config.quiet, false); // Should use default
    }

    #[test]
    fn test_parse_empty_config() {
        let config = parse_config_from_str("");
        assert_eq!(config, Config::default());
    }

    #[test]
    fn test_parse_invalid_config() {
        let config = parse_config_from_str("invalid toml {{{");
        assert_eq!(config, Config::default());
    }

    #[test]
    fn test_determine_action_create() {
        let action = determine_action("nonexistent_file.txt", false);
        assert_eq!(action, "create");
    }

    #[test]
    fn test_determine_action_append() {
        let action = determine_action("any_file.txt", true);
        assert_eq!(action, "append to");
    }

    #[test]
    fn test_determine_action_overwrite() {
        // Create a temp file that exists
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let action = determine_action(path, false);
        assert_eq!(action, "overwrite");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            ask_confirmation: true,
            quiet: true,
        };

        let toml_string = toml::to_string(&config).unwrap();
        let parsed_config: Config = toml::from_str(&toml_string).unwrap();

        assert_eq!(config, parsed_config);
    }

    #[test]
    fn test_config_path() {
        let path = get_config_path();
        assert!(path.to_str().unwrap().contains(".config/c2f/config.toml"));
    }
}
