extern crate std;
use ansicodes::mod_control_code::*;

use std::{io::Write, string::String, vec::Vec};

macro_rules! assert_escape_output {
    ($name:ident, $code:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut buf = Vec::new();
            write!(buf, "{}", $code).unwrap();

            let result = String::from_utf8(buf).unwrap();
            assert_eq!(result, $expected);
        }
    };
}

assert_escape_output!(cursor_up_1, CursorUp(1), "\x1B[1A");
assert_escape_output!(cursor_up_23, CursorUp(23), "\x1B[23A");

assert_escape_output!(cursor_down_1, CursorDown(1), "\x1B[1B");
assert_escape_output!(cursor_down_23, CursorDown(23), "\x1B[23B");

assert_escape_output!(cursor_forward_1, CursorForward(1), "\x1B[1C");
assert_escape_output!(cursor_forward_23, CursorForward(23), "\x1B[23C");

assert_escape_output!(cursor_backward_1, CursorBackward(1), "\x1B[1D");
assert_escape_output!(cursor_backward_23, CursorBackward(23), "\x1B[23D");

assert_escape_output!(cursor_left, CursorLeft, "\x1B[1000D");
assert_escape_output!(cursor_save_position, CursorSavePosition, "\x1B[s");
assert_escape_output!(cursor_restore_position, CursorRestorePosition, "\x1B[u");
assert_escape_output!(cursor_get_position, CursorGetPosition, "\x1B[6n");
assert_escape_output!(cursor_next_line, CursorNextLine, "\x1B[E");
assert_escape_output!(cursor_prev_line, CursorPrevLine, "\x1B[F");
assert_escape_output!(cursor_hide, CursorHide, "\x1B[?25l");
assert_escape_output!(cursor_show, CursorShow, "\x1B[?25h");

assert_escape_output!(erase_lines_1, EraseLines(1), "\x1B[1000D\x1B[K");
assert_escape_output!(
    erase_lines_2,
    EraseLines(2),
    "\x1B[1000D\x1B[K\x1B[1A\x1B[1000D\x1B[K"
);
assert_escape_output!(
    erase_lines_3,
    EraseLines(3),
    "\x1B[1000D\x1B[K\x1B[1A\x1B[1000D\x1B[K\x1B[1A\x1B[1000D\x1B[K"
);
