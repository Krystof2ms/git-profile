#[cfg(not(debug_assertions))]
pub const CONFIG_FILE: &str = "~/.config/git-profile/config.toml";

#[cfg(debug_assertions)]
pub const CONFIG_FILE: &str = "./tests/config.toml";
