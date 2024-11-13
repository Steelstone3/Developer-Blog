use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlogPost {
    pub title: String,
    pub author: String,
    pub description: String,
    pub contents: String,
}

impl Display for BlogPost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0}\n{1}\n{2}\n{3}",
            self.title, self.author, self.description, self.contents
        )
    }
}
