extern crate rumqtt;
extern crate signals;

fn main() {
    let mqtt_options = rumqtt::MqttOptions::new()
        .set_keep_alive(5)
        .set_reconnect(3)
        .set_client_id("signals-rs");

    signals::run(mqtt_options);
}
