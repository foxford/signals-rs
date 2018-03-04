use diesel;
use jsonrpc_core as jsonrpc;

error_chain! {
    foreign_links {
        Diesel(::diesel::result::Error);
        Json(::serde_json::Error);
        Mqtt(::rumqtt::Error);
        Utf8(::std::string::FromUtf8Error);
        Uuid(::uuid::ParseError);
    }

    errors {
        BadRequest
        NotFound
        Nom(kind: ::nom::ErrorKind) {
            description("parsing error")
            display("parsing error: {:?}", kind)
        }
    }
}

impl<'a> From<::nom::Err<::nom::types::CompleteStr<'a>>> for Error {
    fn from(err: ::nom::Err<::nom::types::CompleteStr<'a>>) -> Error {
        let kind = err.into_error_kind();
        Error::from_kind(ErrorKind::Nom(kind))
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
