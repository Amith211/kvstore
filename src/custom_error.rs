use std::{error::Error, fmt, ops::Deref};

#[derive(Debug)]
pub struct CustomError {
    source: Option<Box<dyn Error>>
}

impl CustomError {
    pub fn new() -> Result<CustomError, CustomError> {
        Ok(CustomError { source: None })
    }
}

// impl From<Box<dyn Error>> for CustomError {
//     fn from(error: Box<dyn Error>) -> Self {
//         CustomError {source: Some(error) }
//     }
// }

impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError {source: Some(Box::new(error))}
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(x) => Some(x.deref()),
            None => None,            
        }
    }
}