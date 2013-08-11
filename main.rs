extern mod socket_rust;

use socket_rust::WebSocket::create;

fn main() { println(~"hello " + create()); }
