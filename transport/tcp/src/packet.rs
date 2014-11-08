use std::io::net::ip::{IpAddr, Ipv4Addr};
use std::io::{
  BufReader,
  BufWriter,
  MemWriter,
  SeekSet,

  IoError,
  IoResult,
};
use std::mem::{transmute, size_of};

pub struct TcpPacket {
  pub src_addr: IpAddr,
  pub dst_addr: IpAddr,
  protocol: u8,
  tcp_len:  u16,
  data:     Vec<u8>
}

impl TcpPacket {

  // 4-tuple info
  pub fn get_src_addr(&self) -> IpAddr {
    self.src_addr
  }
  pub fn get_src_port(&self) -> u16 {
    //TODO:
    0
  } 
  pub fn set_src_port(&mut self, port: u16) {
    //TODO:
  }
  pub fn get_dst_addr(&self) -> IpAddr {
    self.dst_addr
  }
  pub fn get_dst_port(&self) -> u16 {
    //TODO:
    0
  }
  pub fn set_dst_port(&mut self, port: u16) {
    //TODO
  }

  // Control Flags
  pub fn is_ack(&self) -> bool {
    //TODO:
    false
  }
  pub fn set_ack(&mut self) {
    //TODO
  }
  pub fn is_syn(&self) -> bool {
    //TODO:
    false
  }
  pub fn set_syn(&mut self) {
    //TODO
  }
  pub fn is_fin(&self) -> bool {
    //TODO:
    false
  }
  pub fn set_fin(&mut self) {
    //TODO
  }

  // Not sure if this is used
  pub fn is_rst(&self) -> bool {
    //TODO:
    false
  }
  pub fn set_rst(&mut self) {
    //TODO
  }

  // Other TCP data
  pub fn get_hdr_size(&self) -> u8 { // really u8
    //TODO: 
    0
  }

  // Sequence Number Ops
  pub fn get_seq_num(&self) -> u32 {
    //TODO:
    0
  }
  pub fn set_seq_num(&mut self, seq_num: u16) {
    //TODO:
  }

  // Acknowledgement Number Ops
  pub fn get_ack_num(&self) -> u32 {
    //TODO:
    assert!(self.is_ack());
    0
  }
  pub fn set_ack_num(&mut self, ack_num: u16) {
    //TODO:
  }

  // Checksum Ops
  pub fn get_checksum(&self) -> u16 {
    //TODO: 
    0
  }
  pub fn compute_checksum(&self) -> u16 {
    //TODO:
    0
  }
  pub fn set_checksum(&mut self, checksum: u16) {
    //TODO:
  }

}

#[inline]
pub fn parse_ip(&[a, b, c, d]: &[u8, ..4]) -> IpAddr {
  Ipv4Addr(a, b, c, d)
}

#[inline]
pub fn write_ip(addr: IpAddr) -> [u8, ..4] {
  match addr {
    Ipv4Addr(a, b, c, d) => [a, b, c, d],
    _                    => panic!("no ipv6 yet"),
  }
}