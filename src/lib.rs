//! TCP connection handling.
//!
//! This crate provides types and utilities for TCP handling.
use std::io::Error;
use std::net::{
    TcpListener, 
    TcpStream, 
    ToSocketAddrs
};


/// A TCP connection Listener.
#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    /// Tries to connect to `addr` specified
    /// 
    /// # Errors: 
    /// 
    /// If any *I/O* error occur, a `ConnectionError` is returned. 
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, Error> {
        let listener = TcpListener::bind(addr)?;
        let stream = listener.accept()?;
        
        Ok(Self { stream: stream.0 })
    }

    /// Tries to connect with a given listener.
    /// 
    /// # Errors: 
    /// 
    /// If any *I/O* error occur, a `ConnectionError` is returned. 
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
