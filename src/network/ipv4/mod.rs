use std::collections::hashmap::HashMap;
use std::sync::RWLock;

use self::headers::IPAddr;
use interface::Interface;

pub mod headers;
pub mod send;
pub mod receive;

pub struct RoutingRow {
    pub cost:      u8,     // How many hops
    pub next_hop:  IPAddr, // which link-layer interface to use
}

// key: IP we want to reach
pub type RoutingTable = HashMap<IPAddr, RoutingRow>;

// key:   adjacent ip (next hop)
// value:  which one of our IPs we put as the src address
//         which interface we send the packet with
pub type InterfaceTable = HashMap<IPAddr, (IPAddr, Box<Interface+'static>)>;

pub struct IPState {
    routes:     RWLock<RoutingTable>,
    interfaces: InterfaceTable,
}

impl IPState {
    pub fn new(interfaces: InterfaceTable) -> IPState {
        IPState {
            routes:     RWLock::new(HashMap::new()),
            interfaces: interfaces,
        }
    }
}
