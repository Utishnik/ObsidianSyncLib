
use std::str::SplitAsciiWhitespace;

use obsidian_sync_lib::splitt_b_space;
use obsidian_sync_lib::{debug::{TESTS, get_count_tests, get_test, result_list,successes_beh}, tokinezed::construction, *};
use std::thread;
use std::time::Duration;

#[test]
fn test()
{
    
    let txt: String = "token struct{ }    \n   if else for ".to_string();
    let result: Result<obsidian_sync_lib::tokinezed::TokenStruct, ()> = splitt_b_space(txt," \t".to_string(),None);
    
    /*
    for ts in result.clone().unwrap().tok_values
    {
        println!("{}",ts);
    }
    
    let mut test_str: String = "".to_string();
    println!("state: {}",test_str.is_empty());
    test_str.push('c');
    println!("state: {}",test_str.is_empty());
    println!("\t: {}",test_str);

    thread::sleep(Duration::from_secs(2));
    clear_console();
    */
    let len: usize = result.clone().unwrap().tok_lines_number.len();
    println!("capacity:\t{}",len);
    test_assert!(len==1,true);
    for ls in result.clone().unwrap().tok_lines_number
    {
        println!("\n---- {} -----\n",ls);
    }
    successes_beh();
}