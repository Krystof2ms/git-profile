use crate::{
    changes::{Change, Operation},
    config::{Level, Profile},
};

pub fn set_config(
    profile: &Profile,
    level: Level,
) -> Vec<Change> {
    let mut operations = vec![];

    match level {
        Level::Session => {
            operations.push(Change {
                level: Level::Session,
                operation: Operation::SetEnv {
                    key: "GIT_AUTHOR_NAME".into(),
                    value: profile.name.clone(),
                },
            });
            operations.push(Change {
                level: Level::Session,
                operation: Operation::SetEnv {
                    key: "GIT_AUTHOR_EMAIL".into(),
                    value: profile.email.clone(),
                },
            });
            operations.push(Change {
                level: Level::Session,
                operation: Operation::SetEnv {
                    key: "GIT_COMMITTER_NAME".into(),
                    value: profile.name.clone(),
                },
            });
            operations.push(Change {
                level: Level::Session,
                operation: Operation::SetEnv {
                    key: "GIT_COMMITTER_EMAIL".into(),
                    value: profile.email.clone(),
                },
            });
        }
        other => {
            operations.push(Change {
                level: other,
                operation: Operation::SetGitConfig {
                    key: "user.name".into(),
                    value: profile.name.clone(),
                },
            });
            operations.push(Change {
                level: other,
                operation: Operation::SetGitConfig {
                    key: "user.email".into(),
                    value: profile.email.clone(),
                },
            });
        }
    };
    operations
}
