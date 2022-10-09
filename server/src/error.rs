use std::fmt::{Display};
use std::error::Error;
pub type Result<T> = std::result::Result<T,NError>;
pub const ERROR_PARSE:i32 = 1;

#[derive(Debug)]
pub struct NError{
    err_code:i32,
}
impl NError {
    pub fn new(code:i32)->Self{
        Self { err_code: code }
    }
    pub fn description(&self)->&'static str{
        match self.err_code {
            ERROR_PARSE=>"parse error",
            _=>"unknow"
        }
    }
}
impl Error for NError {}
impl Display for NError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"NError[{},{}]",self.err_code,self.description())
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test(){
        print!("{}",NError::new(ERROR_PARSE))
    }
}