extern crate rumqtt;
extern crate signals;

use std::env;

macro_rules! invalid_env {
    ($var:expr, $err:ident) => ({
        println!("{} {}", $var, $err);
        std::process::exit(1);
    })
}

fn main() {
    let mqtt_host = match env::var("MQTT_HOST") {
        Ok(val) => val,
        Err(err) => invalid_env!("MQTT_HOST", err),
    };
    let mqtt_port = 1883;
    let mqtt_url = format!("{}:{}", mqtt_host, mqtt_port);

    let mqtt_client_id = match env::var("MQTT_CLIENT_ID") {
        Ok(val) => val,
        Err(err) => invalid_env!("MQTT_CLIENT_ID", err),
    };

    let mqtt_options = rumqtt::MqttOptions::new()
        .set_keep_alive(5)
        .set_reconnect(3)
        .set_client_id(mqtt_client_id)
        .set_broker(&mqtt_url);

    if let Err(err) = env::var("DATABASE_URL") {
        invalid_env!("DATABASE_URL {}", err);
    }

    signals::run(mqtt_options);
}
