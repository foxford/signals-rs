use diesel;
use jsonrpc_core as jsonrpc;

use error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Bad request")]
    BadRequest,

    #[fail(display = "{}", _0)]
    Db(#[cause] diesel::result::Error),

    #[fail(display = "Invalid parameters")]
    InvalidParameters(#[cause] error::ParseError),
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Db(e)
    }
}

impl From<error::ParseError> for Error {
    fn from(e: error::ParseError) -> Self {
        Error::InvalidParameters(e)
    }
}

impl From<Error> for jsonrpc::Error {
    fn from(err: Error) -> Self {
        let code = match err {
            Error::Db(ref e) => match *e {
                diesel::result::Error::NotFound => 404,
                _ => 422,
            },
            _ => 500,
        };

        jsonrpc::Error {
            code: jsonrpc::ErrorCode::ServerError(code),
            message: err.to_string(),
            data: None,
        }
    }
}
