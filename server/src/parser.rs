/**
## pub
```
PUB <subject> <size>\r\n
<message>\r\n
```
## sub
```
SUB <subject> <sid>\r\n
SUB <subject> <queue> <sid>\r\n
```
## MSG
```
MSG <subject> <sid> <size>\r\n
<message>\r\n
```
*/

use crate::error::*;
pub enum ParseState {
    OpStart,
    OpS,
    OpSu,
    OpSub,
    OpSubSpace,
    OpSubArg,
    OpP,
    OpPu,
    OpPub,
    OpPubSpace,
    OpPubArg,
    OpMsg
}
use ParseState::*;
pub struct Parser{
    state:ParseState,
    buf:[u8;512],//消息缓冲区
    arg_len:usize,
    msg_buf:Option<Vec<u8>>,
}

struct SubArg<'a>{
    subject:&'a str,
    sid:&'a str,
    queue:Option<&'a str>,
}
struct PubArg<'a>{
    subject:&'a str,
    size_buf:&'a str,//1024字符串形式
    size:i64,//1024 整数形式
    msg:&'a [u8],
}
pub enum ParseResult<'a> {
    NoMsg,
    Sub(SubArg<'a>),
    Pub(PubArg<'a>),
}

impl Parser {
    pub fn new()->Self{
        Self { state: ParseState::OpStart, buf: [0;512], arg_len: 0,msg_buf:None }
    }
    /**
     * 对收到的字节序列进行解析，解析完后得到pub或sub
     * usize：position处理到的位置
     */
    pub fn parse(&mut self,buf:&[u8])->Result<(ParseResult,usize)>{
        let mut b;
        let mut i:usize = 0;
        while i<buf.len(){
            b = buf[i] as char;
            match  self.state{
                OpStart=>{
                    match b {
                        'S'=>{self.state=OpS},
                        'P'=>{self.state=OpP},
                        _ =>{return Err(NError::new(ERROR_PARSE));}
                    }
                },
                OpS=>{
                    match b {
                        'U'=>{self.state=OpSu},
                        _ =>{return Err(NError::new(ERROR_PARSE));}
                    }
                },
                OpSu=>{
                    match b {
                        'B'=>{self.state=OpSub},
                        _ =>{return Err(NError::new(ERROR_PARSE));}
                    }
                },
                OpSub=>{
                    match b {
                        ' '|'\t'=>{self.state=OpSubSpace},
                        _ =>{return Err(NError::new(ERROR_PARSE));}
                    }
                },
                OpSubSpace=>{
                    match b {
                      ' '|'\t'=>{},
                      _=>{
                            self.state=OpSubArg;
                            self.arg_len = 0;
                            continue;
                        },  
                    }
                },
                OpSubArg=>{
                    match b{
                        '\r'=>{}
                        '\n'=>{
                            self.state = OpStart;
                            let r = self.process_sub()?;
                            return Ok((r,i+1));
                        }
                        _=>{
                            self.add_arg(b as u8)?;
                        }
                    }
                },
                OpP=>{
                    match b{
                        'U'=>{self.state=OpPu},
                        _=>return Err(NError::new(ERROR_PARSE))      
                    }        
                }
                OpPu=>{
                    match b{
                        'B'=>{self.state=OpPub},
                        _=>return Err(NError::new(ERROR_PARSE))      
                    }        
                },
                OpPub=>{
                    match b {
                        ' '|'\t'=>{self.state=OpPubSpace;},
                        _=>return Err(NError::new(ERROR_PARSE))
                    }
                },
                OpPubSpace=>{
                    match b {
                        ' '|'\t'=>{},
                        _=>{
                            self.state= OpPubArg;
                            self.arg_len = 0;
                            continue;
                        }
                    }
                },
                OpPubArg=>{
                    match b {
                        '\r'=>{},
                        '\n'=>{
                            self.state=OpMsg;
                        },
                        _=>{
                            self.add_arg(b as u8)?;
                        }
                    }
                }
                _=>{}
            }
        }
        Ok((ParseResult::NoMsg, buf.len()))
    }

    fn process_sub(&self)->Result<ParseResult>{
        Err(NError::new(ERROR_PARSE))
    }
    fn add_arg(&mut self,b:u8)->Result<()>{
        if self.arg_len>=self.buf.len(){
            return Err(NError::new(ERROR_PARSE));
        }
        self.buf[self.arg_len] = b;
        self.arg_len+=1;
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test(){}
}