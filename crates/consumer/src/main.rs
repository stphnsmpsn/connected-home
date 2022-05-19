extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use common::{make_healthy_filter, make_ready_filter};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{net::SocketAddr, thread};
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), ()> {
    pretty_env_logger::init();
    info!("Consumer Starting Up...");

    let ready_flag = Arc::new(Mutex::new(false));

    let ready = make_ready_filter(String::from("ready"), ready_flag.clone());
    let healthy = make_healthy_filter(String::from("healthy"), ready_flag.clone());

    let routes = healthy.or(ready);

    debug!("Spawning Consumer Thread");
    let r = ready_flag.clone();
    thread::spawn(move || loop {
        let cx = Connection::insecure_open("amqp://rabbitmq:rabbitmq@rabbitmq:5672");
        match cx {
            Ok(connection) => {
                info!("Service Ready");
                {
                    let mut t = r.lock().unwrap();
                    *t = true;
                }
                consume_messages(connection).unwrap();
            }
            Err(_err) => {
                debug!("Service Not Ready");
                {
                    let mut t = r.lock().unwrap();
                    *t = false;
                }
                thread::sleep(Duration::from_millis(2000));
            }
        }
    });

    // we listen on all interfaces because we will be inside of a container
    // and we do not know what IP we will be assigned
    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));
    debug!("Starting Consumer on: {}", addr);
    warp::serve(routes).run(addr).await;
    Ok(())
}

fn consume_messages(mut connection: Connection) -> Result<()> {
    debug!("{:?}", connection);
    let channel = connection.open_channel(None).unwrap();
    let queue = channel
        .queue_declare("hello", QueueDeclareOptions::default())
        .unwrap();

    let consumer = queue.consume(ConsumerOptions::default()).unwrap();

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                info!("({:>3}) Received [{}]", i.to_string(), body);
                consumer.ack(delivery).unwrap();
            }
            other => {
                warn!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
    connection.close()
}
