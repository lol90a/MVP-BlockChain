use gstd::{msg, prelude::*};
use crate::contract::DATA;

pub fn handle() {
    let action: String = msg::load().expect("Unable to load message");
    
    unsafe {
        let data = DATA.get_or_insert(Vec::new());
        data.push(action);
    }

    msg::reply("Action recorded", 0).expect("Unable to send reply");
}
