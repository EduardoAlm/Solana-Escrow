// program logic

use anchor_lang::prelude::{ProgramError, next_account_info};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
};

use crate::{instruction::EscrowInstruction, error::EscrowError};

pub struct processor;

impl Processor {
        
    /// The process function starts by taking the instruction_data recieved and uses the fn from instruction.rs unpack to unpack the slice recieved.
    /// Then through a match tries to understand which processing function to call.
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        // We unpack the data recieved through our unpack function 
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instruction Init Escrow"); // logs the path being undertaken
                Self::process_init_escrow(accounts, amount, program_id)
            }
        }
    }

    /// The process_init_escrow fn starts by creating a mutable iterator of accounts (needs to be mut so we can retrieve information).
    /// The first account we expect is to be the escrow initializer as defined in the instruction.rs. This being said
    /// the first account must be a signer therefore its checked right away.
    fn process_init_escrow(accounts: &[AccountInfo], amount: u64, program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // We dont need to check if its writable for the transaction will fail by itself if it isnt.
        let temp_token_account = next_account_info(account_info_iter)?;

        let account_to_receive_token = next_account_info(account_info_iter)?;
        
        /// We must check if the account_to_recieve_token is owned by the token program and not the temp_token_account for the transfer will fail
        /// later on if the token program cant transfer onwership of the temp_token_account to the PDA it will fail by itself 
        /// (only programs that own accounts may change accounts).
        if *account_to_receive_token.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let escrow_account = next_account_info(account_info_iter)?;
        
        // Rent is deducted from and account balance according to their space reqs. If this account reaches 0 it is erased.
        // An account can be rent-exempt if it reaches a threshold that depends on the space it is consuming.
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        // Since data is an array of u8 we must deserialize it with Escrow::unpack_uncheked
        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
        if escrow_info.is_initialized() {
            return  Err(ProgramError::AccountAlreadyInitialized);
        }

        Ok(())
    }

}