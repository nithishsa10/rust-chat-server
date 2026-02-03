use bcrypt;
use crate::error::{AppError, Result};

pub fn hash_password(plain: &str) -> Result<String> {
    bcrypt::hash(plain, BCRYPT_COST).map_err(AppError::from)
}

pub fn verify_password(plain: &str, hashed: &str) -> Result<bool> {
    bcrypt::verify(plain, hashed).map_err(AppError::from)
}