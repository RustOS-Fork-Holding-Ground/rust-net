use std::collections::hash_map::HashMap;
use std::fmt;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use data_link::interface as dl;

use self::strategy::RoutingTable;

pub mod control;
pub mod packet;
pub mod send;
pub mod receive;
pub mod strategy;


#[derive(PartialEq, PartialOrd, Eq, Ord,
         Copy, Clone, Hash, Debug)]
pub struct Addr(pub [u8; 4]);


impl fmt::Display for Addr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let [a0, a1, a2, a3] = self.0;
    write!(f, "{}.{}.{}.{}", a0, a1, a2, a3)
  }
}

impl FromStr for Addr {
  type Err = ();

  fn from_str(s: &str) -> Result<Addr, ()> {
    let mut quad: [Result<u8, ()>; 4] = [Err(()); 4];
    let iter = s.trim().split('.').map(FromStr::from_str).map(|r| r.map_err(|_| ()));

    for (mut ptr, val) in quad.iter_mut().zip(iter)
    {
      *ptr = val;
    }

    let [a0, a1, a2, a3] = quad;

    Ok(Addr([a0?, a1?, a2?, a3?]))
  }
}


#[inline]
pub fn parse_addr(&[a, b, c, d]: &[u8; 4]) -> Addr {
  Addr([a, b, c, d])
}

#[inline]
pub fn parse_addr_unsafe(b: &[u8]) -> Addr {
  Addr([b[0], b[1], b[2], b[3]])
}

// TODO: remove
#[inline]
pub fn write_addr(Addr(slice): Addr) -> [u8; 4] {
  slice
}



// key:    adjacent ip (next hop)
// value:  index to InterfaceRow (see below)
pub type InterfaceTable = HashMap<Addr, usize>;

pub struct InterfaceRow<'a, E> {
  pub local_ip:  Addr,
  pub interface: RwLock<Box<dl::Interface<'a, Error=E> + Send + Sync + 'a>>,
}

// TODO: use Box<[u8]> instead of Vec<u8>
// TODO: real network card may consolidate multiple packets per interrupt
pub type Handler<'a> = super::misc::interface::Handler<'a, packet::V>;

pub type ProtocolTable<'a> = Vec<Vec<Handler<'a>>>;

pub struct State<'a, A, E> where A: RoutingTable<'a> + 'a
{
  pub interfaces:        Vec<InterfaceRow<'a, E>>,
  pub neighbors:         InterfaceTable,
  pub routes:            A,
  pub protocol_handlers: RwLock<ProtocolTable<'a>>,
  // Identification counter? increased with each packet sent out,
  // used in Identification header for fragmentation purposes
}

impl<'a, RT, DE> State<'a, RT, DE>
  where RT: RoutingTable<'a> + 'a,
        DE: fmt::Debug + 'a

{
  pub fn new(interfaces: Vec<InterfaceRow<'a, DE>>,
             neighbors: InterfaceTable)
             -> Arc<State<'a, RT, DE>>
  {
    let routes: RT = RoutingTable::init(neighbors.keys().map(|x| *x));

    let state: Arc<State<'a, RT, DE>> = Arc::new(State {
      routes:            routes,
      neighbors:         neighbors,
      interfaces:        interfaces,
      // handlers are not clonable, so the nice ways of doing this do not work
      protocol_handlers: RwLock::new(vec![
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],

        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],

        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],

        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],


        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],

        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],

        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],

        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![],   vec![], vec![], vec![], vec![]]),
    });

    for &InterfaceRow { ref interface, .. } in state.interfaces.iter() {
      use self::receive::make_receive_callback;
      interface.write().unwrap()
        .update_recv_handler(make_receive_callback::<RT, DE>(state.clone()));
    }

    RoutingTable::monitor(state.clone());

    state
  }

  /// Returns dl::Interface struct for the requested interface
  pub fn get_interface(&self, interface_ix: usize)
                      -> Option<&InterfaceRow<'a, DE>>
  {
    self.interfaces.as_slice().get(interface_ix)
  }
}
