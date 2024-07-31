#![no_std]

extern crate alloc;

pub mod contract;
pub mod handle;
pub mod state;
pub mod init;

#[no_mangle]
extern "C" fn handle() {
    handle::handle();
}

#[no_mangle]
extern "C" fn state() {
    state::state();
}

#[no_mangle]
extern "C" fn init() {
    init::init();
}
