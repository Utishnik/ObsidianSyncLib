use obsidian_sync_lib::Parser::*;
use obsidian_sync_lib::{debug::{TESTS, get_count_tests, get_test, result_list}, tokinezed::construction, *};

#[test]
fn test_parse_text_commit_iterator()
{
    let mut res: Vec<IteratorCommit> = Vec::new();
    let option_res: Option<Vec<IteratorCommit>>=parse_text_commit_iterator("ghghghaaa553gh {{}}  gggghhg7_hhth",0);
    
    test_assert!(option_res.is_some(),true);
    let mut x: Vec<IteratorCommit> = option_res.unwrap();
    let b: bool=/*x.is_empty()*/false;
    //test_assert!(b,false);
    if !b
    {
        let ex: Option<IteratorCommit>=x.pop();
        let unwex: IteratorCommit= ex.unwrap();
        let poss: IteratorDecl = unwex.msgpos;
        debug_println_fileinfo!("start {}  end {}",poss.start,poss.end);
    }

    if !result_list()
    {
        std::process::exit(1);
    }
}
