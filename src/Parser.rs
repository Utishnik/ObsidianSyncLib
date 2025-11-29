use crate::tokinezed::{self, *};
use std::fmt;

pub enum ParserError {
    NotFindInit(String),
    EmptyFile(String),
    NotSearchConfig(String),
    DoubleFind(String)
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            ParserError::NotFindInit(msg) => msg,
            ParserError::EmptyFile(msg) => msg,
            ParserError::NotSearchConfig(msg) => msg,
            ParserError::DoubleFind(msg) => msg,
        };
        write!(f, "{}", message)
    }
}

//leak tok_val, please drop
pub fn parse_string<'a>(toks: &'a TokenStruct,index: usize) -> Result<Token_String<'a>, String>
{
    //let toks_len:usize=toks.get_size();
    let mut start_index : usize=0;
    let mut start_index_find : bool=false;
    let mut end_index : usize=0;

    let mut find_str = |str:String| -> bool
    {
        for item in str.chars().enumerate().skip(index)
        {
            let (i, t): (usize, char) = item;
            if (t=='\"' && !start_index_find)
            {
                start_index=i;
                start_index_find=true;
            }
            else if(t=='\"')
            {
                end_index=i;
            }
        }
        end_index!=0
    };
    for t in toks.tok_values.to_vec()
    {
        if find_str(t)
        {
            break;
        }
    }

    if end_index==0
    {
        return Err("not find end \"".to_string());
    }

    let mut tstr = Token_String {
        tok_start: start_index,
        tok_end: end_index,
        tok_val: Box::leak(Box::new(String::new())) 
    };
    let slice: &str = toks.tok_values[start_index].as_str();

    tstr.tok_start=start_index;
    tstr.tok_end=end_index;
    tstr.tok_val=&slice[start_index..end_index];
    

    Ok(tstr)
}

pub struct FindTokenResult
{
    index: usize,
    double_find: bool,
    double_find_index: usize,
}

pub fn find_token(toks: &TokenStruct,skip_index: usize,token: Token) -> Result<FindTokenResult,ParserError>
{
    let mut fndt_res = FindTokenResult
    {
        index: 0,
        double_find: false,
        double_find_index: 0,
    };
    let mut result: Result<FindTokenResult, ParserError>=Err(ParserError::NotFindInit(format!("Not {} Init",token.as_str())));
    'leave:
    for item in toks.tok_values.to_vec().iter().enumerate().skip(skip_index)
    {
            let (i, t): (usize, &String) = item;
            if(t==token.as_str())
            {
                if(!fndt_res.double_find)
                {
                    fndt_res.index=i;
                    fndt_res.double_find=true;
                }
                else 
                {
                    fndt_res.double_find_index=i;
                    result=Err(ParserError::DoubleFind(("Double find colon: ".to_owned() + &i.to_string()).to_string()));
                    break 'leave;
                }
            }
    }
    result=Ok(fndt_res);
    result     
}

macro_rules! generate_tok_parse {
    ($parse_type: ident , $token: path,$error: ty) => {
        pub fn $parse_type(toksref: &TokenStruct,index: usize) -> Result<FindTokenResult,$error>
        {    
            let result = find_token(toksref,index,Token::UserName);
            result
        }
    };
}

generate_tok_parse!(parse_email, Token::Email, ParserError);
generate_tok_parse!(parse_acctok, Token::AccTok, ParserError);
generate_tok_parse!(parse_remote_rep_addr, Token::RemoteRepAddr, ParserError);
generate_tok_parse!(parse_set_val, Token::SetVal, ParserError);
generate_tok_parse!(parse_path_obsidian, Token::PathObsidian, ParserError);
generate_tok_parse!(parse_time_commit, Token::TimeCommit, ParserError);
generate_tok_parse!(parse_text_commit, Token::TextCommit, ParserError);
generate_tok_parse!(parse_username, Token::UserName, ParserError);

pub fn parse_set_username(toksref: &TokenStruct,index: usize) -> Result<String,String>
{
   let mut result: String="".to_string();
   let parse_usrname=parse_username(toksref, index);
   if let Ok(findres) = parse_usrname
   {
        let index_find=findres.index;
        let parse_str: Result<Token_String,String> = parse_string(toksref,index_find);
        if let Ok(str_find) = parse_str
        {
            result=str_find.tok_val.to_string();
        }
        else if let Err(e) = parse_str
        {
            return Err(e);
        }
   }
   else if let Err(e) = parse_usrname
   {
       let msg_err: String=e.to_string();
       return Err(msg_err);
   }
   Ok(result)
} 
