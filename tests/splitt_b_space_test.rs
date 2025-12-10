
use std::str::SplitAsciiWhitespace;

use obsidian_sync_lib::splitt_b_space;

#[test]
fn test()
{
    
    let txt: String = "token struct{ }       if else for ".to_string();
    let result: Result<obsidian_sync_lib::tokinezed::TokenStruct, ()> = splitt_b_space(txt," \t".to_string(),None);
    for ts in result.unwrap().tok_values
    {
        println!("{}",ts);
    }
    
    let mut test_str: String = "".to_string();
    println!("state: {}",test_str.is_empty());
    test_str.push('c');
    println!("state: {}",test_str.is_empty());
    println!("\t: {}",test_str);
}