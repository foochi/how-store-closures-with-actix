#[macro_use]
extern crate log;
extern crate env_logger;
extern crate actix;
extern crate actix_web;
extern crate futures;

pub mod events;

use events::{EventManager};

pub fn subscribe() {
    let mut client = EventManager::new();

    client.capture(|data| {
        debug!("method: {:?}", data)
    });
    client.run();
}
