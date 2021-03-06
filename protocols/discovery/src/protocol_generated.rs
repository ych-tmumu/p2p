// automatically generated by the FlatBuffers compiler, do not modify


pub mod p2p {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
pub mod discovery {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DiscoveryPayload {
  NONE = 0,
  GetNodes = 1,
  Nodes = 2,

}

const ENUM_MIN_DISCOVERY_PAYLOAD: u8 = 0;
const ENUM_MAX_DISCOVERY_PAYLOAD: u8 = 2;

impl<'a> flatbuffers::Follow<'a> for DiscoveryPayload {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for DiscoveryPayload {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const DiscoveryPayload;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const DiscoveryPayload;
    unsafe { *p }
  }
}

impl flatbuffers::Push for DiscoveryPayload {
    type Output = DiscoveryPayload;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<DiscoveryPayload>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_DISCOVERY_PAYLOAD:[DiscoveryPayload; 3] = [
  DiscoveryPayload::NONE,
  DiscoveryPayload::GetNodes,
  DiscoveryPayload::Nodes
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_DISCOVERY_PAYLOAD:[&'static str; 3] = [
    "NONE",
    "GetNodes",
    "Nodes"
];

pub fn enum_name_discovery_payload(e: DiscoveryPayload) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_DISCOVERY_PAYLOAD[index]
}

pub struct DiscoveryPayloadUnionTableOffset {}
pub enum DiscoveryMessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct DiscoveryMessage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for DiscoveryMessage<'a> {
    type Inner = DiscoveryMessage<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> DiscoveryMessage<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        DiscoveryMessage {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args DiscoveryMessageArgs) -> flatbuffers::WIPOffset<DiscoveryMessage<'bldr>> {
      let mut builder = DiscoveryMessageBuilder::new(_fbb);
      if let Some(x) = args.payload { builder.add_payload(x); }
      builder.add_payload_type(args.payload_type);
      builder.finish()
    }

    pub const VT_PAYLOAD_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_PAYLOAD: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn payload_type(&self) -> DiscoveryPayload {
    self._tab.get::<DiscoveryPayload>(DiscoveryMessage::VT_PAYLOAD_TYPE, Some(DiscoveryPayload::NONE)).unwrap()
  }
  #[inline]
  pub fn payload(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(DiscoveryMessage::VT_PAYLOAD, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn payload_as_get_nodes(&'a self) -> Option<GetNodes> {
    if self.payload_type() == DiscoveryPayload::GetNodes {
      self.payload().map(|u| GetNodes::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn payload_as_nodes(&'a self) -> Option<Nodes> {
    if self.payload_type() == DiscoveryPayload::Nodes {
      self.payload().map(|u| Nodes::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct DiscoveryMessageArgs {
    pub payload_type: DiscoveryPayload,
    pub payload: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for DiscoveryMessageArgs {
    #[inline]
    fn default() -> Self {
        DiscoveryMessageArgs {
            payload_type: DiscoveryPayload::NONE,
            payload: None,
        }
    }
}
pub struct DiscoveryMessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> DiscoveryMessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_payload_type(&mut self, payload_type: DiscoveryPayload) {
    self.fbb_.push_slot::<DiscoveryPayload>(DiscoveryMessage::VT_PAYLOAD_TYPE, payload_type, DiscoveryPayload::NONE);
  }
  #[inline]
  pub fn add_payload(&mut self, payload: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(DiscoveryMessage::VT_PAYLOAD, payload);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> DiscoveryMessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    DiscoveryMessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<DiscoveryMessage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum GetNodesOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct GetNodes<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for GetNodes<'a> {
    type Inner = GetNodes<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> GetNodes<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        GetNodes {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args GetNodesArgs) -> flatbuffers::WIPOffset<GetNodes<'bldr>> {
      let mut builder = GetNodesBuilder::new(_fbb);
      builder.add_count(args.count);
      builder.add_version(args.version);
      builder.add_listen_port(args.listen_port);
      builder.finish()
    }

    pub const VT_VERSION: flatbuffers::VOffsetT = 4;
    pub const VT_COUNT: flatbuffers::VOffsetT = 6;
    pub const VT_LISTEN_PORT: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn version(&self) -> u32 {
    self._tab.get::<u32>(GetNodes::VT_VERSION, Some(0)).unwrap()
  }
  #[inline]
  pub fn count(&self) -> u32 {
    self._tab.get::<u32>(GetNodes::VT_COUNT, Some(0)).unwrap()
  }
  #[inline]
  pub fn listen_port(&self) -> u16 {
    self._tab.get::<u16>(GetNodes::VT_LISTEN_PORT, Some(0)).unwrap()
  }
}

pub struct GetNodesArgs {
    pub version: u32,
    pub count: u32,
    pub listen_port: u16,
}
impl<'a> Default for GetNodesArgs {
    #[inline]
    fn default() -> Self {
        GetNodesArgs {
            version: 0,
            count: 0,
            listen_port: 0,
        }
    }
}
pub struct GetNodesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> GetNodesBuilder<'a, 'b> {
  #[inline]
  pub fn add_version(&mut self, version: u32) {
    self.fbb_.push_slot::<u32>(GetNodes::VT_VERSION, version, 0);
  }
  #[inline]
  pub fn add_count(&mut self, count: u32) {
    self.fbb_.push_slot::<u32>(GetNodes::VT_COUNT, count, 0);
  }
  #[inline]
  pub fn add_listen_port(&mut self, listen_port: u16) {
    self.fbb_.push_slot::<u16>(GetNodes::VT_LISTEN_PORT, listen_port, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> GetNodesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    GetNodesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<GetNodes<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum NodesOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Nodes<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Nodes<'a> {
    type Inner = Nodes<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Nodes<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Nodes {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args NodesArgs<'args>) -> flatbuffers::WIPOffset<Nodes<'bldr>> {
      let mut builder = NodesBuilder::new(_fbb);
      if let Some(x) = args.items { builder.add_items(x); }
      builder.add_announce(args.announce);
      builder.finish()
    }

    pub const VT_ANNOUNCE: flatbuffers::VOffsetT = 4;
    pub const VT_ITEMS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn announce(&self) -> bool {
    self._tab.get::<bool>(Nodes::VT_ANNOUNCE, Some(false)).unwrap()
  }
  #[inline]
  pub fn items(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Node<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Node<'a>>>>>(Nodes::VT_ITEMS, None)
  }
}

pub struct NodesArgs<'a> {
    pub announce: bool,
    pub items: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Node<'a >>>>>,
}
impl<'a> Default for NodesArgs<'a> {
    #[inline]
    fn default() -> Self {
        NodesArgs {
            announce: false,
            items: None,
        }
    }
}
pub struct NodesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> NodesBuilder<'a, 'b> {
  #[inline]
  pub fn add_announce(&mut self, announce: bool) {
    self.fbb_.push_slot::<bool>(Nodes::VT_ANNOUNCE, announce, false);
  }
  #[inline]
  pub fn add_items(&mut self, items: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Node<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Nodes::VT_ITEMS, items);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> NodesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    NodesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Nodes<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum NodeOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Node<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Node<'a> {
    type Inner = Node<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Node<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Node {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args NodeArgs<'args>) -> flatbuffers::WIPOffset<Node<'bldr>> {
      let mut builder = NodeBuilder::new(_fbb);
      if let Some(x) = args.addresses { builder.add_addresses(x); }
      builder.finish()
    }

    pub const VT_ADDRESSES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn addresses(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Bytes<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Bytes<'a>>>>>(Node::VT_ADDRESSES, None)
  }
}

pub struct NodeArgs<'a> {
    pub addresses: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Bytes<'a >>>>>,
}
impl<'a> Default for NodeArgs<'a> {
    #[inline]
    fn default() -> Self {
        NodeArgs {
            addresses: None,
        }
    }
}
pub struct NodeBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> NodeBuilder<'a, 'b> {
  #[inline]
  pub fn add_addresses(&mut self, addresses: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Bytes<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Node::VT_ADDRESSES, addresses);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> NodeBuilder<'a, 'b> {
    let start = _fbb.start_table();
    NodeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Node<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum BytesOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Bytes<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Bytes<'a> {
    type Inner = Bytes<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Bytes<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Bytes {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args BytesArgs<'args>) -> flatbuffers::WIPOffset<Bytes<'bldr>> {
      let mut builder = BytesBuilder::new(_fbb);
      if let Some(x) = args.seq { builder.add_seq(x); }
      builder.finish()
    }

    pub const VT_SEQ: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn seq(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Bytes::VT_SEQ, None).map(|v| v.safe_slice())
  }
}

pub struct BytesArgs<'a> {
    pub seq: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
}
impl<'a> Default for BytesArgs<'a> {
    #[inline]
    fn default() -> Self {
        BytesArgs {
            seq: None,
        }
    }
}
pub struct BytesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BytesBuilder<'a, 'b> {
  #[inline]
  pub fn add_seq(&mut self, seq: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Bytes::VT_SEQ, seq);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BytesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    BytesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Bytes<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod Discovery
}  // pub mod P2P

