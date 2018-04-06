#[macro_use]
extern crate failure;
extern crate rumqtt;
extern crate signals;

use rumqtt::MqttOptions;
use signals::Options;

use std::{env, process};

macro_rules! die {
    ($err:ident) => {{
        println!("{}", $err);
        process::exit(1);
    }};
}

fn main() {
    let options = build_options().unwrap_or_else(|e| die!(e));

    if let Err(e) = signals::try_run(options) {
        die!(e);
    }
}

fn build_options() -> Result<Options, failure::Error> {
    let mqtt_host = env::var("MQTT_HOST").map_err(|e| VarError {
        var: "MQTT_HOST",
        std_error: e,
    })?;

    let mqtt_port = 1883;
    let mqtt_url = format!("{}:{}", mqtt_host, mqtt_port);

    let mqtt_client_id = env::var("MQTT_CLIENT_ID").map_err(|e| VarError {
        var: "MQTT_CLIENT_ID",
        std_error: e,
    })?;

    let mqtt_options = MqttOptions::new()
        .set_keep_alive(5)
        .set_reconnect(3)
        .set_client_id(mqtt_client_id)
        .set_broker(&mqtt_url);

    let database_url = env::var("DATABASE_URL").map_err(|e| VarError {
        var: "DATABASE_URL",
        std_error: e,
    })?;

    Ok(Options {
        mqtt: mqtt_options,
        database_url,
    })
}

#[derive(Debug, Fail)]
#[fail(display = "{} {}", var, std_error)]
struct VarError {
    var: &'static str,
    std_error: env::VarError,
}
