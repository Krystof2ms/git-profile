use crate::config::Level;

pub struct Change {
    pub level: Level,
    pub operation: Operation,
}

pub enum Operation {
    SetGitConfig {
        key: String,
        value: String,
    },
    UnsetGitConfig {
        key: String,
    },
    SetEnv {
        key: String,
        value: String,
    },
    UnsetEnv {
        key: String,
    },
}
