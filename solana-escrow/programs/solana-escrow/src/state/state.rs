// program objects, de/serializing state

use solana_program::pubkey::Pubkey;

/// The state.rs: 
/// (1) defines state objects which the processor can use,
/// (2) serializes and deserializes these objects into arrays of u8.
/// Here we need to save the temp_token_account_pubkey so that the escrow can send the tokens from 
/// the temp account to the second (receiving) party.
pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub initializer_account_to_receive_token_pubkey: Pubkey,
    pub expected_amount: u64,
}
