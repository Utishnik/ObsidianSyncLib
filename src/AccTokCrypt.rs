use crate::fs::File;
use dotenvy::{Iter, dotenv, var};
use git2::Status;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

#[derive(Clone)]
pub struct SecurityParam {
    password: String,
    env_path: String,
}

//todo добавить через dotenvy var запись в переменнные(именно системный) директорию с конфигами
pub const ENV_DIR: &str = "ObsidianSyncEnv";

fn make_path() -> String {
    let res: String = get_home_dir() + ENV_DIR;
    res
}

fn make_certain_path(obsidian_rep_name: String) -> String {
    let res: String = make_path() + &obsidian_rep_name;
    res
}

fn get_home_dir() -> String {
    if cfg!(unix) {
        std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string())
    } else if cfg!(windows) {
        std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\User".to_string())
    } else {
        "/home/user".to_string()
    }
}
pub fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

pub fn xor_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    xor_encrypt(data, key)
}

pub fn decrypt_acctok(
    crypt_acctok: &[u8],
    security_param: SecurityParam,
) -> Result<String, Box<dyn Error>> {
    let res: Result<(), Box<dyn Error>> = load_env_file(security_param.clone());
    match res {
        Ok(_) => {
            let acctok: String = String::from_utf8_lossy(&xor_decrypt(
                crypt_acctok,
                security_param.password.as_bytes(),
            ))
            .to_string();
            Ok(acctok)
        }
        Err(error) => Err(error),
    }
}

pub fn crypt_acctok(
    decrypt_acctok: &[u8],
    security_param: SecurityParam,
) -> Result<String, Box<dyn Error>> {
    let res: Result<(), Box<dyn Error>> = load_env_file(security_param.clone());
    match res {
        Ok(_) => {
            let encrypt_acctok: String = String::from_utf8_lossy(&xor_encrypt(
                decrypt_acctok,
                security_param.password.as_bytes(),
            ))
            .to_string();
            Ok(encrypt_acctok)
        }
        Err(error) => Err(error),
    }
}

fn create_env_file(sec_par: SecurityParam) -> Result<(), std::io::Error> {
    let mut file = File::create(format!("{}.env", sec_par.env_path))?;

    writeln!(file, "PASSWORD={}", sec_par.password)?;

    Ok(())
}

pub fn set_env_path(mut sec_par: SecurityParam, path: String) {
    sec_par.env_path = path;
}

pub fn create_env_dir(
    obsidian_rep_name: String,
    sec_par: &mut SecurityParam,
) -> Result<(), std::io::Error> {
    let dir: String = make_path();
    let ret_val: Result<(), std::io::Error> = fs::create_dir(dir);
    if ret_val.is_ok() {
        let path: String = make_certain_path(obsidian_rep_name);
        let fs_res: Result<(), std::io::Error> = fs::create_dir(&path);
        let match_res: Result<(), std::io::Error> = match fs_res {
            Ok(_) => {
                if cfg!(unix) {
                    fs::set_permissions(&path, fs::Permissions::from_mode(0o700))?;
                } else if cfg!(windows) {
                    //todo сделать под винду
                }
                sec_par.env_path = path;
                Ok(())
            }
            Err(error) => Err(error),
        };
        match_res
    } else if let Err(e) = ret_val {
        Err(e)
    } else {
        //ебаный раст не может понять что оба случая обработаны и это условие не нужно
        Ok(())
    }
}

pub fn set_pass_env(sec_par: SecurityParam) -> Result<(), std::io::Error> {
    let clone_param = sec_par.clone();
    std::fs::remove_file(sec_par.env_path)?;
    create_env_file(clone_param)?;
    Ok(())
}

pub fn load_env_file(mut path: SecurityParam) -> Result<(), Box<dyn Error>> {
    for item in dotenvy::from_filename_iter(&path.env_path)? {
        let (key, val) = item?;
        if key == "PASSWORD" {
            path.password = val;
        }
    }
    Ok(())
}
