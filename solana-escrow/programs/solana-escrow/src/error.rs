// Program specific errors.
use thiserror::Error;

use solana_program::program_error::ProgramError;


/// Defines the error type.
#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Missing Required Signature")]
    MissingRequiredSignature,
    #[error("Not Rent Exempt")]
    NotRentExempt,
}

/// Implements from<EscrowError> allowing the translation from EscrowError to ProgramError.
/// To implement the From trait we must implement the from fn which carries the conversion.
/// The ProgramError implements the Custom variant which allows for convertion of other types
///  to ProgramError.
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}