extern crate rumqtt;
extern crate signals;

use std::{env, process};

fn main() {
    let mqtt_client_id = match env::var("MQTT_CLIENT_ID") {
        Ok(val) => val,
        Err(err) => {
            println!("MQTT_CLIENT_ID {}", err);
            process::exit(1);
        }
    };

    let mqtt_options = rumqtt::MqttOptions::new()
        .set_keep_alive(5)
        .set_reconnect(3)
        .set_client_id(mqtt_client_id);

    signals::run(mqtt_options);
}
