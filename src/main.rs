#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate rumqtt;

use rumqtt::{Message, MqttCallback, MqttClient, MqttOptions, QoS};
use std::{process, thread, time};
use std::sync::{mpsc, Mutex};

use errors::*;

mod errors;

fn main() {
    let client_options = MqttOptions::new()
        .set_keep_alive(5)
        .set_reconnect(3)
        .set_client_id("signals-rs");

    let (tx, rx) = mpsc::channel();

    let on_message_clbk = {
        let tx = Mutex::new(tx);
        move |msg: Message| {
            let tx = tx.lock().unwrap();
            tx.send(msg).unwrap();
        }
    };
    let callbacks = MqttCallback::new().on_message(on_message_clbk);

    let mut client = MqttClient::start(client_options, Some(callbacks)).unwrap_or_else(|err| {
        println!("error: {:?}", err);
        process::exit(1);
    });

    subscribe(&mut client).unwrap_or_else(|err| {
        println!("error: {:?}", err);
        process::exit(1);
    });

    for msg in rx.iter() {
        if let Err(ref e) = handle_message(&mut client, msg) {
            use std::io::Write;
            let stderr = &mut ::std::io::stderr();
            let errmsg = "Error writing to stderr";

            writeln!(stderr, "error: {}", e).expect(errmsg);
        }
    }
}

fn subscribe(client: &mut MqttClient) -> Result<()> {
    let topics = vec![("ping", QoS::Level0)];
    Ok(client.subscribe(topics)?)
}

fn handle_message(client: &mut MqttClient, msg: Message) -> Result<()> {
    println!("Received message: {:?}", msg);
    let payload = String::from_utf8(msg.payload.to_vec())?;
    println!("Payload: {:?}", payload);

    match msg.topic.as_ref() {
        "ping" => {
            let payload = format!("Pong");
            client.publish("pong", QoS::Level0, payload.into_bytes())?;
        }
        _ => {
            unimplemented!();
        }
    }

    Ok(())
}
