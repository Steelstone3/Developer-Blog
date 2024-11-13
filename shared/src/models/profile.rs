use serde::Deserialize;

#[derive(Deserialize)]
pub struct Profile {
    pub name: Option<String>,
    pub description: Option<String>,
}
