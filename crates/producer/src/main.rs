extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use amiquip::{Connection, Exchange, Publish, Result};
use common::{make_healthy_filter, make_ready_filter};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), ()> {
    pretty_env_logger::init();
    info!("Producer Starting Up...");

    let ready_flag = Arc::new(Mutex::new(false));

    let ready = make_ready_filter(String::from("ready"), ready_flag.clone());
    let healthy = make_healthy_filter(String::from("healthy"), ready_flag.clone());

    let routes = healthy.or(ready);

    info!("Spawning Producer Thread");
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
                produce_messages(connection).unwrap();
            }
            Err(_err) => {
                warn!("Service Not Ready");
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
    info!("Starting Producer on: {}", addr);
    warp::serve(routes).run(addr).await;
    Ok(())
}

fn produce_messages(mut connection: Connection) -> Result<()> {
    info!("{:?}", connection);
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    for num in 0..u128::MAX {
        info!("Publishing Message to RabbitMQ");
        exchange.publish(Publish::new(
            format!("hello there: {}", num).as_bytes(),
            "hello",
        ))?;
        thread::sleep(Duration::from_millis(1000));
    }
    connection.close()
}
