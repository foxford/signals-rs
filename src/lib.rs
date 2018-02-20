#![recursion_limit = "1024"]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
extern crate jsonrpc_core;
#[macro_use]
extern crate nom;
extern crate rumqtt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use rumqtt::{Message as MqttMessage, MqttCallback, MqttClient, MqttOptions, QoS};
use std::process;
use std::sync::{mpsc, Mutex};

use controllers::MainController;
use errors::*;
use messages::Envelope;
use topic::Topic;

mod controllers;
mod errors;
mod messages;
mod rpc;
mod topic;

mod schema;
mod models;

pub fn run(mqtt_options: MqttOptions) {
    let (tx, rx) = mpsc::channel();
    let tx = Mutex::new(tx);

    let callbacks = MqttCallback::new().on_message(move |msg| {
        let tx = tx.lock().unwrap();
        tx.send(msg).unwrap();
    });

    let mut client = MqttClient::start(mqtt_options, Some(callbacks)).unwrap_or_else(|err| {
        println!("error: {:?}", err);
        process::exit(1);
    });

    subscribe(&mut client).unwrap_or_else(|err| {
        println!("error: {:?}", err);
        process::exit(1);
    });

    let server = rpc::build_server();

    for msg in rx.iter() {
        if let Err(ref e) = handle_message(&server, &mut client, &msg) {
            use std::io::Write;
            let stderr = &mut ::std::io::stderr();
            let errmsg = "Error writing to stderr";

            writeln!(stderr, "error: {}", e).expect(errmsg);
        }
    }
}

fn subscribe(client: &mut MqttClient) -> Result<()> {
    let topics = vec![
        ("ping", QoS::Level0),
        (
            "agents/+/out/signals.netology-group.services/api/v1",
            QoS::Level1,
        ),
    ];

    client.subscribe(topics)?;

    Ok(())
}

fn handle_message(
    server: &rpc::Server,
    mqtt_client: &mut MqttClient,
    mqtt_msg: &MqttMessage,
) -> Result<()> {
    println!("Received message: {:?}", mqtt_msg);

    let topic = Topic::parse(&mqtt_msg.topic)?;
    println!("Topic: {:?}", topic);

    let payload = String::from_utf8(mqtt_msg.payload.to_vec())?;
    println!("Payload: {:?}", payload);

    let envelope: Envelope = serde_json::from_str(&payload)?;
    let request = envelope.msg;

    let meta = rpc::Meta {
        subject: envelope.sub,
    };

    let resp = server.handle_request_sync(&request, meta).unwrap();
    let resp_topic = topic.get_reverse();
    mqtt_client.publish(&resp_topic.to_string(), QoS::Level1, resp.into_bytes())?;

    // let ctrl = MainController::new(&topic);

    // for message in ctrl.call(envelope)? {
    //     mqtt_client.publish(&message.topic.to_string(), message.qos, message.payload)?;
    // }

    Ok(())
}
