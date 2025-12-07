use obsidian_sync_lib::{black_list_iterator::*};

#[test]
fn test_black_list_iterator()
{
    let test: String = AsciiSymbol::new("".to_string()).collect();
    println!("{}\n\n",test);
    let ascii_test: String = test.bytes()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}\n\n",ascii_test);
}