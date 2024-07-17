use gstd::msg;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};  // Add this line
use std::ptr::addr_of_mut;

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct State {
    pub owner: String,
    pub transactions: Vec<Transaction>, // Store transactions directly
}

pub(crate) static mut STATE: Option<State> = None;

#[derive(Debug, Clone, Deserialize, Serialize, Encode, Decode, TypeInfo)]  // Ensure serde macros are included
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

pub fn handle() {
    let input: String = msg::load().expect("Unable to load input message");
    crate::contract::security::validate_input(&input).expect("Invalid input");
    let transaction =
        serde_json::from_str::<Transaction>(&input).expect("Invalid transaction format");
    unsafe {
        if let Some(state) = &mut *addr_of_mut!(STATE).as_mut().unwrap() {
            crate::contract::security::authenticate_owner(state, &format!("{:?}", msg::source()))
                .expect("Unauthorized access");
            state.transactions.push(transaction.clone());
            crate::contract::transparency::log_transaction(
                &format!("{:?}", msg::source()),
                transaction.amount.into(),  // Convert u64 to u128
            );
            msg::reply(format!("Transaction added: {:?}", transaction), 0)
                .expect("Failed to send reply");
        }
    }
}

#[no_mangle]
pub extern "C" fn state() {
    // Placeholder for state function
    // crate::contract::transparency::get_state();
}

#[no_mangle]
pub extern "C" fn distribute_rewards() {
    crate::contract::decentralization::distribute_rewards();
}

#[no_mangle]
pub extern "C" fn handle_decentralized_votes() {
    crate::contract::decentralization::handle_decentralized_votes();
}
