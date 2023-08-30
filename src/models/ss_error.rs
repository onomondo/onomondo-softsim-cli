use std::fmt;
#[derive(Debug)]
pub struct GenericError {
    pub message: String,
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SS_Cli generic error: {}", self.message)
    }
}

impl std::error::Error for GenericError {}

impl GenericError {
    pub fn new(message: String) -> GenericError {
        GenericError { message }
    }
}
