use crate::{
    changes::{Change, Operation},
    config::Level,
};

pub fn set_config(ssh_key: String, level: Level, operations: &mut Vec<Change>) {
    match level {
        Level::Session => {
            operations.push(Change {
                level: Level::Session,
                operation: Operation::SetGitConfig {
                    key: "GIT_SSH_COMMAND".into(),
                    value: ssh_key,
                },
            });
        }
        other => {
            operations.push(Change {
                level: other,
                operation: Operation::SetGitConfig {
                    key: "GIT_SSH_COMMAND".into(),
                    value: ssh_key,
                },
            });
        }
    }
}
