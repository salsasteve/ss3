use std::convert::TryFrom;
use std::error::Error;
use super::method::Method;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}


impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message()) 
    }
    
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message()) 
    }
    
}
    
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
    

impl Error for ParseError {
    
}

