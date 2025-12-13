use obsidian_sync_lib::Parser::*;
use obsidian_sync_lib::{
    debug::{get_count_tests, get_test, result_list, TESTS},
    tokinezed::construction,
    *,
};

#[test]
fn test_parse_text_commit_iterator() {
    let mut res: Vec<IteratorCommit> = Vec::new();
    let option_res: Option<Vec<IteratorCommit>> =
        parse_text_commit_iterator("ghghghaaa553gh {{}} {zxc} {iter} gggghhg7_hhth", 0);

    test_assert!(option_res.is_some(), true);
    let mut x: Vec<IteratorCommit> = option_res.unwrap();
    let b: bool = x.is_empty();
    debug_println_fileinfo!("vec len = {}",x.len());
    test_assert!(x.len()==3,true);
    test_assert!(b, false);
    if !b {
        let ex: Option<IteratorCommit> = x.pop();
        let unwex: IteratorCommit = ex.unwrap();
        let poss: IteratorDecl = unwex.msgpos;
        let get_pos: PubPosStr = poss.get_pos();
        debug_println_fileinfo!("start {}  end {}", get_pos.start, get_pos.end);
    }

    if !result_list() {
        std::process::exit(1);
    }
}

#[test]
fn test_parse_text_commit_iterator2() {
    let mut res: Vec<IteratorCommit> = Vec::new();
    let option_res: Option<Vec<IteratorCommit>> =
        parse_text_commit_iterator("ghghghaaa553gh {iter} {aaa} {zxc} {iter} gggghhg7_hhth", 0);

    test_assert!(option_res.is_some(), true);
    let mut x: Vec<IteratorCommit> = option_res.unwrap();
    let b: bool = x.is_empty();
    debug_println_fileinfo!("vec len = {}",x.len());
    test_assert!(x.len()==4,true);
    test_assert!(b, false);
    if !b {
        let ex: Option<IteratorCommit> = x.pop();
        let unwex: IteratorCommit = ex.unwrap();
        let poss: IteratorDecl = unwex.msgpos;
        let get_pos: PubPosStr = poss.get_pos();
        debug_println_fileinfo!("start {}  end {}", get_pos.start, get_pos.end);
    }

    if !result_list() {
        std::process::exit(1);
    }
}

#[test]
fn test_parse_text_commit_iterator3() {
    let mut res: Vec<IteratorCommit> = Vec::new();
    let option_res: Option<Vec<IteratorCommit>> =
        parse_text_commit_iterator("ghghghaaa553gh {{iter} {aaa} {zxc}} {iter} gggghhg7_hhth", 0);
        //todo такой синтаксис {{}} это значит если например итератор по времени и по кол ву изменений и типо когда оба условия
        // тоесть прошло например 5 мин и 2 изменения

    test_assert!(option_res.is_some(), true);
    let mut x: Vec<IteratorCommit> = option_res.unwrap();
    let b: bool = x.is_empty();
    debug_println_fileinfo!("vec len = {}",x.len());
    test_assert!(x.len()==2,true);
    test_assert!(b, false);
    if !b {
        let ex: Option<IteratorCommit> = x.pop();
        let unwex: IteratorCommit = ex.unwrap();
        let poss: IteratorDecl = unwex.msgpos;
        let get_pos: PubPosStr = poss.get_pos();
        debug_println_fileinfo!("start {}  end {}", get_pos.start, get_pos.end);
    }

    if !result_list() {
        std::process::exit(1);
    }
}



