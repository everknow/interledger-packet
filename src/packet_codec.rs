use bytes::{Buf, BufMut, BytesMut};
use crate::{Packet};
use std::io;
use tokio_util::codec::{Decoder, Encoder};
use std::convert::TryFrom;
use log::debug;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct PacketCodec(());

impl PacketCodec {
  /// Creates a new `PacketCodec` for shipping around raw bytes.
  pub fn new() -> PacketCodec {
    PacketCodec(())
  }
}

impl Encoder<Packet> for PacketCodec {
  type Error = io::Error;

  fn encode(&mut self, packet: Packet, buf: &mut BytesMut) -> Result<(), io::Error> {

    match packet {
      Packet::Prepare(prepare) => {
        let data = prepare.data();
        buf.reserve(data.len());
        buf.put(data);
      }
      Packet::Fulfill(fulfill) => {
        let data = fulfill.data();
        buf.reserve(data.len());
        buf.put(data);
      }
      Packet::Reject(reject) => {
        let data = reject.data();
        buf.reserve(data.len());
        buf.put(data);
      }
    };

    // buf.reserve(data.len());
    // buf.put(data);
    Ok(())
  }
}

impl Decoder for PacketCodec {
  type Item = Packet;
  type Error = io::Error;

  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Packet>, io::Error> {
    if src.len() > 0 {
      match Packet::try_from(src.clone()) {
        Ok(Packet::Prepare(prepare)) => {
          debug!("size: {}, buf_before: {:?}", prepare.packet_length.unwrap(), src.clone());
          src.advance(prepare.packet_length.unwrap());
          debug!("buf_after: {:?}, buf_size: {}", src.clone(), src.capacity());
          Ok(Some(Packet::Prepare(prepare)))
        }
        Ok(Packet::Fulfill(fulfill)) => {
          src.advance(fulfill.packet_length.unwrap());
          Ok(Some(Packet::Fulfill(fulfill)))
        }
        Ok(Packet::Reject(reject)) => {
          src.advance(reject.packet_length.unwrap());
          Ok(Some(Packet::Reject(reject)))
        }
        Err(_err) => {
          // eprintln!("error {:?}",src.clone());
          Ok(None)
        }
      }
    } else{
        Ok(None)
    }
  }
}
