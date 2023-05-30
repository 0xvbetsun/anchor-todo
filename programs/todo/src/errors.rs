use anchor_lang::error_code;

#[error_code]
pub enum TodoErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Already marked")]
    AlreadyMarked,
    #[msg("The provided text should be 20 characters long maximum.")]
    TextTooLong,
    #[msg("Not Found")]
    NotFound
}