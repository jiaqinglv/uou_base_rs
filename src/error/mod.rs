use std::fmt;
 
#[derive(Debug)]
pub struct Error {
    pub code: i32,
    message: String,
}
 
impl Error {
    pub fn new(code: i32, message: &str) -> Error {
        Error {
            code: code,
            message: message.to_string(),
        }
    }
}
 
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code:{} message:{}", self.code, self.message)
    }
}
 
impl std::error::Error for Error {}