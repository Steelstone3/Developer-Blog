use std::fmt::{Display, Formatter, Result};

pub enum Path {
    Root,
    Profiles,
    ProfileId,
    Example,
}

impl Display for Path {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Path::Root => write!(formatter, "/"),
            Path::Profiles => write!(formatter, "/profiles"),
            Path::ProfileId => write!(formatter, "/profiles/:id"),
            Path::Example => write!(formatter, "/example"),
        }
    }
}
