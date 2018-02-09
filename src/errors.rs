error_chain! {
    foreign_links {
        Diesel(::diesel::result::Error);
        Json(::serde_json::Error);
        Mqtt(::rumqtt::Error);
        Nom(::nom::ErrorKind);
        Utf8(::std::string::FromUtf8Error);
    }

    errors {
        BadRequest
        NotFound
    }
}
