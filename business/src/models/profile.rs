use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub role: String,
    pub employer: String,
    pub description: String,
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0}\n{1}\n{2}\n{3}",
            self.name, self.role, self.employer, self.description
        )
    }
}
