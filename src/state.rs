use gstd::{msg, prelude::*};
use crate::contract::DATA;

pub fn state() {
    unsafe {
        let data = DATA.as_ref().expect("No data available");
        msg::reply(data, 0).expect("Unable to send state");
    }
}
