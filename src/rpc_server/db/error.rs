use thiserror::Error;

#[derive(Error, Debug)]
pub enum AddUserError {
    #[error("already have same name account")]
    AlreadyHaveSameName,
    #[error("sqlx reported error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("account or password wrong")]
    AccountOrPasswordWrong,
    #[error("sqlx reported error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
