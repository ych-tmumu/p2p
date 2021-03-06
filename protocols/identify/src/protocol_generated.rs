// automatically generated by the FlatBuffers compiler, do not modify


pub mod p2p {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
pub mod identify {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IdentifyPayload {
  NONE = 0,
  ListenAddrs = 1,
  ObservedAddr = 2,

}

const ENUM_MIN_IDENTIFY_PAYLOAD: u8 = 0;
const ENUM_MAX_IDENTIFY_PAYLOAD: u8 = 2;

impl<'a> flatbuffers::Follow<'a> for IdentifyPayload {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for IdentifyPayload {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const IdentifyPayload;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const IdentifyPayload;
    unsafe { *p }
  }
}

impl flatbuffers::Push for IdentifyPayload {
    type Output = IdentifyPayload;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<IdentifyPayload>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_IDENTIFY_PAYLOAD:[IdentifyPayload; 3] = [
  IdentifyPayload::NONE,
  IdentifyPayload::ListenAddrs,
  IdentifyPayload::ObservedAddr
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_IDENTIFY_PAYLOAD:[&'static str; 3] = [
    "NONE",
    "ListenAddrs",
    "ObservedAddr"
];

pub fn enum_name_identify_payload(e: IdentifyPayload) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_IDENTIFY_PAYLOAD[index]
}

pub struct IdentifyPayloadUnionTableOffset {}
pub enum IdentifyMessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct IdentifyMessage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for IdentifyMessage<'a> {
    type Inner = IdentifyMessage<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> IdentifyMessage<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        IdentifyMessage {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args IdentifyMessageArgs) -> flatbuffers::WIPOffset<IdentifyMessage<'bldr>> {
      let mut builder = IdentifyMessageBuilder::new(_fbb);
      if let Some(x) = args.payload { builder.add_payload(x); }
      builder.add_payload_type(args.payload_type);
      builder.finish()
    }

    pub const VT_PAYLOAD_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_PAYLOAD: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn payload_type(&self) -> IdentifyPayload {
    self._tab.get::<IdentifyPayload>(IdentifyMessage::VT_PAYLOAD_TYPE, Some(IdentifyPayload::NONE)).unwrap()
  }
  #[inline]
  pub fn payload(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(IdentifyMessage::VT_PAYLOAD, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn payload_as_listen_addrs(&'a self) -> Option<ListenAddrs> {
    if self.payload_type() == IdentifyPayload::ListenAddrs {
      self.payload().map(|u| ListenAddrs::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn payload_as_observed_addr(&'a self) -> Option<ObservedAddr> {
    if self.payload_type() == IdentifyPayload::ObservedAddr {
      self.payload().map(|u| ObservedAddr::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct IdentifyMessageArgs {
    pub payload_type: IdentifyPayload,
    pub payload: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for IdentifyMessageArgs {
    #[inline]
    fn default() -> Self {
        IdentifyMessageArgs {
            payload_type: IdentifyPayload::NONE,
            payload: None,
        }
    }
}
pub struct IdentifyMessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> IdentifyMessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_payload_type(&mut self, payload_type: IdentifyPayload) {
    self.fbb_.push_slot::<IdentifyPayload>(IdentifyMessage::VT_PAYLOAD_TYPE, payload_type, IdentifyPayload::NONE);
  }
  #[inline]
  pub fn add_payload(&mut self, payload: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(IdentifyMessage::VT_PAYLOAD, payload);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> IdentifyMessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    IdentifyMessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<IdentifyMessage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum ListenAddrsOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct ListenAddrs<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ListenAddrs<'a> {
    type Inner = ListenAddrs<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> ListenAddrs<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ListenAddrs {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ListenAddrsArgs<'args>) -> flatbuffers::WIPOffset<ListenAddrs<'bldr>> {
      let mut builder = ListenAddrsBuilder::new(_fbb);
      if let Some(x) = args.addrs { builder.add_addrs(x); }
      builder.finish()
    }

    pub const VT_ADDRS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn addrs(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Address<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Address<'a>>>>>(ListenAddrs::VT_ADDRS, None)
  }
}

pub struct ListenAddrsArgs<'a> {
    pub addrs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Address<'a >>>>>,
}
impl<'a> Default for ListenAddrsArgs<'a> {
    #[inline]
    fn default() -> Self {
        ListenAddrsArgs {
            addrs: None,
        }
    }
}
pub struct ListenAddrsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ListenAddrsBuilder<'a, 'b> {
  #[inline]
  pub fn add_addrs(&mut self, addrs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Address<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ListenAddrs::VT_ADDRS, addrs);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ListenAddrsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ListenAddrsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ListenAddrs<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum ObservedAddrOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct ObservedAddr<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ObservedAddr<'a> {
    type Inner = ObservedAddr<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> ObservedAddr<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ObservedAddr {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ObservedAddrArgs<'args>) -> flatbuffers::WIPOffset<ObservedAddr<'bldr>> {
      let mut builder = ObservedAddrBuilder::new(_fbb);
      if let Some(x) = args.addr { builder.add_addr(x); }
      builder.finish()
    }

    pub const VT_ADDR: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn addr(&self) -> Option<Address<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Address<'a>>>(ObservedAddr::VT_ADDR, None)
  }
}

pub struct ObservedAddrArgs<'a> {
    pub addr: Option<flatbuffers::WIPOffset<Address<'a >>>,
}
impl<'a> Default for ObservedAddrArgs<'a> {
    #[inline]
    fn default() -> Self {
        ObservedAddrArgs {
            addr: None,
        }
    }
}
pub struct ObservedAddrBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ObservedAddrBuilder<'a, 'b> {
  #[inline]
  pub fn add_addr(&mut self, addr: flatbuffers::WIPOffset<Address<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Address>>(ObservedAddr::VT_ADDR, addr);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ObservedAddrBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ObservedAddrBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ObservedAddr<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum AddressOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Address<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Address<'a> {
    type Inner = Address<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Address<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Address {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args AddressArgs<'args>) -> flatbuffers::WIPOffset<Address<'bldr>> {
      let mut builder = AddressBuilder::new(_fbb);
      if let Some(x) = args.bytes { builder.add_bytes(x); }
      builder.finish()
    }

    pub const VT_BYTES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn bytes(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Address::VT_BYTES, None).map(|v| v.safe_slice())
  }
}

pub struct AddressArgs<'a> {
    pub bytes: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
}
impl<'a> Default for AddressArgs<'a> {
    #[inline]
    fn default() -> Self {
        AddressArgs {
            bytes: None,
        }
    }
}
pub struct AddressBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AddressBuilder<'a, 'b> {
  #[inline]
  pub fn add_bytes(&mut self, bytes: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Address::VT_BYTES, bytes);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AddressBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AddressBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Address<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod Identify
}  // pub mod P2P

