use diesel::result::Error as DieselError;
use hex::FromHexError;
use serde_json::Error;

#[derive(Debug)]
pub enum GetByTokenError {
    Diesel(DieselError),
    Connection(String), // TODO: This is string because Diesel pub use
    Hex(FromHexError),
    InvalidGene,
    Serde(serde_json::Error),
    Handlebars,
}
