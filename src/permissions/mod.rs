pub mod constraint;
pub mod overrides;

use overrides::Overrides;

#[derive(Default, Debug)]
pub enum PermissionEntity {
    // if the default value is ever used for PermissionEntity, Global will be the default
    #[default]
    Global,

    Category(String),
    Role(String),
    Channel(String),
}

#[derive(Debug)]
pub struct Permissions {
    pub roles: Overrides,
    pub channels: Overrides,
}

impl Default for Permissions {
    fn default() -> Self {
        Permissions {
            roles: Overrides {
                allowed: vec![PermissionEntity::Global],
                denied: vec![],
            },
            channels: Overrides {
                allowed: vec![PermissionEntity::Global],
                denied: vec![],
            },
        }
    }
}

// impl Into<T> means ANY type that implements the Into trait for T, which needs to be implemented for .into() to work
pub fn channel(channel_name: impl Into<String>) -> PermissionEntity {
    PermissionEntity::Channel(channel_name.into())
}

pub fn category(category_name: impl Into<String>) -> PermissionEntity {
    PermissionEntity::Category(category_name.into())
}

pub fn role(role_name: impl Into<String>) -> PermissionEntity {
    PermissionEntity::Role(role_name.into())
}

pub fn everyone() -> PermissionEntity {
    PermissionEntity::Global
}

pub fn all_channels() -> PermissionEntity {
    PermissionEntity::Global
}

pub fn nobody() -> Overrides {
    Overrides::empty()
}

pub fn noone() -> Overrides {
    Overrides::empty()
}

pub fn nowhere() -> Overrides {
    Overrides::empty()
}

pub fn all_channels_except(channels: Vec<PermissionEntity>) -> Overrides {
    Overrides {
        allowed: vec![all_channels()],
        denied: channels,
    }
}

pub fn everyone_except(roles: Vec<PermissionEntity>) -> Overrides {
    Overrides {
        allowed: vec![everyone()],
        denied: roles,
    }
}

pub fn only_these_channels(channels: Vec<PermissionEntity>) -> Overrides {
    Overrides {
        allowed: channels,
        denied: vec![all_channels()],
    }
}

pub fn only_these_roles(roles: Vec<PermissionEntity>) -> Overrides {
    Overrides {
        allowed: roles,
        denied: vec![everyone()],
    }
}
