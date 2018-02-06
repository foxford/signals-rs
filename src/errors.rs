error_chain! {
    foreign_links {
        Json(::serde_json::Error);
        Mqtt(::rumqtt::Error);
        Utf8(::std::string::FromUtf8Error);
    }
}
