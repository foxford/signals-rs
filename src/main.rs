#![recursion_limit = "1024"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
extern crate rumqtt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use rumqtt::{Message as MqttMessage, MqttCallback, MqttClient, MqttOptions, QoS};
use std::process;
use std::sync::{mpsc, Mutex};

use errors::*;
use messages::{Envelope, Message};
use schema::rooms;
use topic::Topic;

mod errors;
mod messages;
mod topic;

mod schema;
mod models;

fn main() {
    let client_options = MqttOptions::new()
        .set_keep_alive(5)
        .set_reconnect(3)
        .set_client_id("signals-rs");

    let (tx, rx) = mpsc::channel();
    let tx = Mutex::new(tx);

    let callbacks = MqttCallback::new().on_message(move |msg| {
        let tx = tx.lock().unwrap();
        tx.send(msg).unwrap();
    });

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
    let topics = vec![
        ("ping", QoS::Level0),
        (
            "agents/+/out/signals.netology-group.services/api/v1/rooms/#",
            QoS::Level1,
        ),
    ];
    Ok(client.subscribe(topics)?)
}

fn handle_message(client: &mut MqttClient, mqtt_msg: MqttMessage) -> Result<()> {
    // FIXME: use connection pool
    let conn = establish_connection();

    println!("Received message: {:?}", mqtt_msg);
    let payload = String::from_utf8(mqtt_msg.payload.to_vec())?;
    println!("Payload: {:?}", payload);

    let envelope: Envelope = serde_json::from_str(&payload)?;
    let msg = envelope.message()?;
    let topic = Topic::parse(&mqtt_msg.topic)?;
    println!("Topic: {:?}", topic);

    match topic {
        Topic::Ping => match msg {
            Message::Ping => {
                let payload = serde_json::to_string(&Message::Pong).unwrap();
                Ok(client.publish("pong", QoS::Level0, payload.into_bytes())?)
            }
            _ => unimplemented!(),
        },
        Topic::Pong => unreachable!(),
        Topic::Agent(t) => match t.room {
            Some(_room) => unimplemented!(),
            None => match msg {
                Message::RoomsCreateRequest(req) => {
                    let room: models::Room = {
                        diesel::insert_into(rooms::table)
                            .values(&req.payload)
                            .get_result(&conn)?
                    };

                    let resp = req.build_response(&room);
                    let resp = Message::RoomsCreateResponse(resp);
                    let payload = serde_json::to_string(&resp).unwrap();

                    let topic = t.get_reverse().to_string();
                    Ok(client.publish(&topic, QoS::Level1, payload.into_bytes())?)
                }
                _ => Err(ErrorKind::BadRequest)?,
            },
        },
    }
}

fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}
