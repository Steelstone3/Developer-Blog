use serde::{Deserialize, Serialize};

// TODO see if you can use an option of profile
#[derive(Deserialize, Serialize)]
pub struct UpdateProfile {
    pub name: Option<String>,
    pub role: Option<String>,
    pub employer: Option<String>,
    pub description: Option<String>,
}
