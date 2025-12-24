use crate::{
    black_list_iterator::{self, AsciiSymbol},
    debug_eprintln_fileinfo, debug_println, debug_println_fileinfo,
    tokinezed::{self, *},
    utils::{self, *},
    AccTokCrypt::{self, SecurityParam},
    Config,
    DirCheck::full_check_directory,
};

use std::error::Error;
use std::path::Path;
use std::sync::LazyLock;
use std::sync::Once;
use std::{fmt, sync::RwLock};
use validator::{Validate, ValidationError};

#[derive(Clone)]
pub enum ParserError {
    NotFindInit(String),
    EmptyFile(String),
    NotSearchConfig(String),
    DoubleFind(String),
}

#[derive(Debug, Validate)]
struct SignupData {
    #[validate(email)]
    mail: String,
    #[validate(url)]
    site: String,
}

#[derive(Clone, PartialEq)]
pub enum ParseTimeError {
    None,
    KeyWord(String),
    Time(String),
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

pub static CONFIG: LazyLock<Config::Config> = LazyLock::new(Config::Config::new);

//todo логическая ошибка с непривальным возвравтом
pub fn parse_string(toks: &TokenStruct, index: usize) -> Result<Token_String, String> {
    //let toks_len:usize=toks.get_size();
    let mut start_index: usize = 0;
    let mut start_index_find: bool = false;
    let mut end_index: usize = 0;

    let mut find_str = |str: String| -> bool {
        for item in str.chars().enumerate().skip(index) {
            let (i, t): (usize, char) = item;
            if t == '\"' && !start_index_find {
                start_index = i;
                start_index_find = true;
            } else if t == '\"' {
                end_index = i;
            }
        }
        end_index != 0
    };
    for t in toks.tok_values.iter().cloned() {
        if find_str(t) {
            break;
        }
    }

    if end_index == 0 {
        return Err("not find end \"".to_string());
    }

    let mut tstr = Token_String {
        tok_start: start_index,
        tok_end: end_index,
        tok_val: "".to_string(),
    };

    let mut original_str: String = "".to_string();
    let cfg_res: Result<String, String> = CONFIG.get_value();
    match cfg_res {
        Ok(val) => {
            original_str = val;
        }
        Err(err) => {
            return Err(err);
        }
    }
    //todo распаковка config
    tstr.tok_start = start_index;
    tstr.tok_end = end_index;
    tstr.tok_val = original_str[start_index..end_index].to_string();

    Ok(tstr)
}

pub struct FindTokenResult {
    index: usize,
    double_find: bool,
    double_find_index: usize,
}

#[derive(Clone)]
pub enum IteratorCommitType {
    None,
    Numeration,
    Time,
    CustomScript,
}

impl IteratorCommitType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IteratorCommitType::None => "",
            IteratorCommitType::Numeration => "num",
            IteratorCommitType::Time => "time", //{{time:(d:,h:,...)}}
            IteratorCommitType::CustomScript => "JS", //наверное буду использовать boa js
        }
    }
}

pub enum IteratorPushType {
    None,
    Cnt,
    Diff,
    CustomScript,
    GraphDiff,
}

impl IteratorPushType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IteratorPushType::None => "",
            IteratorPushType::Cnt => "cnt",
            IteratorPushType::Diff => "diff",
            IteratorPushType::CustomScript => "JS", //наверное буду использовать boa js
            IteratorPushType::GraphDiff => "graph_diff",
        }
    }
}

pub struct IteratorCommit
//{{ iter_type }}
{
    pub msgpos: IteratorDecl, //pub это временное решение
    pub itr_type: IteratorCommitType,
}

impl IteratorCommit {
    pub fn get_iter_pos(&self) -> IteratorDecl {
        let pos: IteratorDecl = IteratorDecl {
            start: self.msgpos.start,
            end: self.msgpos.end,
            init: self.msgpos.init,
        };
        pos
    }

    pub fn get_itr_type(&self) -> IteratorCommitType {
        self.itr_type.clone() //как по мне удобнее будет чем сыллка + enum же мало это же примерно небольшой массив
    }

    //todo set
}

pub struct iteratorPush
// << iter_type >>
{
    msgpos: IteratorDecl,
    itr_type: IteratorPushType,
}

pub struct IteratorDecl {
    start: usize,
    end: usize,
    init: bool,
}

pub struct PubPosStr
// нужен для получения в него из приватных структур
{
    pub start: usize,
    pub end: usize,
}

impl IteratorDecl {
    fn is_one_symbol(&self) -> bool {
        if self.start == self.end {
            false
        } else {
            true
        }
    }

    pub fn get_pos(&self) -> PubPosStr {
        PubPosStr {
            start: self.start,
            end: self.end,
        }
    }

    pub fn get_init_state(&self) -> bool {
        self.init
    }

    fn set(&mut self, new_start: usize, new_end: usize, init_state: bool) {
        self.start = new_start;
        self.end = new_end;
        self.init = init_state;
    }
}

pub fn find_token(
    toks: &TokenStruct,
    skip_index: usize,
    token: Token,
) -> Result<FindTokenResult, ParserError> {
    let mut fndt_res: FindTokenResult = FindTokenResult {
        index: 0,
        double_find: false,
        double_find_index: 0,
    };
    let mut result: Result<FindTokenResult, ParserError> = Err(ParserError::NotFindInit(format!(
        "Not {} Init",
        token.as_str()
    )));
    'leave: for item in toks.tok_values.to_vec().iter().enumerate().skip(skip_index) {
        let (i, t): (usize, &String) = item;
        if t == token.as_str() {
            if !fndt_res.double_find {
                fndt_res.index = i;
                fndt_res.double_find = true;
            } else {
                fndt_res.double_find_index = i;
                result = Err(ParserError::DoubleFind(
                    ("Double find colon: ".to_owned() + &i.to_string()).to_string(),
                ));
                break 'leave;
            }
        }
    }
    if fndt_res.index != 0 {
        result = Ok(fndt_res);
    }
    result
}

macro_rules! generate_tok_parse {
    ($parse_type: ident , $token: path,$error: ty) => {
        pub fn $parse_type(toksref: &TokenStruct, index: usize) -> Result<FindTokenResult, $error> {
            let result = find_token(toksref, index, $token); //ебаному расту похуй на неиспользования в макросах аргументов
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
pub fn parse_set_username(toksref: &TokenStruct, index: usize) -> Result<String, String> {
    let mut username: String = "".to_string();
    let parse_usrname = parse_username(toksref, index);
    if let Ok(findres) = parse_usrname {
        let index_find = findres.index;
        let parse_str: Result<Token_String, String> = parse_string(toksref, index_find);
        if let Ok(str_find) = parse_str {
            username = str_find.tok_val.to_string();
        } else if let Err(e) = parse_str {
            return Err(e);
        }
    } else if let Err(e) = parse_usrname {
        let msg_err: String = e.to_string();
        return Err(msg_err);
    }
    if !username
        .chars()
        .any(|c| c.is_control() || c == '<' || c == '>' || c == '"' || c == '\\' || c == '\0')
    {
        return Ok(username);
    }
    Err(format!("unresolved characters in username\t {}", username))
}

//not validation
fn parse_set_email(toksref: &TokenStruct, index: usize) -> Result<String, String> {
    let mut email: String = "".to_string();
    let parse_email = parse_email(toksref, index);
    if let Ok(findres) = parse_email {
        let index_find = findres.index;
        let parse_str: Result<Token_String, String> = parse_string(toksref, index_find);
        if let Ok(str_find) = parse_str {
            email = str_find.tok_val.to_string();
        } else if let Err(e) = parse_str {
            return Err(e);
        }
    } else if let Err(e) = parse_email {
        let msg_err: String = e.to_string();
        return Err(msg_err);
    }
    if email.is_empty() {
        email = "Empty".to_string();
        return Err("email address is not specified".to_string());
    }
    Ok(email)
}

//auto decrypt
pub fn parse_set_acctok(
    toksref: &TokenStruct,
    index: usize,
    sec_par: &SecurityParam,
) -> Result<String, String> {
    let mut acctok: String = "".to_string();
    let parse_acctok = parse_acctok(toksref, index);
    if let Ok(findres) = parse_acctok {
        let index_find = findres.index;
        let parse_str: Result<Token_String, String> = parse_string(toksref, index_find);
        if let Ok(str_find) = parse_str {
            acctok = str_find.tok_val.to_string();
            if acctok.is_empty() {
                return Err("access token is not specified".to_string());
            }
            match AccTokCrypt::decrypt_acctok(acctok.as_bytes(), sec_par.clone()) {
                Ok(val) => return Ok(val),
                Err(_) => return Err("Parser: Decrypt Access token error".to_string()),
            };
        } else if let Err(e) = parse_str {
            return Err(e);
        }
    } else if let Err(e) = parse_acctok {
        let msg_err: String = e.to_string();
        return Err(msg_err);
    }
    Ok(acctok) //ебаный раст не может понять что во всех ветках возврат значения
}

//auto validation
pub fn parse_set_path_obsidian(toksref: &TokenStruct, index: usize) -> Result<String, String> {
    let mut obs_storage_path: String = "".to_string();
    let parse_path = parse_path_obsidian(toksref, index);
    if let Ok(findres) = parse_path {
        let index_find = findres.index;
        let parse_str: Result<Token_String, String> = parse_string(toksref, index_find);
        if let Ok(str_find) = parse_str {
            obs_storage_path = str_find.tok_val.to_string();
            if obs_storage_path.is_empty() {
                return Err("obsidian path is not specified".to_string());
            }
            let dir_check: Result<bool, String> = full_check_directory(obs_storage_path.clone());
            let res_check: bool = match dir_check {
                Ok(val) => val,
                Err(error) => {
                    return Err(error);
                }
            };
            if !res_check {
                return Err("obsidian storage access denied".to_string());
            }
        } else if let Err(e) = parse_str {
            return Err(e);
        }
    } else if let Err(e) = parse_path {
        let msg_err: String = e.to_string();
        return Err(msg_err);
    }
    Ok(obs_storage_path)
}
//todo потдежка итератор типо cm1 cm2 ....
pub fn parse_set_text_commit(toksref: &TokenStruct, index: usize) -> Result<String, String> {
    let mut commit_text: String = "".to_string();
    let parse_usrname = parse_text_commit(toksref, index);
    if let Ok(findres) = parse_usrname {
        let index_find = findres.index;
        let parse_str: Result<Token_String, String> = parse_string(toksref, index_find);
        if let Ok(str_find) = parse_str {
            commit_text = str_find.tok_val.to_string();
        } else if let Err(e) = parse_str {
            return Err(e);
        }
    } else if let Err(e) = parse_usrname {
        let msg_err: String = e.to_string();
        return Err(msg_err);
    }
    if !commit_text
        .chars()
        .any(|c| c.is_control() || c == '"' || c == '\\' || c == '\0')
    {
        return Ok(commit_text);
    }
    Err(format!(
        "unresolved characters in username\t {}",
        commit_text
    ))
}

//mili seconds
//{{time:(d:,h:,...)}}
pub fn parse_time(time: &str, index: &mut usize, file: String) -> Result<(), ParseTimeError> {
    let d_construct: &mut Construction = &mut Construction::default();
    let h_construct: &mut Construction = &mut Construction::default();
    let m_construct: &mut Construction = &mut Construction::default();
    let s_construct: &mut Construction = &mut Construction::default();
    let ms_construct: &mut Construction = &mut Construction::default();

    let state1: bool = skip_symbol(time, index, "(".to_string());
    #[rustfmt::skip]
    let d: Option<crate::abstract__tokinezer::AbstractParseValue<String, TokinezedErrorLow>> =
    skip_construction_abstract_parse_value(time, index, " \t\n",
    "d:", d_construct, file);
    let unwrap_d: crate::abstract__tokinezer::AbstractParseValue<String, TokinezedErrorLow> =
        match d {
            Some(val) => val,
            None => {
                let err_msg: String;
                if d_construct.start.is_some() {
                    let unwrap_d_construct_start = unsafe { d_construct.start.unwrap_unchecked() };
                    err_msg = format!(
                        "the beginning of an unsuccessful parsing: {}",
                        unwrap_d_construct_start
                    );
                } else {
                    err_msg = format!(
                        "skip_symbol_abstract_parse_value err and len_str < len_construction"
                    );
                }
                debug_eprintln_fileinfo!("{}", err_msg);
                return Err(ParseTimeError::Time(err_msg));
            }
        };
    //skip value и ,
    let h: bool = skip_construction(time, index, "todo!", "h:", h_construct);
    let m: bool = skip_construction(time, index, "todo!", "m:", m_construct);
    let s: bool = skip_construction(time, index, "todo!", "s:", s_construct);
    let ms: bool = skip_construction(time, index, "todo!", "ms:", ms_construct);
    let state2: bool = skip_symbol(time, index, ")".to_string());
    Ok(())
}

//todo тестировать не только с ascii
pub fn parse_time_commit_sync(
    str: &str,
    index: &mut usize,
    ignore_symbol_list: &str,
) -> Result<(), ParseTimeError> {
    let mut parse_error_hand: ParseTimeError = ParseTimeError::None;
    let skip_time_iter: &mut Construction = &mut Construction::default();
    let skip_time: bool = skip_construction(str, index, ignore_symbol_list, "time", skip_time_iter);
    let find_s: Option<usize> = str.find(":"); //не skip так как неопредельшь при ошибки в key word time где начинать
    if find_s.is_none() {
        parse_error_hand = ParseTimeError::KeyWord("syntax error: not find".to_string());
        return Err(parse_error_hand);
    }
    if !skip_time {
        let error_msg: Option<String> =
            substr_by_char_end_start_idx_owned(str, find_s.unwrap(), *index);
        if error_msg.is_some() {
            debug_println_fileinfo!("error msg = {}", error_msg.clone().unwrap());
            parse_error_hand = ParseTimeError::KeyWord(error_msg.clone().unwrap());
        }
        return Err(parse_error_hand);
    } else if find_s.unwrap_or(0) < skip_time_iter.end.unwrap_or(0) {
        parse_error_hand = ParseTimeError::KeyWord("syntax error: time ... -> \':\'".to_string());
        return Err(parse_error_hand);
    }
    //todo дописать
    Ok(())
}

pub fn parse_text_commit_iter_body(str: &str) {}

pub fn parse_text_commit_iterator(
    str: &str,
    index: usize,
    start_commit_iter: String,
    end_commit_iter: String,
) -> Option<Vec<IteratorCommit>> {
    let mut iterators: Vec<IteratorCommit> = Vec::new();
    let mut index_clone: usize = index;
    let str_len: usize = str.len();
    let len_end_commit_iter: usize = end_commit_iter.len();

    let mut find_decl = |str: String| -> Option<IteratorDecl> {
        let mut decl: IteratorDecl = IteratorDecl {
            start: 0,
            end: 0,
            init: false,
        };
        Some(decl)
    };

    let mut find_iterator = |str: &str| -> Option<IteratorCommit> {
        let mut decl: IteratorDecl = IteratorDecl {
            start: 0,
            end: 0,
            init: false,
        };
        let mut iterator: IteratorCommit = IteratorCommit {
            msgpos: decl,
            itr_type: IteratorCommitType::None,
        };
        let mut iter_construct: Construction = Construction {
            start: None,
            end: None,
            monolit: false,
        };
        let us: String = utils::unique_sym_to_str(&start_commit_iter, &end_commit_iter);
        let ignored_symbols: String = AsciiSymbol::new(us.to_string()).collect();
        let mut skip_result: bool = skip_construction(
            &str,
            &mut index_clone,
            &ignored_symbols,
            &start_commit_iter,
            &mut iter_construct,
        );
        if skip_result {
            decl = IteratorDecl {
                start: iter_construct.start?,
                end: 0, /*тут будет конец }} */
                init: true,
            };
        } else {
            debug_println_fileinfo!(
                "parse_text_commit_iterator find_iterator debug skip_result1 branch"
            );
            return None;
        }
        debug_println!("index: {}", index_clone);
        index_clone += 1;
        skip_result = skip_construction(
            &str,
            &mut index_clone,
            &ignored_symbols,
            &end_commit_iter,
            &mut iter_construct,
        );
        index_clone += len_end_commit_iter;
        if skip_result {
            decl.end = iter_construct.end?; //
            iterator.msgpos = decl;
        } else {
            debug_println_fileinfo!(
                "parse_text_commit_iterator find_iterator debug skip_result2 branch"
            );
            return None;
        }

        Some(iterator)
    };
    loop {
        let option_find_iter: Option<IteratorCommit> = find_iterator(str);
        let find_iter_res: IteratorCommit = match option_find_iter {
            None => {
                debug_println_fileinfo!("parse_text_commit_iterator loop debug break branch");
                break;
            }
            Some(x) => {
                debug_println_fileinfo!(
                    "parse_text_commit_iterator loop debug  start: {}   end: {}",
                    x.msgpos.get_pos().start,
                    x.msgpos.get_pos().end
                );
                x
            }
        };
        iterators.push(find_iter_res);
    }

    Some(iterators)
}
