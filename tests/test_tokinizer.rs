use obsidian_sync_lib;

#[test]
fn test_skip_construction()
{
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction(" { { ", &mut 0, " ", "{{".to_string());
    print!("{}\n",result1);
    assert_eq!(result1,true);
}