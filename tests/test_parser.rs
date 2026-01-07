use obsidian_sync_lib::Parser::*;
use obsidian_sync_lib::{
    debug::debug::{TESTS, get_count_tests, get_test, result_list},
    tokinezed::Construction,
    *,
};

use tinyvec::*;

#[test]
fn test_parse_text_commit_iterator() {
    let start_iter: String = "{{".to_string();
    let end_iter: String = "}}".to_string();
    let mut res: Vec<IteratorCommit> = Vec::new();
    let option_res: Option<Vec<IteratorCommit>> = parse_text_commit_iterator(
        "ghghghaaa553gh {{}} {{iter}} {zxc}  gggghhg7_hhth",
        0,
        start_iter,
        end_iter,
    );

    test_assert!(option_res.is_some(), true);
    let mut x: Vec<IteratorCommit> = option_res.unwrap();
    let b: bool = x.is_empty();
    debug_println_fileinfo!("vec len = {}", x.len());
    test_assert!(x.len() == 2, true);
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
    let start_iter: String = "{{".to_string();
    let end_iter: String = "}}".to_string();

    let mut res: Vec<IteratorCommit> = Vec::new();
    let option_res: Option<Vec<IteratorCommit>> = parse_text_commit_iterator(
        "ghghghaaa553gh {{iter}} {{aaa}} {{zxc}} {{iter}} gggghhg7_hhth",
        0,
        start_iter,
        end_iter,
    );

    test_assert!(option_res.is_some(), true);
    let mut x: Vec<IteratorCommit> = option_res.unwrap();
    let b: bool = x.is_empty();
    debug_println_fileinfo!("vec len = {}", x.len());
    test_assert!(x.len() == 4, true);
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
fn tst() {
    let operations = vec![|x| x + 1, |x| x * 2, |x| x - 5];
    let m = 1;
    let m1 = 1;
    let m2 = 1;
    println!("{}", operations[0](m));
    println!("{}", operations[1](m));
    println!("{}", operations[2](m));

    let a = |&&x| x;
    let v = 1;
    let rv = &v;
    let rrv = &rv;
    let ret = a(rrv);
}

//todo

/*
#[test]
fn test_parse_text_commit_iterator3() {
    let mut res: Vec<IteratorCommit> = Vec::new();
    let option_res: Option<Vec<IteratorCommit>> = parse_text_commit_iterator(
        "ghghghaaa553gh {{iter} {aaa} {zxc}} {iter} gggghhg7_hhth",
        0,
    );
    //todo такой синтаксис {{}} это значит если например итератор по времени и по кол ву изменений и типо когда оба условия
    // тоесть прошло например 5 мин и 2 изменения

    test_assert!(option_res.is_some(), true);
    let mut x: Vec<IteratorCommit> = option_res.unwrap();
    let b: bool = x.is_empty();
    debug_println_fileinfo!("vec len = {}", x.len());
    test_assert!(x.len() == 2, true);
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
*/
