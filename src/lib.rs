//! TCP connection handling.
//!
//! This crate provides types and utilities for TCP handling.
use std::io::Error;

use std::net::{
    TcpListener, 
    TcpStream, 
    ToSocketAddrs
};

/// A reusable TCP listener.  
/// 
/// [`Connection`] drops the listener once the connection is established.
pub struct PersistentConnection {
    listener: TcpListener
}

impl PersistentConnection {
    /// Binds a TCP listener to `addr` returing an in instance of `PersistentConnection`.
    /// 
    /// # Errors:
    /// 
    /// Returns an error if the binding fails. 
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, Error> {
        Ok(Self { 
            listener: TcpListener::bind(addr)? 
        })
    }

    /// Gets a reference to the `PersistentConnection` listener.
    pub fn listener(&self) -> &TcpListener {
        &self.listener
    }

    /// Creates a `PersistentConnection` instance with a given TCP listener.
    pub fn new_with_listener(listener: TcpListener) -> Self {
        Self { listener }
    }
}

/// A TCP connection Listener.
#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    /// Binds a TCP listener to `addr` and accepts one incoming connection.
    ///
    /// This function blocks until a connection is accepted or an I/O error
    /// occurs. The listener is dropped after the connection is accepted.
    ///
    /// # Errors
    ///
    /// Returns an error if binding the listener fails or if accepting the
    /// incoming connection fails.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, Error> {
        let listener = TcpListener::bind(addr)?;
        let stream = listener.accept()?;
        
        Ok(Self { stream: stream.0 })
    }

    /// Accepts an incoming TCP connection from `listener` and returns a
    /// `Connection` instance.
    ///
    /// This function blocks until a connection is accepted or an I/O error
    /// occurs.
    ///
    /// # Errors
    ///
    /// Returns an error if accepting the incoming connection fails.
    pub fn connect_with_listener(listener: TcpListener) -> Result<Self, Error>  {
        Ok(Self { stream: listener.accept()?.0 })
    }

    /// Gets a mutable reference to the connection stream.
    pub fn stream_mut(&mut self) -> &mut TcpStream {
        &mut self.stream
    }

    /// Gets a reference to the connection stream.
    pub fn stream(&self) -> &TcpStream {
        &self.stream
    }
}
