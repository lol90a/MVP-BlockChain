use gstd::{msg, prelude::*};
use crate::contract::contract::State;

pub fn validate_input(input: &str) -> Result<(), &'static str> {
    if input.parse::<u128>().is_err() {
        return Err("Invalid input, not a number");
    }
    Ok(())
}

pub fn authenticate_owner(state: &State, sender: &str) -> Result<(), &'static str> {
    if &state.owner != sender {
        return Err("Unauthorized access");
    }
    Ok(())
}
