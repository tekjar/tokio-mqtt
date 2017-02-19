use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::sync::Arc;
use std::net::{SocketAddr, ToSocketAddrs};

use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_core::io::{Io, Framed};
use tokio_timer::{Timer, Interval};
use tokio_service::Service;
use tokio_proto::pipeline::ClientService;
use futures::{Future, BoxFuture, future};

// use futures::Sink;
// use futures::Stream;
// use futures::sync::mpsc;

use mqtt3::{Packet, Message, QoS, TopicPath, PacketIdentifier, SubscribeTopic};

use protocol::{MqttProto};

use error::*;
use packet::*;
use codec::MqttCodec;
use clientoptions::MqttOptions;

pub struct Connection {
    pub inner: ClientService<TcpStream, MqttProto>,
}

impl Service for Connection {
    type Request = Packet;
    type Response = Packet;
    type Error = Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}