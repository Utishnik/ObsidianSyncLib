use obsidian_sync_lib::str_utils::*;

#[test]
fn test1() {
    let res: String = crate::chunk_str_get("zxc demon zitrex", 3, 7);
    println!("{}", res);
    let res: String = crate::chunk_str_get("zxc demon zitrex", 0, 10);
    println!("{}", res);
}
