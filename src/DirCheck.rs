use std::error::Error;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

pub fn check_directory(path: String) -> Result<bool, std::io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.is_dir())
}

pub fn can_read_directory(path: String) -> bool {
    fs::read_dir(path).is_ok()
}

pub fn can_write_directory(path: String) -> bool {
    let test_file = path + "/.test_write_access";
    fs::write(&test_file, b"test").is_ok_and(|_| {
        let _ = fs::remove_file(&test_file);
        true
    })
}

pub fn full_check_directory(path: String) -> Result<bool, String> {
    let check_err: Result<bool, std::io::Error> = check_directory(path.clone());
    let check: bool = match check_err {
        Ok(val) => val,
        Err(_) => {
            return Err("full_check_directory io error".to_string());
        }
    };
    if !can_read_directory(path.clone()) || !can_write_directory(path.clone()) || !check {
        return Ok(false);
    }
    Ok(true)
}
