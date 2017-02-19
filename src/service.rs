use std::boxed::Box;
use std::net::SocketAddr;

use tokio_proto::TcpClient;
use tokio_core::reactor::Handle;
use futures::Future;

use protocol::{MqttProto};
use connection::Connection;
use error::Error;

pub struct Client;

impl Client {
    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item=Connection, Error=Error>> {
        let result = TcpClient::new(MqttProto)
            .connect(addr, handle)
            .map_err(Error::from)
            .map(|client_service| {
                Connection {
                    inner: client_service,
                }
            });

        Box::new(result)
    }

}