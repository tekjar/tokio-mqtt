extern crate futures;
extern crate tokio_core;
extern crate tokio_mqtt;

use std::default::Default;
use futures::Future;
use tokio_core::reactor::Core;

use tokio_mqtt::{Client};

fn main() {
    let addr = "127.0.0.1:1883".parse().unwrap();
    let mut lp = Core::new().unwrap();

    let res = Client::connect(&addr, &lp.handle())
        .and_then(|conn| {
            Ok(())
        })
    ;

    let val = lp.run(res).unwrap();
    // println!("{:?}", val.value());
}