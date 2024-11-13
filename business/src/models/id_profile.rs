use super::profile::Profile;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct IdProfile {
    pub id: usize,
    #[serde(flatten)]
    pub profile: Profile,
}

impl IdProfile {
    pub fn new(id: usize, profile: Profile) -> IdProfile {
        IdProfile { id, profile }
    }
}
