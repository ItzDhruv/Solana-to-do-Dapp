use anchor_lang::prelude::*;

#[error_code]
pub enum TodoError {

    #[msg("You ar  not authorized to perform ths actio")]
    Unauthorized,
    #[msg("not allowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MatghOverflow,
    #[msg("Already marked")]
    AlreadyMarked,
}