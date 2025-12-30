use obsidian_sync_lib::abstract__tokinezer::*;

#[test]
fn test() {
    let test: String = "zxxcxcvvxdff ".to_string();
    let r: Result<String, obsidian_sync_lib::Parser::ParseExprError> =
        String::parse_value(&test, &mut 0);
    if r.is_err() {
        println!("is err");
        assert!(true);
    }
    let r_unwrap: String = unsafe { r.unwrap_unchecked() };
    println!("test: {}", r_unwrap);
}
