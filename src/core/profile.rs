use crate::config::{Config, Profile};

pub fn save_profile(profile: Profile, config: &mut Config) -> std::io::Result<()> {
    let new_id = config.profiles.len() as u32;
    let new_profile = Profile {
        id: new_id,
        ..profile
    };

    config.profiles.push(new_profile);
    config.save()
}
