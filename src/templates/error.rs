use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum TemplateError {
    Rendering(String),
    InvalidTemplate(String),
    IoError(std::io::Error),
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::Rendering(e) => write!(f, "Template rendering error: {}", e),
            TemplateError::InvalidTemplate(e) => write!(f, "Invalid template: {}", e),
            TemplateError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl StdError for TemplateError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            TemplateError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TemplateError {
    fn from(err: std::io::Error) -> Self {
        TemplateError::IoError(err)
    }
}

impl From<tera::Error> for TemplateError {
    fn from(err: tera::Error) -> Self {
        TemplateError::Rendering(err.to_string())
    }
}
