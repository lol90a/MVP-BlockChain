use gstd::{msg, prelude::*};
use crate::contract::DATA;

pub fn init() {
    unsafe {
        DATA = Some(Vec::new());
    }
    msg::reply("Initialized", 0).expect("Unable to send init reply");
}
