#![recursion_limit = "1024"]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
extern crate jsonrpc_core;
#[macro_use]
extern crate jsonrpc_macros;
#[macro_use]
extern crate nom;
extern crate rumqtt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use diesel::{PgConnection, r2d2};
use jsonrpc_core::Notification;
use rumqtt::{Message as MqttMessage, MqttCallback, MqttClient, MqttOptions, QoS};
use std::{env, process, thread};
use std::sync::{mpsc, Arc, Mutex};

use errors::*;
use messages::{Envelope, EventKind};
use topic::{AppTopic, ResourceKind, Topic};

macro_rules! establish_connection {
    ($pool:expr) => (
        &$pool.get().expect("Error establishing DB connection")
    )
}

pub mod errors;
pub mod messages;
pub mod rpc;
pub mod topic;
pub mod version;

pub mod schema;
pub mod models;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn run(mqtt_options: MqttOptions) {
    let (tx, rx) = mpsc::channel::<MqttMessage>();
    let tx = Mutex::new(tx);

    let (event_tx, event_rx) = mpsc::channel::<EventKind>();

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

    let client = Arc::new(Mutex::new(client));
    let mut handles = vec![];

    let handle = thread::spawn({
        let client = Arc::clone(&client);
        move || {
            let database_url = env::var("DATABASE_URL").unwrap();
            let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
            let pool = r2d2::Pool::builder()
                .build(manager)
                .expect("Error creating pool.");

            let server = rpc::build_server();

            for msg in rx.iter() {
                let event_tx = event_tx.clone();
                let pool = pool.clone();
                let mut client = client.lock().unwrap();

                if let Err(ref e) = handle_message(&server, &mut client, &msg, event_tx, pool) {
                    use std::io::Write;
                    let stderr = &mut ::std::io::stderr();
                    let errmsg = "Error writing to stderr";

                    writeln!(stderr, "error: {}", e).expect(errmsg);
                }
            }
        }
    });
    handles.push(handle);

    let handle = thread::spawn({
        let client = Arc::clone(&client);
        move || {
            for event in event_rx.iter() {
                let topic = match event {
                    EventKind::AgentCreate(ref event) => AppTopic {
                        room_id: event.room_id,
                        resource: ResourceKind::Agents,
                    },
                    EventKind::AgentDelete(ref event) => AppTopic {
                        room_id: event.room_id,
                        resource: ResourceKind::Agents,
                    },
                    EventKind::TrackCreate(ref event) => AppTopic {
                        room_id: event.room_id,
                        resource: ResourceKind::Tracks,
                    },
                    EventKind::TrackUpdate(ref event) => AppTopic {
                        room_id: event.room_id,
                        resource: ResourceKind::Tracks,
                    },
                    EventKind::TrackDelete(ref event) => AppTopic {
                        room_id: event.room_id,
                        resource: ResourceKind::Tracks,
                    },
                };

                let note = Notification::from(event);
                let payload = serde_json::to_string(&note).unwrap();
                println!("EVENT: {}", payload);

                let mut client = client.lock().unwrap();
                client
                    .publish(&topic.to_string(), QoS::Level1, payload.into_bytes())
                    .unwrap();
            }
        }
    });
    handles.push(handle);

    for handle in handles {
        handle.join().expect("Error joining a thread");
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
    event_tx: ::std::sync::mpsc::Sender<EventKind>,
    pool: DbPool,
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
        event_tx: Some(event_tx),
        db_pool: Some(pool),
    };

    let resp = server.handle_request_sync(&request, meta).unwrap();
    let resp_topic = topic.get_reverse();
    mqtt_client.publish(&resp_topic.to_string(), QoS::Level1, resp.into_bytes())?;

    Ok(())
}
