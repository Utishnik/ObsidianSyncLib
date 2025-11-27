use crate::tokinezed::{self, *};

pub fn Parse_String(toks: &TokenStruct,index: usize) -> Result<Token_String, std::string::String>
{
    Ok(toks);
}

pub fn Parse_UserName(toksref: &TokenStruct,index:usize) -> Result<String,String>
{
    if(toksref.to_vec()[index]==Token::as_str(&Token::SetVal))
    {
        if let result=Parse_String(toksref,index+1) 
        {

        }
    }
    else {
        Err(" \'=\' not find".to_string())// надо еще строчку ошибки писать
    }
} 
