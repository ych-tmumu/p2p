use env_logger;
use log::debug;
use std::net::SocketAddr;

use futures::{future::lazy, prelude::*};
use identify::{AddrManager, IdentifyProtocol, MisbehaveResult, Misbehavior};
use tentacle::{
    builder::ServiceBuilder,
    context::ServiceContext,
    multiaddr::Multiaddr,
    secio::SecioKeyPair,
    service::{ServiceError, ServiceEvent},
    traits::ServiceHandle,
};

fn main() {
    env_logger::init();
    let addr_mgr = SimpleAddrManager {};
    let protocol = IdentifyProtocol::new(1, addr_mgr);
    if std::env::args().nth(1) == Some("server".to_string()) {
        debug!("Starting server ......");
        let mut service = ServiceBuilder::default()
            .insert_protocol(protocol)
            .key_pair(SecioKeyPair::secp256k1_generated())
            .forever(true)
            .build(SimpleHandler {});
        let _ = service.dial("/ip4/127.0.0.1/tcp/1338".parse().unwrap());
        let _ = service.listen("/ip4/127.0.0.1/tcp/1337".parse().unwrap());
        tokio::run(lazy(|| service.for_each(|_| Ok(()))))
    } else {
        debug!("Starting client ......");
        let mut service = ServiceBuilder::default()
            .insert_protocol(protocol)
            .key_pair(SecioKeyPair::secp256k1_generated())
            .forever(true)
            .build(SimpleHandler {});
        let _ = service.dial("/ip4/127.0.0.1/tcp/1337".parse().unwrap());
        let _ = service.listen("/ip4/127.0.0.1/tcp/1338".parse().unwrap());
        tokio::run(lazy(|| service.for_each(|_| Ok(()))))
    }
}

#[derive(Clone)]
struct SimpleAddrManager {}

impl AddrManager for SimpleAddrManager {
    /// Add remote peer's listen addresses
    fn add_listen_addrs(&mut self, _peer_addr: Multiaddr, _addrs: Vec<SocketAddr>) {}
    /// Add our address observed by remote peer
    fn add_observed_addr(&mut self, _peer_addr: Multiaddr, _addr: SocketAddr) {}
    /// Report misbehavior
    fn misbehave(&mut self, _peer_addr: Multiaddr, _kind: Misbehavior) -> MisbehaveResult {
        MisbehaveResult::Disconnect
    }
}

struct SimpleHandler {}

impl ServiceHandle for SimpleHandler {
    fn handle_error(&mut self, _env: &mut ServiceContext, error: ServiceError) {
        debug!("service error: {:?}", error);
    }

    fn handle_event(&mut self, _env: &mut ServiceContext, event: ServiceEvent) {
        debug!("service event: {:?}", event);
    }
}
