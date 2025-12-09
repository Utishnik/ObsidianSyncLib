use obsidian_sync_lib::{debug::{TESTS, get_count_tests, get_test, result_list}, tokinezed::construction, *};
 
#[test]
fn test_skip_construction()
{
    let defult_val:usize = 1000000;
    let mut construct:construction = construction { start:None, end: None,monolit:false};
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("      { { ", &mut 0, " ", "{{",&mut construct);
    println!("{}",result1);
    println!("\nSTART:  {}\tEND:  {}\n",construct.start.unwrap_or(defult_val),construct.end.unwrap_or(defult_val));
    test_assert!(result1,true);
    construct.reset();
   
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("       [  ", &mut 0, " ", "[[",&mut construct);
    println!("{}",result1);
    println!("\nSTART:  {}\tEND:  {}\n",construct.start.unwrap_or(defult_val),construct.end.unwrap_or(defult_val));
    test_assert!(result1,false); 
    construct.reset();
    
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("  {i t e r}", &mut 0, " ", "{iter}",&mut construct);
    println!("{}",result1);
    println!("\nSTART:  {}\tEND:  {}\n",construct.start.unwrap_or(defult_val),construct.end.unwrap_or(defult_val));
    test_assert!(result1,true); 
    construct.reset();

    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("struct  {}", &mut 6, " ", "{}",&mut construct);
    println!("{}",result1);
    test_assert!(result1,true); 
    construct.reset();

    construct.monolit=true;
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("    struc  t  {}", &mut 0, " ", "struct",&mut construct);
    println!("{}",result1);
    println!("\nSTART:  {}\tEND:  {}\n",construct.start.unwrap_or(defult_val),construct.end.unwrap_or(defult_val));
    test_assert!(result1,false); 
    construct.reset();
    
    construct.monolit=false;
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("stuct  {}", &mut 0, " ", "struct",&mut construct);
    println!("{}",result1);
    println!("\nSTART:  {}\tEND:  {}\n",construct.start.unwrap_or(defult_val),construct.end.unwrap_or(defult_val));
    test_assert!(result1,false); 
    construct.reset();

    construct.monolit=false;
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("  struct", &mut 2, " ", "struct",&mut construct);
    println!("{}",result1);
    println!("\nSTART:  {}\tEND:  {}\n",construct.start.unwrap_or(defult_val),construct.end.unwrap_or(defult_val));
    test_assert!(result1,true); 
    construct.reset();

    construct.monolit=false;
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("{{", &mut 0, " ", "{{",&mut construct);
    println!("{}",result1);
    println!("\nSTART:  {}\tEND:  {}\n",construct.start.unwrap_or(defult_val),construct.end.unwrap_or(defult_val));
    test_assert!(result1,true); 
    construct.reset();


    if !result_list()
    {
        std::process::exit(1);
    }
}