#![no_std]
#![no_main]

use user_lib;
use libafl_qemu_cmd::backdoor;

#[no_mangle]
fn main() -> i32 {
    backdoor();
    0
}
