
use std::io;

use tokio_core::io::{Io, Framed};
use tokio_proto::pipeline::ClientProto;

use codec::MqttCodec;

use mqtt3::Packet;

pub struct MqttProto;

impl<T: Io + 'static> ClientProto<T> for MqttProto {
    type Request = Packet;
    type Response = Packet;
    type Transport = Framed<T, MqttCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(MqttCodec))
    }
}