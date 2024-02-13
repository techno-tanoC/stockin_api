pub mod item;
pub mod query;
mod request;
mod response;

use self::response::{AppError, NotFound};

pub async fn not_found() -> AppError {
    NotFound.into()
}
