use std::fmt;

#[derive(Debug, Clone)]
pub struct StringConversionRPSError;

impl fmt::Display for StringConversionRPSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid string to parse into Rock, Paper or Scissors")
    }
}
