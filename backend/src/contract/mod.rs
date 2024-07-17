pub mod contract;
pub mod decentralization;
pub mod security;
pub mod transparency;

pub use security::{authenticate_owner, validate_input};

pub use crate::contract::contract::{handle, state, distribute_rewards, handle_decentralized_votes};
pub use crate::contract::transparency::log_transaction;
pub use crate::contract::decentralization::{
    distribute_rewards as decentralization_distribute_rewards,
    handle_decentralized_votes as decentralization_handle_decentralized_votes,
};


