use git2::{Repository, PushOptions, RemoteCallbacks};
use std::path::Path;
use std::fs;
use std::panic;
use std::error::Error;
pub mod tokinezed;
use tokinezed::Token;
#[derive(Debug)]
enum Parser_Error {
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

const Space_Symbols: &str = " \t\n";

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
    if(str.is_empty() || syms.is_empty()) 
    {
        return Err(())
    }
    let mut cnt : u64 = 0; 
    for c in str.chars() 
    {
        for ss in syms.chars()
        {
            if(c==ss) 
            {
                cnt+=1;
                break;
            }
        }
    }
    Ok(cnt)
}

fn Splitt_b_Space(str: String,syms:String) -> Result<Vec<String>,()>
{
    let mut idx: usize = 0;
    if let Ok(cnt)=Count_Syms_b_Str(&str,syms.to_string())
    {
        let safe_cnt: usize= usize::try_from(cnt)
    .map_err(|_| ())?;
        let mut toks: Vec<String> = vec![String::new();safe_cnt];
        'outer:
        for c in str.chars() 
        {
            for ss in syms.chars()
            {
                if(c==ss){
                    if(!toks[idx].is_empty())//чтоб небыло пустыъ токенов
                    {
                        idx+=1;
                    }
                    continue 'outer;
                }
            }
            
            toks[idx].push(c);
        }
        Ok(toks)
    }
    else {
        Err(())
    }
 
}

//fn Parse_UserName() надо передовать сылку с массива токена сдвинутую ну типо все токены впереди 

fn tokinezed(config: String) -> Result<Vec<String>,Tokinezed_Error>
{
    if let Ok(mut tokens)=Splitt_b_Space(config,Space_Symbols.to_string())
    {
        for tok in tokens.to_vec()
        {
            if(tok == Token::as_str( &Token::UserName)) 
            {
                
            }
        }
        return Ok(tokens)
    }
    else 
    {
       return Err(Tokinezed_Error::Empty("Empty config".to_string()));
    }
}

fn config_parser(config: String) -> Result<String,Parser_Error>
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