use git2::Error;
use git2::{Repository, PushOptions, RemoteCallbacks};
use std::fmt::format;
use std::path::Path;
use std::fs;
use std::panic;
//use std::error::Error;
use std::usize;
pub mod tokinezed;
use tokinezed::Token;
use tokinezed::TokenStruct;

pub mod Parser;
use Parser::*;
pub mod AccTokCrypt;
use AccTokCrypt::*;
pub mod DirCheck;
use DirCheck::*;
pub mod Config;
use Config::*;
pub mod debug;
use debug::*;
pub mod black_list_iterator;

#[derive(Debug)]
enum Cfg_Error {
    ParseError(String),
    EmptyFile(String),
    NotSearchConfig(String)
}
#[derive(Debug)]
pub enum Tokinezed_Error {
    SyntaxErr(String),
    UnknowToken(String),
    Empty(String),
    RecordBlock(String)
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

pub fn splitt_b_space(str: String,syms:String) -> Result<TokenStruct,()>
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

//тип ошибки Tokinezed_Error соответствует самой высокой причине тоесть если
//ошибка записи config была и был empty config то причина самая верхняя =>
//тоесть RecordBlock
pub fn tokinezed(config: String) -> Result<Vec<String>,Tokinezed_Error>
{
    let cnf_result: Result<(), String> = CONFIG.set_value(&config);
    let mut msg_err: String="".to_string();
    match cnf_result
    {
        Ok(_) =>{},
        Err(err) =>
        {
            msg_err+=&err;
        }
    }
    if let Ok(tokens)=splitt_b_space(config,SPACE_SYMBOLS.to_string())
    {
        for tok in tokens.tok_values.iter().cloned()
        {
            //тут будут вызовы потом из Parser функций
            if tok == Token::as_str( &Token::UserName)
            {
                
            }
        }
        if msg_err.is_empty()
        {
            return Ok(tokens.tok_values);
        }
        else 
        {
            return Err(Tokinezed_Error::RecordBlock(msg_err));
        }
    }
    else 
    {
        msg_err+="Empty config";
        if msg_err == "Empty config"
        {
            return Err(Tokinezed_Error::Empty(msg_err));
        }
        else 
        {
            return Err(Tokinezed_Error::RecordBlock(msg_err));
        }
    }
}

fn config_parser(config: String) -> Result<String,Cfg_Error>
{

    Ok("Ok".to_string())
}

fn get_config<P: AsRef<Path>>(path: P)-> std::io::Result<()> {
    let path: &Path = path.as_ref(); 
    
    if let Ok(contents) = fs::read_to_string("file.txt") 
    {
        
    }
    else 
    {
        let e: Result<String, std::io::Error> = fs::read_to_string("file.txt");
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
    //todo check inits
    let repo: Repository = Repository::open(obsid_vlt_path)?;
    let mut remote: git2::Remote<'_> = repo.find_remote(remote_name)?;
    
    let mut callbacks: RemoteCallbacks<'_> = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        git2::Cred::userpass_plaintext(username, password_or_token)
    });

    let mut push_options: PushOptions<'_> = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // Формат: refs/heads/branch_name
    let refspec: String = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    remote.push(&[&refspec], Some(&mut push_options))?;

    Ok(())
}

pub fn errhandl_indx_commits(error: git2::Error) -> String
{
    let result: String = format!("message -> {}  \n code -> {}",error.message(),error.raw_code());
    return result;
}
// todo check spec https://docs.rs/git2/latest/git2/struct.Revwalk.html
fn indexing_commits(repo_path: String) -> Result<usize, git2::Error> 
{
    let repo: Result<Repository, git2::Error> = Repository::open(&repo_path);
    if let Ok(repo_valid) = repo
    {
        let revw: git2::Revwalk = repo_valid.revwalk()?;
        let cnt: usize = revw.count();
        return Ok(cnt);
    }
    else if let Err(_) = repo
    {
        let init_res: Result<Repository, git2::Error> = Repository::init(&repo_path);
        match init_res
        {
            Ok(_) => {
                return Ok(0);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(0) //ебаный раст не может понять что во всех ветках есть возврат
}