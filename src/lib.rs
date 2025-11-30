use git2::{Repository, PushOptions, RemoteCallbacks};
use std::path::Path;
use std::fs;
use std::panic;
use std::error::Error;
use std::usize;
pub mod tokinezed;
use tokinezed::Token;
use tokinezed::TokenStruct;
pub mod Parser;
use Parser::*;
pub mod AccTokCrypt;
use AccTokCrypt::*;
#[derive(Debug)]
enum Cfg_Error {
    ParseError(String),
    EmptyFile(String),
    NotSearchConfig(String)
}
#[derive(Debug)]
enum Tokinezed_Error {
    SyntaxErr(String),
    UnknowToken(String),
    Empty(String)
}

const SPACE_SYMBOLS: &str = " \t\n";

///  `git add .`
pub fn add_all<P: AsRef<std::path::Path>>(repo_path: P) -> Result<(), git2::Error>
{
    let repo = Repository::open(repo_path)?;
    let mut index = repo.index()?;
    index.add_all(["."], git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

pub fn last_commit_message<P: AsRef<std::path::Path>>(repo_path: P) -> Result<String, git2::Error> 
{
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?;
    let oid = head.target().ok_or(git2::Error::from_str("No HEAD"))?;
    let commit = repo.find_commit(oid)?;
    Ok(commit.message().unwrap_or("No message").to_string())
}

fn Count_Syms_b_Str(str:&String,syms: String) -> Result<u64,()>
{
    if str.is_empty() || syms.is_empty()
    {
        return Err(())
    }
    let mut cnt : u64 = 0; 
    for c in str.chars() 
    {
        for ss in syms.chars()
        {
            if c==ss
            {
                cnt+=1;
                break;
            }
        }
    }
    Ok(cnt)
}

fn Splitt_b_Space(str: String,syms:String) -> Result<TokenStruct,()>
{
    let mut idx: usize = 0;
    let mut line: u64 =0;
    if let Ok(cnt)=Count_Syms_b_Str(&str,syms.to_string())
    {
        let safe_cnt: usize= usize::try_from(cnt)
    .map_err(|_| ())?;
        let mut toks: TokenStruct = TokenStruct::new(safe_cnt);
        'outer:
        for c in str.chars() 
        {
            for ss in syms.chars()
            {
                if c=='\n'//todo переделать для произвольных символово переноса
                {
                    line+=1;
                    continue 'outer;
                }
                if c==ss
                {
                    if !toks.tok_values[idx].is_empty()//чтоб небыло пустыъ токенов
                    {
                        idx+=1;
                    }
                    continue 'outer;
                }
            }
            
            toks.tok_values[idx].push(c);
            toks.tok_lines_number[idx]=line;
        }
        Ok(toks)
    }
    else {
        Err(())
    }
 
}



fn tokinezed(config: String) -> Result<Vec<String>,Tokinezed_Error>
{
    if let Ok(tokens)=Splitt_b_Space(config,SPACE_SYMBOLS.to_string())
    {
        for tok in tokens.tok_values.iter().cloned()
        {
            if tok == Token::as_str( &Token::UserName)
            {
                
            }
        }
        Ok(tokens.tok_values)
    }
    else 
    {
        Err(Tokinezed_Error::Empty("Empty config".to_string()))
    }
}


fn config_parser(config: String) -> Result<String,Cfg_Error>
{

    Ok("Ok".to_string())
}

fn get_config<P: AsRef<Path>>(path: P)-> std::io::Result<()> {
    let path = path.as_ref(); 
    
    if let Ok(contents) = fs::read_to_string("file.txt") 
    {
        
    }
    else 
    {
        let e = fs::read_to_string("file.txt");
        println!("{:#?}", e);   
    }

    Ok(())
}
fn git_push<P: AsRef<std::path::Path>>(
    obsid_vlt_path: P,
    remote_name: &str, 
    branch_name: &str, 
    username: &str,
    password_or_token: &str,
) -> Result<(), git2::Error> 
{
    let repo = Repository::open(obsid_vlt_path)?;
    let mut remote = repo.find_remote(remote_name)?;
    
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        git2::Cred::userpass_plaintext(username, password_or_token)
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // Формат: refs/heads/branch_name
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    remote.push(&[&refspec], Some(&mut push_options))?;

    Ok(())
}