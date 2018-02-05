error_chain! {
    foreign_links {
        Mqtt(::rumqtt::Error);
        Utf8(::std::string::FromUtf8Error);
    }
}
