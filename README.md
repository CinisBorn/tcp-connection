# Description 

A wrapper around of `std::net` for TCP connection handling.

The goal is reduce boilerplate, establish a common interface between projects and so on...

# Features 

Provides two types of connection life-cycle: `Connection` and `PersistentConnection`.

`Connection` has a short life-cycle. Once the incoming connection is accepted, the TCP listener is dropped and a `Connection`
instance containing the correspondent `TcpStream` is returned.

`PersistentConnection` preserves the listener. Different from `Connection`, `PersistentConnection` creates and binds a listener
in a given *address*, returning a `PersistentConnection` instance containing the correspondent listener. The listener can 
be reused "n" times.
