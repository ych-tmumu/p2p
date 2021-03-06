use futures::{prelude::*, stream::iter_ok, sync::mpsc};
use log::{debug, error, trace, warn};
use std::collections::{HashMap, HashSet, VecDeque};
use std::{
    io::{self, ErrorKind},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::prelude::{AsyncRead, AsyncWrite, FutureExt};
use tokio::{
    codec::{Framed, FramedParts, LengthDelimitedCodec},
    timer::Delay,
};

use crate::{
    error::Error,
    multiaddr::Multiaddr,
    protocol_select::{client_select, server_select, ProtocolInfo},
    secio::{codec::stream_handle::StreamHandle as SecureHandle, PublicKey},
    service::{
        config::Meta, event::Priority, SessionType, BUF_SHRINK_THRESHOLD, DELAY_TIME,
        RECEIVED_SIZE, SEND_SIZE,
    },
    substream::{ProtocolEvent, SubStream},
    transports::{MultiIncoming, MultiStream},
    yamux::{Config, Session as YamuxSession, StreamHandle},
    ProtocolId, SessionId, StreamId,
};

/// Event generated/received by the Session
#[derive(Debug)]
pub(crate) enum SessionEvent {
    /// Session close event
    SessionClose {
        /// Session id
        id: SessionId,
    },
    ListenStart {
        listen_address: Multiaddr,
        incoming: MultiIncoming,
    },
    DialStart {
        remote_address: Multiaddr,
        stream: MultiStream,
    },
    HandshakeSuccess {
        /// Secure handle
        handle: SecureHandle,
        /// Remote Public key
        public_key: PublicKey,
        /// Remote address
        address: Multiaddr,
        /// Session type
        ty: SessionType,
    },
    HandshakeFail {
        /// remote address
        address: Multiaddr,
        /// Session type
        ty: SessionType,
        /// error
        error: Error,
    },
    DialError {
        /// remote address
        address: Multiaddr,
        /// error
        error: Error,
    },
    ListenError {
        /// listen address
        address: Multiaddr,
        /// error
        error: Error,
    },
    /// Protocol data
    ProtocolMessage {
        /// Session id
        id: SessionId,
        /// Protocol id
        proto_id: ProtocolId,
        /// priority
        priority: Priority,
        /// Data
        data: bytes::Bytes,
    },
    /// Protocol open event
    ProtocolOpen {
        /// Session id
        id: SessionId,
        /// Protocol id
        proto_id: ProtocolId,
        /// Protocol version
        version: String,
    },
    /// Protocol close event
    ProtocolClose {
        /// Session id
        id: SessionId,
        /// Protocol id
        proto_id: ProtocolId,
    },
    ProtocolSelectError {
        /// Session id
        id: SessionId,
        /// proto_name
        proto_name: Option<String>,
    },
    SessionTimeout {
        /// Session id
        id: SessionId,
    },
    /// Codec error
    ProtocolError {
        /// Session id
        id: SessionId,
        /// Protocol id
        proto_id: ProtocolId,
        /// Codec error
        error: Error,
    },
    MuxerError {
        id: SessionId,
        error: Error,
    },
}

/// Wrapper for real data streams, such as TCP stream
pub(crate) struct Session<T> {
    socket: YamuxSession<T>,

    protocol_configs: HashMap<String, Arc<Meta>>,

    config: Config,

    id: SessionId,
    timeout: Duration,
    timeout_check: Option<Delay>,

    state: SessionState,

    // NOTE: Not used yet, may useful later
    // remote_address: ::std::net::SocketAddr,
    // remote_public_key: Option<PublicKey>,
    next_stream: StreamId,
    /// Indicates the identity of the current session
    ty: SessionType,

    /// Sub streams maps a stream id to a sender of sub stream
    sub_streams: HashMap<StreamId, mpsc::Sender<ProtocolEvent>>,
    proto_streams: HashMap<ProtocolId, StreamId>,
    /// The buffer will be prioritized for distribute to sub streams
    high_write_buf: VecDeque<(ProtocolId, ProtocolEvent)>,
    /// The buffer which will distribute to sub streams
    write_buf: VecDeque<(ProtocolId, ProtocolEvent)>,
    /// The buffer which will send to service
    read_buf: VecDeque<SessionEvent>,

    /// Clone to new sub stream
    proto_event_sender: mpsc::Sender<ProtocolEvent>,
    /// Receive events from sub streams
    proto_event_receiver: mpsc::Receiver<ProtocolEvent>,

    /// Send events to service
    service_sender: mpsc::Sender<SessionEvent>,
    /// Receive event from service
    service_receiver: mpsc::Receiver<SessionEvent>,
    /// Delay notify with abnormally poor machines
    delay: Arc<AtomicBool>,
}

impl<T> Session<T>
where
    T: AsyncRead + AsyncWrite,
{
    /// New a session
    pub fn new(
        socket: T,
        service_sender: mpsc::Sender<SessionEvent>,
        service_receiver: mpsc::Receiver<SessionEvent>,
        meta: SessionMeta,
    ) -> Self {
        let socket = YamuxSession::new(socket, meta.config, meta.ty.into());
        let (proto_event_sender, proto_event_receiver) = mpsc::channel(RECEIVED_SIZE);
        Session {
            socket,
            protocol_configs: meta.protocol_configs,
            config: meta.config,
            id: meta.id,
            timeout: meta.timeout,
            timeout_check: Some(Delay::new(Instant::now() + meta.timeout)),
            ty: meta.ty,
            next_stream: 0,
            sub_streams: HashMap::default(),
            proto_streams: HashMap::default(),
            high_write_buf: VecDeque::default(),
            write_buf: VecDeque::default(),
            read_buf: VecDeque::default(),
            proto_event_sender,
            proto_event_receiver,
            service_sender,
            service_receiver,
            delay: Arc::new(AtomicBool::new(false)),
            state: SessionState::Normal,
        }
    }

    /// select procedure
    #[inline(always)]
    fn select_procedure(
        &mut self,
        procedure: impl Future<
                Item = (
                    Framed<StreamHandle, LengthDelimitedCodec>,
                    String,
                    Option<String>,
                ),
                Error = io::Error,
            > + Send
            + 'static,
    ) {
        let event_sender = self.proto_event_sender.clone();
        let task = procedure.timeout(self.timeout).then(|result| {
            match result {
                Ok((handle, name, version)) => match version {
                    Some(version) => {
                        let send_task = event_sender.send(ProtocolEvent::Open {
                            sub_stream: Box::new(handle),
                            proto_name: name,
                            version,
                        });
                        tokio::spawn(send_task.map(|_| ()).map_err(|err| {
                            debug!("stream send back error: {:?}", err);
                        }));
                    }
                    None => {
                        debug!("Negotiation to open the protocol {} failed", name);
                        let send_task = event_sender.send(ProtocolEvent::SelectError {
                            proto_name: Some(name),
                        });
                        tokio::spawn(send_task.map(|_| ()).map_err(|err| {
                            debug!("select error send back error: {:?}", err);
                        }));
                    }
                },
                Err(err) => {
                    debug!("stream protocol select err: {:?}", err);
                    let send_task =
                        event_sender.send(ProtocolEvent::SelectError { proto_name: None });
                    tokio::spawn(send_task.map(|_| ()).map_err(|err| {
                        debug!("select error send back error: {:?}", err);
                    }));
                }
            }

            Ok(())
        });

        tokio::spawn(task);
    }

    /// After the session is established, the client is requested to open some custom protocol sub stream.
    pub fn open_proto_stream(&mut self, proto_name: &str) {
        debug!("try open proto, {}", proto_name);
        let handle = self.socket.open_stream().unwrap();
        let versions = self.protocol_configs[proto_name].support_versions.clone();
        let proto_info = ProtocolInfo::new(&proto_name, versions);

        let task = client_select(handle, proto_info);
        self.select_procedure(task);
    }

    /// Push the generated event to the Service
    #[inline]
    fn event_output(&mut self, event: SessionEvent) {
        self.read_buf.push_back(event);
        self.output();
    }

    #[inline]
    fn output(&mut self) {
        while let Some(event) = self.read_buf.pop_front() {
            if let Err(e) = self.service_sender.try_send(event) {
                if e.is_full() {
                    self.read_buf.push_front(e.into_inner());
                    self.set_delay();
                    return;
                } else {
                    error!("session send to service error: {}", e);
                }
            }
        }
    }

    fn push_back(&mut self, priority: Priority, id: ProtocolId, event: ProtocolEvent) {
        if priority.is_high() {
            self.high_write_buf.push_back((id, event));
        } else {
            self.write_buf.push_back((id, event));
        }
    }

    #[inline(always)]
    fn distribute_to_substream_process<D: Iterator<Item = (ProtocolId, ProtocolEvent)>>(
        &mut self,
        data: D,
        priority: Priority,
        block_substreams: &mut HashSet<ProtocolId>,
    ) {
        for (proto_id, event) in data {
            // Guarantee the order in which messages are sent
            if block_substreams.contains(&proto_id) {
                self.push_back(priority, proto_id, event);
                continue;
            }
            if let Some(stream_id) = self.proto_streams.get(&proto_id) {
                if let Some(sender) = self.sub_streams.get_mut(&stream_id) {
                    if let Err(e) = sender.try_send(event) {
                        if e.is_full() {
                            self.push_back(priority, proto_id, e.into_inner());
                            self.set_delay();
                            block_substreams.insert(proto_id);
                        } else {
                            debug!("session send to sub stream error: {}", e);
                        }
                    }
                };
            }
        }
    }

    #[inline]
    fn distribute_to_substream(&mut self) {
        let mut block_substreams = HashSet::new();

        let high = self.high_write_buf.split_off(0).into_iter();
        self.distribute_to_substream_process(high, Priority::High, &mut block_substreams);

        if self.sub_streams.len() > block_substreams.len() {
            let normal = self.write_buf.split_off(0).into_iter();
            self.distribute_to_substream_process(normal, Priority::Normal, &mut block_substreams);
        }

        if self.write_buf.capacity() > BUF_SHRINK_THRESHOLD {
            self.write_buf.shrink_to_fit();
        }

        if self.high_write_buf.capacity() > BUF_SHRINK_THRESHOLD {
            self.high_write_buf.shrink_to_fit();
        }
    }

    /// Handling client-initiated open protocol sub stream requests
    fn handle_sub_stream(&mut self, sub_stream: StreamHandle) {
        let proto_metas = self
            .protocol_configs
            .values()
            .map(|proto_meta| {
                let name = (proto_meta.name)(proto_meta.id);
                let proto_info = ProtocolInfo::new(&name, proto_meta.support_versions.clone());
                let select_fn = (proto_meta.select_version)();
                (name, (proto_info, select_fn))
            })
            .collect();

        let task = server_select(sub_stream, proto_metas);
        self.select_procedure(task);
    }

    /// Handling events uploaded by the protocol stream
    fn handle_stream_event(&mut self, event: ProtocolEvent) {
        match event {
            ProtocolEvent::Open {
                proto_name,
                sub_stream,
                version,
            } => {
                let proto = match self.protocol_configs.get(&proto_name) {
                    Some(proto) => proto,
                    None => unreachable!(),
                };

                let proto_id = proto.id;
                let raw_part = sub_stream.into_parts();
                let mut part = FramedParts::new(raw_part.io, (proto.codec)());
                // Replace buffered data
                part.read_buf = raw_part.read_buf;
                part.write_buf = raw_part.write_buf;
                let frame = Framed::from_parts(part);
                let (session_to_proto_sender, session_to_proto_receiver) = mpsc::channel(SEND_SIZE);
                let proto_stream = SubStream::new(
                    frame,
                    self.proto_event_sender.clone(),
                    session_to_proto_receiver,
                    self.next_stream,
                    proto_id,
                    self.config,
                );
                self.sub_streams
                    .insert(self.next_stream, session_to_proto_sender);
                self.proto_streams.insert(proto_id, self.next_stream);

                self.event_output(SessionEvent::ProtocolOpen {
                    id: self.id,
                    proto_id,
                    version,
                });
                self.next_stream += 1;

                debug!("session [{}] proto [{}] open", self.id, proto_id);

                tokio::spawn(proto_stream.for_each(|_| Ok(())));
            }
            ProtocolEvent::Close { id, proto_id } => {
                debug!("session [{}] proto [{}] closed", self.id, proto_id);
                self.sub_streams.remove(&id);
                self.proto_streams.remove(&proto_id);
                self.event_output(SessionEvent::ProtocolClose {
                    id: self.id,
                    proto_id,
                });
            }
            ProtocolEvent::Message { data, proto_id, .. } => {
                debug!("get proto [{}] data len: {}", proto_id, data.len());
                self.event_output(SessionEvent::ProtocolMessage {
                    id: self.id,
                    proto_id,
                    data,
                    priority: Priority::Normal,
                })
            }
            ProtocolEvent::SelectError { proto_name } => {
                self.event_output(SessionEvent::ProtocolSelectError {
                    id: self.id,
                    proto_name,
                })
            }
            ProtocolEvent::Error {
                proto_id, error, ..
            } => {
                debug!("Codec error: {:?}", error);
                self.event_output(SessionEvent::ProtocolError {
                    id: self.id,
                    proto_id,
                    error,
                })
            }
        }
    }

    /// Handling events send by the service
    fn handle_session_event(&mut self, event: SessionEvent) {
        match event {
            SessionEvent::ProtocolMessage {
                proto_id,
                data,
                priority,
                ..
            } => {
                if let Some(stream_id) = self.proto_streams.get(&proto_id) {
                    let event = ProtocolEvent::Message {
                        id: *stream_id,
                        proto_id,
                        priority,
                        data,
                    };
                    self.push_back(priority, proto_id, event);
                } else {
                    trace!("protocol {} not ready", proto_id);
                }
            }
            SessionEvent::SessionClose { .. } => {
                if self.sub_streams.is_empty() {
                    // if no proto open, just close session
                    self.close_session();
                } else {
                    self.state = SessionState::LocalClose;
                    self.close_all_proto();
                }
            }
            SessionEvent::ProtocolOpen { proto_id, .. } => {
                if self.proto_streams.contains_key(&proto_id) {
                    debug!("proto [{}] has been open", proto_id);
                } else {
                    let name = self.protocol_configs.values().find_map(|meta| {
                        if meta.id == proto_id {
                            Some((meta.name)(meta.id))
                        } else {
                            None
                        }
                    });
                    match name {
                        Some(name) => self.open_proto_stream(&name),
                        None => debug!("This protocol [{}] is not supported", proto_id),
                    }
                }
            }
            SessionEvent::ProtocolClose { proto_id, .. } => {
                if !self.proto_streams.contains_key(&proto_id) {
                    debug!("proto [{}] has been closed", proto_id);
                } else {
                    self.write_buf.push_back((
                        proto_id,
                        ProtocolEvent::Close {
                            id: self.proto_streams[&proto_id],
                            proto_id,
                        },
                    ));
                }
            }
            _ => (),
        }
        self.distribute_to_substream();
    }

    fn recv_substreams(&mut self) -> Option<()> {
        loop {
            // Local close means user doesn't want any message from this session
            // But when remote close, we should try my best to accept all data as much as possible
            if self.state.is_local_close() {
                break;
            }

            if self.read_buf.len() > self.config.recv_event_size() {
                self.set_delay();
                break;
            }

            match self.proto_event_receiver.poll() {
                Ok(Async::Ready(Some(event))) => self.handle_stream_event(event),
                Ok(Async::Ready(None)) => {
                    // Drop by self
                    return None;
                }
                Ok(Async::NotReady) => break,
                Err(err) => {
                    debug!("receive proto event error: {:?}", err);
                    break;
                }
            }
        }

        Some(())
    }

    fn recv_service(&mut self) -> Option<()> {
        loop {
            if !self.state.is_normal() {
                break;
            }

            if self.write_buf.len() > self.config.send_event_size() {
                self.set_delay();
                break;
            }

            match self.service_receiver.poll() {
                Ok(Async::Ready(Some(event))) => self.handle_session_event(event),
                Ok(Async::Ready(None)) => {
                    // Must drop by service
                    self.state = SessionState::LocalClose;
                    self.clean();
                    return None;
                }
                Ok(Async::NotReady) => break,
                Err(err) => {
                    warn!("receive service message error: {:?}", err);
                    break;
                }
            }
        }

        Some(())
    }

    /// Try close all protocol
    #[inline]
    fn close_all_proto(&mut self) {
        for (proto_id, stream_id) in self.proto_streams.iter() {
            self.write_buf.push_back((
                *proto_id,
                ProtocolEvent::Close {
                    id: *stream_id,
                    proto_id: *proto_id,
                },
            ));
        }
        self.distribute_to_substream();
    }

    /// Close session
    fn close_session(&mut self) {
        self.read_buf
            .push_back(SessionEvent::SessionClose { id: self.id });
        let events = self.read_buf.split_off(0);

        tokio::spawn(
            self.service_sender
                .clone()
                .send_all(iter_ok(events))
                .map(|_| ())
                .map_err(|e| error!("session close event send to service error: {:?}", e)),
        );
        self.clean();
    }

    /// Clean env
    fn clean(&mut self) {
        self.sub_streams.clear();
        self.service_receiver.close();
        self.proto_event_receiver.close();

        let _ = self.socket.shutdown();
    }

    #[inline]
    fn flush(&mut self) {
        self.distribute_to_substream();
        self.output();
    }

    #[inline]
    fn set_delay(&mut self) {
        // Why use `delay` instead of `notify`?
        //
        // In fact, on machines that can use multi-core normally, there is almost no problem with the `notify` behavior,
        // and even the efficiency will be higher.
        //
        // However, if you are on a single-core bully machine, `notify` may have a very amazing starvation behavior.
        //
        // Under a single-core machine, `notify` may fall into the loop of infinitely preemptive CPU, causing starvation.
        if !self.delay.load(Ordering::Acquire) {
            self.delay.store(true, Ordering::Release);
            let notify = futures::task::current();
            let delay = self.delay.clone();
            let delay_task = Delay::new(Instant::now() + DELAY_TIME).then(move |_| {
                notify.notify();
                delay.store(false, Ordering::Release);
                Ok(())
            });
            tokio::spawn(delay_task);
        }
    }
}

impl<T> Stream for Session<T>
where
    T: AsyncRead + AsyncWrite,
{
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        debug!(
            "session [{}], [{:?}], proto count [{}], state: {:?} ",
            self.id,
            self.ty,
            self.sub_streams.len(),
            self.state
        );

        // double check here
        if self.state.is_local_close() {
            return Ok(Async::Ready(None));
        }

        if !self.read_buf.is_empty()
            || !self.write_buf.is_empty()
            || !self.high_write_buf.is_empty()
        {
            self.flush();
        }

        if let Some(mut check) = self.timeout_check.take() {
            match check.poll() {
                Ok(Async::Ready(_)) => {
                    if self.sub_streams.is_empty() {
                        self.event_output(SessionEvent::SessionTimeout { id: self.id });
                        self.state = SessionState::LocalClose;
                    }
                }
                Ok(Async::NotReady) => self.timeout_check = Some(check),
                Err(e) => debug!("timeout check error: {}", e),
            }
        }

        loop {
            if !self.state.is_normal() {
                break;
            }
            match self.socket.poll() {
                Ok(Async::Ready(Some(sub_stream))) => self.handle_sub_stream(sub_stream),
                Ok(Async::Ready(None)) => {
                    self.state = SessionState::RemoteClose;
                    break;
                }
                Ok(Async::NotReady) => break,
                Err(err) => {
                    debug!("session poll error: {:?}", err);
                    self.state = SessionState::RemoteClose;

                    match err.kind() {
                        ErrorKind::BrokenPipe
                        | ErrorKind::ConnectionAborted
                        | ErrorKind::ConnectionReset
                        | ErrorKind::NotConnected
                        | ErrorKind::UnexpectedEof => (),
                        _ => {
                            self.event_output(SessionEvent::MuxerError {
                                id: self.id,
                                error: err.into(),
                            });
                        }
                    }

                    break;
                }
            }
        }

        if self.recv_substreams().is_none() {
            return Ok(Async::Ready(None));
        }

        if self.recv_service().is_none() {
            return Ok(Async::Ready(None));
        }

        match self.state {
            SessionState::LocalClose => {
                self.close_session();
                return Ok(Async::Ready(None));
            }
            SessionState::RemoteClose => {
                // try close all protocol stream, and then close session
                if self.proto_streams.is_empty() {
                    self.close_session();
                    return Ok(Async::Ready(None));
                } else {
                    self.close_all_proto();
                }
            }
            SessionState::Normal => (),
        }

        Ok(Async::NotReady)
    }
}

pub(crate) struct SessionMeta {
    config: Config,
    id: SessionId,
    protocol_configs: HashMap<String, Arc<Meta>>,
    ty: SessionType,
    // remote_address: ::std::net::SocketAddr,
    // remote_public_key: Option<PublicKey>,
    timeout: Duration,
}

impl SessionMeta {
    pub fn new(id: SessionId, ty: SessionType, timeout: Duration) -> Self {
        SessionMeta {
            config: Config::default(),
            id,
            ty,
            protocol_configs: HashMap::new(),
            timeout,
        }
    }

    pub fn protocol(mut self, config: HashMap<String, Arc<Meta>>) -> Self {
        self.protocol_configs = config;
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }
}

/// Session state
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum SessionState {
    /// Close by remote, accept all data as much as possible
    RemoteClose,
    /// Close by self, don't receive any more
    LocalClose,
    /// Normal communication
    Normal,
}

impl SessionState {
    #[inline]
    fn is_local_close(self) -> bool {
        match self {
            SessionState::LocalClose => true,
            _ => false,
        }
    }

    #[inline]
    fn is_normal(self) -> bool {
        match self {
            SessionState::Normal => true,
            _ => false,
        }
    }
}
