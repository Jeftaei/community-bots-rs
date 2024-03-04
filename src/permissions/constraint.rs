// use super::Rule;

// pub struct Constraints {
//     rules: Vec<Rule>,
// }
// TODO: Make builders, conversions and eventually the macro that will make this easier to use

use super::{overrides::Overrides, PermissionEntity};

#[derive(Default, Debug)]
pub struct RateLimit {
    window: u32,
    max: u32,
}

#[derive(Default, Debug)]
pub struct Rule {
    roles: Overrides,
    ratelimit: Option<RateLimit>,
}

#[derive(Default, Debug)]
pub struct Constraint {
    rules: Vec<Rule>,
    enforce_in_bot_channels: bool,
}
