use crate::{AccTokCrypt::{self, SecurityParam}, DirCheck::full_check_directory, tokinezed::{self, *}};
use std::fmt;
use validator::{Validate, ValidationError};
use std::error::Error;
use std::path::Path;

pub enum ParserError {
    NotFindInit(String),
    EmptyFile(String),
    NotSearchConfig(String),
    DoubleFind(String)
}

#[derive(Debug, Validate)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,
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

pub fn parse_string(toks: & TokenStruct,index: usize) -> Result<Token_String, String>
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
            if t=='\"' && !start_index_find
            {
                start_index=i;
                start_index_find=true;
            }
            else if t=='\"'
            {
                end_index=i;
            }
        }
        end_index!=0
    };
    for t in toks.tok_values.iter().cloned()
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
        tok_val: "".to_string()
    };

    tstr.tok_start=start_index;
    tstr.tok_end=end_index;
    tstr.tok_val=toks.tok_values[start_index].as_str()[start_index..end_index].to_string();
    

    Ok(tstr)
}

pub struct FindTokenResult
{
    index: usize,
    double_find: bool,
    double_find_index: usize,
}

pub enum IteratorType 
{
   None,
   Numeration,
   Time,
   CustomScript,
}

pub struct IteratorCommit
{
    msgpos: IteratorDecl,
    itr_type: IteratorType
}

pub struct IteratorDecl
{
    start: usize,
    end: usize
}

impl IteratorDecl
{
    fn is_decl(&self) -> bool
    {
        if self.start == self.end 
        {
            return false;
        }
        else 
        {
            return true;    
        }
    }
    
    fn set(&mut self,new_start: usize,new_end: usize) 
    {
        self.start=new_start;
        self.end=new_end;
    }
}

pub fn find_token(toks: &TokenStruct,skip_index: usize,token: Token) -> Result<FindTokenResult,ParserError>
{
    let mut fndt_res: FindTokenResult = FindTokenResult
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
            if t==token.as_str()
            {
                if !fndt_res.double_find
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
    if fndt_res.index!=0
    {
        result=Ok(fndt_res);
    }
    result     
}

macro_rules! generate_tok_parse {
    ($parse_type: ident , $token: path,$error: ty) => {
        pub fn $parse_type(toksref: &TokenStruct,index: usize) -> Result<FindTokenResult,$error>
        {    
            let result = find_token(toksref,index,$token);//ебаному расту похуй на неиспользования в макросах аргументов
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

//local username repository
//auto validation
pub fn parse_set_username(toksref: &TokenStruct,index: usize) -> Result<String,String>
{
   let mut username: String="".to_string();
   let parse_usrname=parse_username(toksref, index);
   if let Ok(findres) = parse_usrname
   {
        let index_find=findres.index;
        let parse_str: Result<Token_String,String> = parse_string(toksref,index_find);
        if let Ok(str_find) = parse_str
        {
            username=str_find.tok_val.to_string();
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
   if !username.chars().any(|c| 
        c.is_control() ||    
        c == '<' ||         
        c == '>' ||
        c == '"' ||          
        c == '\\' ||         
        c == '\0'            
    )
    {
        return Ok(username);
    }
    Err(format!("unresolved characters in username\t {}",username))
} 

//not validation
fn parse_set_email(toksref: &TokenStruct,index: usize) -> Result<String,String>
{
    let mut email: String="".to_string();
    let parse_email=parse_email(toksref, index);
    if let Ok(findres) = parse_email
    {
        let index_find=findres.index;
        let parse_str: Result<Token_String,String> = parse_string(toksref,index_find);
        if let Ok(str_find) = parse_str
        {
            email=str_find.tok_val.to_string();
        }
        else if let Err(e) = parse_str
        {
            return Err(e);
        }
    }
    else if let Err(e) = parse_email
    {
       let msg_err: String=e.to_string();
       return Err(msg_err);
    }
    if(email.is_empty())
    {
        email="Empty".to_string();
        return Err("email address is not specified".to_string());
    }
    Ok(email)   
}

//auto decrypt
pub fn parse_set_acctok(toksref: &TokenStruct,index: usize,sec_par: &SecurityParam) -> Result<String,String>
{
    let mut acctok: String="".to_string();
    let parse_acctok=parse_acctok(toksref, index);
    if let Ok(findres) = parse_acctok
    {
        let index_find=findres.index;
        let parse_str: Result<Token_String,String> = parse_string(toksref,index_find);
        if let Ok(str_find) = parse_str
        {
            acctok=str_find.tok_val.to_string();
            if(acctok.is_empty())
            {
                return Err("access token is not specified".to_string());
            }
            match AccTokCrypt::decrypt_acctok(acctok.as_bytes(), sec_par.clone())
            {
                Ok(val) => {
                    return Ok(val)
                }
                Err(_) => return Err("Parser: Decrypt Access token error".to_string().into())
            };
        }
        else if let Err(e) = parse_str
        {
            return Err(e);
        }
    }
    else if let Err(e) = parse_acctok
    {
       let msg_err: String=e.to_string();
       return Err(msg_err);
    }
    Ok(acctok) //ебаный раст не может понять что во всех ветках возврат значения
}

//auto validation
pub fn parse_set_path_obsidian(toksref: &TokenStruct,index: usize) -> Result<String,String>
{
    let mut obs_storage_path: String="".to_string();
    let parse_path=parse_path_obsidian(toksref, index);
    if let Ok(findres) = parse_path
    {
        let index_find=findres.index;
        let parse_str: Result<Token_String,String> = parse_string(toksref,index_find);
        if let Ok(str_find) = parse_str
        {
            obs_storage_path=str_find.tok_val.to_string();
            if obs_storage_path.is_empty()
            {
                return Err("obsidian path is not specified".to_string());
            }    
            let dir_check: Result<bool, String>=full_check_directory(obs_storage_path.clone());
            let res_check:bool = match dir_check
            {
                Ok(val) => {val},
                Err(error) => {
                    return Err(error);
                }
            };
            if !res_check
            {
                return Err("obsidian storage access denied".to_string());
            }
        }
        else if let Err(e) = parse_str
        {
            return Err(e);
        }
    }
    else if let Err(e) = parse_path
    {
       let msg_err: String=e.to_string();
       return Err(msg_err);
    }
    Ok(obs_storage_path)
}
//todo потдежка итератор типо cm1 cm2 ....
fn parse_set_text_commit(toksref: &TokenStruct,index: usize) -> Result<String,String>
{
   let mut commit_text: String="".to_string();
   let parse_usrname=parse_text_commit(toksref, index);
   if let Ok(findres) = parse_usrname
   {
        let index_find=findres.index;
        let parse_str: Result<Token_String,String> = parse_string(toksref,index_find);
        if let Ok(str_find) = parse_str
        {
            commit_text=str_find.tok_val.to_string();
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
   if !commit_text.chars().any(|c| 
        c.is_control() ||    
        c == '"' ||          
        c == '\\' ||         
        c == '\0'            
    )
    {
        return Ok(commit_text);
    }
    Err(format!("unresolved characters in username\t {}",commit_text))
}

pub fn parse_text_commit_iterator(str: String, index:usize) -> Option<Vec<IteratorCommit>>
{
    let mut iterators: Vec<IteratorCommit> = Vec::new();

    let mut find_decl = |str:String| -> Option<IteratorDecl>
    {
        let mut decl: IteratorDecl = IteratorDecl { start: 0, end: 0 };
        Some(decl)
    };

    let mut find_iterator = |str:String| -> Option<IteratorCommit>
    {
        let decl: IteratorDecl = IteratorDecl { start: 0, end: 0};
        let mut iterator: IteratorCommit = IteratorCommit { msgpos: decl, itr_type: IteratorType::None};
        for item in str.chars().enumerate().skip(index)
        {
            let (i, t): (usize, char) = item;
            
        }
        Some(iterator)
    };
    None
}