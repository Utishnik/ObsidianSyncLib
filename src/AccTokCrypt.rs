use dotenvy::{var,dotenv,Iter};
use crate::fs::File;
use std::io::Write;
use std::error::Error;

#[derive(Clone)]
pub struct SecurityParam
{
    password: String,
    env_path: String
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

pub fn decrypt_acctok(String:&[u8]) -> Result<(),Box<dyn Error>>
{
    let mut sec_par = SecurityParam
    {
        password: "".to_string(),
        env_path: "".to_string()
    };
    let res: Result<(), Box<dyn Error>> = load_env_file(sec_par);
    match res 
    {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}

fn create_env_file(sec_par: SecurityParam) -> Result<(), std::io::Error>  {
    let mut file = File::create(format!("{}.env",sec_par.env_path))?;
    
    writeln!(file, "PASSWORD={}", sec_par.password)?;

    Ok(())
}
pub fn set_pass_env(sec_par: SecurityParam) -> Result<(), std::io::Error> 
{
    let clone_param=sec_par.clone();
    std::fs::remove_file(sec_par.env_path)?;
    if let Err(e) = create_env_file(clone_param)
    {
        return Err(e);
    }
    Ok(())
}

pub fn load_env_file(mut path: SecurityParam) -> Result<(), Box<dyn Error>>
{
    for item in dotenvy::from_filename_iter(path.env_path.to_string())? 
    {
        let (key, val) = item?;
        if(key=="PASSWORD")
        {
            path.password=val;
        }
    }
    Ok(())
}