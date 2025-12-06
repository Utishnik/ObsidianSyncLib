 use obsidian_sync_lib::*;
 
#[test]
fn test_skip_construction()
{
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("      { { ", &mut 0, " ", "{{");
    print!("{}\n",result1);
    test_assert!(result1,true);
   
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("       [  ", &mut 0, " ", "[[");
    print!("{}\n",result1);
    assert_eq!(result1,false); 
    
    let result1: bool = obsidian_sync_lib::tokinezed::skip_construction("  {i t e r}", &mut 0, " ", "{iter}");
    print!("{}\n",result1);
    assert_eq!(result1,true); 
}