use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyError(String);

impl MyError {
    // usage: return Err(MyError::new("some error".into()).into())
    pub fn new(s: String) -> MyError {
        MyError(s)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for MyError {}