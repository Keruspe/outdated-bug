extern crate libc;

pub fn foo() -> String {
    format!("{}", libc::MSG_INFO)
}
