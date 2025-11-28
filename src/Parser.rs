use crate::tokinezed::{self, *};

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
    

    return Ok(tstr);
}

pub fn parse_username(toksref: &TokenStruct,index:usize) -> Result<String,String>
{
    if(toksref.tok_values.to_vec()[index]==Token::as_str(&Token::SetVal))
    {
        if let Ok(result)=parse_string(toksref,index+1) 
        {
            Ok(result.tok_val.to_string())
        }
        else 
        {
            Err("Parsing fail".to_string())    
        }
    }
    else {
        Err(" \'=\' not find".to_string())// надо еще строчку ошибки писать
    }
} 
