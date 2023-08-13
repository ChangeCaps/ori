use std::fmt::Display;

use super::ArcStr;

#[derive(Debug)]
pub enum ThemeError {
    NotFound(ArcStr),
    MaxDepth,
    Downcast,
}

impl Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeError::NotFound(name) => write!(f, "missing theme value for key '{}'", name),
            ThemeError::MaxDepth => write!(f, "maximum theme depth exceeded"),
            ThemeError::Downcast => write!(f, "failed to downcast theme value"),
        }
    }
}
