use diesel;
use jsonrpc_core as jsonrpc;

error_chain! {
    foreign_links {
        Diesel(::diesel::result::Error);
        Json(::serde_json::Error);
        Mqtt(::rumqtt::Error);
        Nom(::nom::ErrorKind);
        Utf8(::std::string::FromUtf8Error);
        Uuid(::uuid::ParseError);
    }

    errors {
        BadRequest
        NotFound
    }
}

impl From<Error> for jsonrpc::Error {
    fn from(err: Error) -> Self {
        match err {
            Error(ErrorKind::Diesel(ref e), _) => match *e {
                diesel::result::Error::NotFound => jsonrpc::Error {
                    code: jsonrpc::ErrorCode::ServerError(404),
                    message: err.description().into(),
                    data: None,
                },
                _ => jsonrpc::Error::internal_error(),
            },
            _ => jsonrpc::Error::internal_error(),
        }
    }
}
