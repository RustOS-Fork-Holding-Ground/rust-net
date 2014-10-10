use std::collections::hashmap::HashMap;
use std::io::net::ip::IpAddr;
use std::iter::FromIterator;
use std::mem::size_of;
use std::sync::{Arc, RWLock};

use interface::Handler;

use packet::ipv4::V as Ip;

use data_link::{DLInterface, DLHandler};


pub mod control;
pub mod send;
pub mod receive;


pub struct RoutingRow {
    pub cost:      u8,          // How many hops
    pub next_hop:  IpAddr,      // Which link-layer interface to use
    pub learned_from: IpAddr,   // Who we learned this route from (used in split-horizon)
}

// key: IP we want to reach
pub type RoutingTable = HashMap<IpAddr, RoutingRow>;

// key:   adjacent ip (next hop)
// value:  which one of our IPs we put as the src address
//         which interface we send the packet with
//pub type InterfaceTable = HashMap<IpAddr, (IpAddr, Box<DLInterface+'static>)>;
pub type InterfaceTable = HashMap<IpAddr, uint>;

pub type InterfaceRow = (IpAddr, IpAddr, RWLock<Box<DLInterface + Send + Sync + 'static>>);

// TODO: use Box<[u8]> instead of Vec<u8>
// TODO: real network card may consolidate multiple packets per interrupt
// TODO: lifetime for IPState probably needs fixing
// TODO: Make some Sender type
pub type IPProtocolHandler = //Handler<Ip>;
    Box<Fn<(Ip,), ()> + Send + Sync + 'static>;

pub type ProtocolTable = Vec<Vec<IPProtocolHandler>>;

pub struct IPState {
    pub routes:            RWLock<RoutingTable>,
    pub ip_to_interface:    InterfaceTable,
    pub interfaces:        Vec<InterfaceRow>,
    pub protocol_handlers: RWLock<ProtocolTable>,
    // Identification counter? increased with each packet sent out,
    // used in Identification header for fragmentation purposes
}

impl IPState {
    pub fn new(ip_to_interface_vec: Vec<InterfaceRow>) -> Arc<IPState>
    {
        use std::iter::count;
        let ip_to_interface = {
            let ip_to_interface_iter = ip_to_interface_vec.iter()
                .zip(count(0, 1))
                .map(|(&(ref src, _, _), ix)| (src.clone(), ix));
            FromIterator::from_iter(ip_to_interface_iter)
        };

        let state = Arc::new(IPState {
            routes:            RWLock::new(HashMap::new()),
            ip_to_interface:        ip_to_interface,
            interfaces:     ip_to_interface_vec,
            protocol_handlers: RWLock::new(Vec::with_capacity(size_of::<u8>())),
        });

        for &(_, _, ref interface) in state.interfaces.iter() {
            use self::receive::make_receive_callback;
            (*interface.write())
                .update_recv_handler(make_receive_callback(state.clone()));
        }

        state
    }

    /// Returns DLInterface struct for the requested interface
    pub fn get_interface<'a> (&'a self, interface_ix: uint) -> Option<&'a InterfaceRow> {
        self.interfaces.as_slice().get(interface_ix)
    }
}
