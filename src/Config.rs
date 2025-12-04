use std::error::Error;
use std::path::Path;
use std::sync::Once;
use std::sync::RwLock;

pub struct Config
{
    config: RwLock<String>,
}

impl Config
{
    pub fn new() -> Self
    {
        Self { config: RwLock::new(String::new())}
    }

    pub fn init_check(&self) -> String
    {
        let _guard: std::sync::RwLockReadGuard<'_, String> = match self.config.read()
        {
            Ok(val) => {return val.to_string()},
            Err(_) => {return "".to_string()},
        };  
    }
    pub fn set_value(&self,value: &str) -> Result<(),String>{
        let mut guard = self.config.write()
                .map_err(|e: std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, String>>| format!("Failed to acquire write lock: {}", e))?;
        *guard = value.to_string();
        Ok(())
    }

    pub fn get_value(&self) -> Result<String,String>
    {
        if !self.init_check().is_empty()
        {
            return Ok("".to_string());
        }
        else 
        {
            let guard: std::sync::RwLockReadGuard<'_, String> = self.config.read()
                .map_err(|e: std::sync::PoisonError<std::sync::RwLockReadGuard<'_, String>>| format!("Failed to acquire write lock: {}", e))?;
            return Ok(guard.clone());
        }
    }
}
