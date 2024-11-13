use std::fmt::{Display, Formatter, Result};

pub enum Path {
    Root,
    Blogs,
    BlogId,
    Example,
}

impl Display for Path {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Path::Root => write!(formatter, "/"),
            Path::Blogs => write!(formatter, "/blogs"),
            Path::BlogId => write!(formatter, "/blog/:id"),
            Path::Example => write!(formatter, "/example"),
        }
    }
}
