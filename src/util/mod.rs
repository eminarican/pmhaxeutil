pub mod file;
pub mod phar;
pub mod build;
pub mod manifest;

use std::result;

pub type Result = result::Result<(), ()>;
pub type ResultError = std::result::Result<(), String>;

pub fn Ok<E>() -> result::Result<(), E> {
    result::Result::Ok(())
}

pub fn Err() -> Result {
    Result::Err(())
}

pub fn to_result<T, E>(from: result::Result<T, E>) -> Result {
    match from {
        result::Result::Ok(_) => Ok(),
        result::Result::Err(_) => Err()
    }
}

pub fn to_result_err(from: Result, message: String) -> ResultError {
    match from {
        result::Result::Ok(_) => Ok(),
        result::Result::Err(_) => ResultError::Err(message)
    }
}
