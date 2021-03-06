#![feature(box_syntax)]

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate misc;
extern crate interface as dl;
extern crate udp_mock;

use std::io;

use std::sync::{Arc, Barrier};
use std::str::from_utf8;
use std::string::String;
use std::str::FromStr;

use misc::SenderClosure;
use udp_mock::*;


fn talk_to_self_channel_helper(num_threads: usize) {
  use std::sync::mpsc::*;

  let (l1, a1) = Listener::new_loopback(num_threads).unwrap();
  let (l2, a2) = Listener::new_loopback(num_threads).unwrap();

  let (tx1, rx1) = channel::<(dl::Packet,)>();
  let (tx2, rx2) = channel::<(dl::Packet,)>();

  const M1: &'static str = "Hey Josh!";
  const M2: &'static str = "Hey Cody!";

  let interface1 = Interface::new(&l1, a2, box SenderClosure::new(tx1));
  let interface2 = Interface::new(&l2, a1, box SenderClosure::new(tx2));

  dl::Interface::send(&interface1, String::from_str(M2).unwrap().into()).unwrap();
  dl::Interface::send(&interface2, String::from_str(M1).unwrap().into()).unwrap();

  let (packet_1,) = rx1.recv().unwrap();
  assert_eq!(packet_1.as_slice(), M1.as_bytes());
  debug!("Got the first packet");

  let (packet_2,) = rx2.recv().unwrap();
  assert_eq!(packet_2.as_slice(), M2.as_bytes());
  debug!("Got the second packet");
}

#[test]
fn talk_to_self_channel() {
  talk_to_self_channel_helper(1);
}
#[test]
fn talk_to_self_channel_parallel() {
  talk_to_self_channel_helper(4);
}

fn talk_to_self_callback_helper(num_threads: usize) {
  fn mk_callback(barrier: Arc<Barrier>, msg: &'static str) -> dl::Handler {
    box move |packet: dl::Packet| {
      debug!("got packet: {}", from_utf8(packet.as_slice()).unwrap());
      debug!("matching against: {}", msg);
      assert_eq!(packet.as_slice(), msg.as_bytes());
      barrier.wait();
    }
  }

  let barrier = Arc::new(Barrier::new(3));

  let (l1, a1) = Listener::new_loopback(num_threads).unwrap();
  let (l2, a2) = Listener::new_loopback(num_threads).unwrap();

  const M1: &'static str = "Hey Josh!";
  const M2: &'static str = "Hey Cody!";

  let interface1 = Interface::new(&l1, a2, mk_callback(barrier.clone(), M1));
  let interface2 = Interface::new(&l2, a1, mk_callback(barrier.clone(), M2));

  dl::Interface::send(&interface1, String::from_str(M2).unwrap().into()).unwrap();
  dl::Interface::send(&interface2, String::from_str(M1).unwrap().into()).unwrap();

  barrier.wait();
}

#[test]
fn talk_to_self_callback() {
  talk_to_self_callback_helper(1);
}

#[test]
fn talk_to_self_callback_parallel() {
  talk_to_self_callback_helper(4);
}

#[test]
fn disable_then_cant_send() {

  fn inner() -> io::Result<()> {
    let (l, a) = Listener::new_loopback(1).unwrap();
    let mut i = Interface::new(&l, a, box |_| {});

    dl::Interface::disable(&mut i);

    match dl::Interface::send(&i, Vec::new()).unwrap_err() {
      dl::Error::Disabled => (),
      _ => panic!("was not disabled")
    }

    Ok(())
  }
  inner().unwrap();
}
