use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("Unauthorized access! Only the admin can add test results.")]
    UnauthorizedAccess,
}