use std::io;
use std::net::SocketAddr;

use async_trait::async_trait;

use crate::{
    proxy::{
        OutboundConnect, OutboundDatagram, OutboundTransport, UdpOutboundHandler, UdpTransportType,
    },
    session::Session,
};

pub struct Handler {
    pub path: String,
}

#[async_trait]
impl UdpOutboundHandler for Handler {
    fn name(&self) -> &str {
        super::NAME
    }

    fn udp_connect_addr(&self) -> Option<OutboundConnect> {
        None
    }

    fn udp_transport_type(&self) -> UdpTransportType {
        UdpTransportType::Stream
    }

    async fn handle_udp<'a>(
        &'a self,
        _sess: &'a Session,
        _transport: Option<OutboundTransport>,
    ) -> io::Result<Box<dyn OutboundDatagram>> {
        unimplemented!()
    }
}