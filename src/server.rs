
// Server Implementation which will be used for the TCP tunneling or proxy service

// this import provides functionality related to network socket addresses
// used to represent the address and port of the server
use std::net::SocketAddr;

// this provide a shared ownership of data, using this data can be shared between two threads
use std::sync::Arc;

//  used to specify timeouts for waiting connections
use std::time::Duration;

// used for handling errors and provides additional context when an error occurs
use anyhow::{Context, Result};

// concurrent hash map implementation
// allows simultaneous reads and writes from multiple threads.
use dashmap::DashMap;

// used for serialization and deserialization of data structures
use serde::de::DeserializeOwned;
use serde::Serialize;

// provides asynchronous I/O 
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader};


// * provide asynchronous TCP networking capabilities. 
// * TcpListener is used to listen for incoming TCP connections
// * TcpStream represents an asynchronous TCP stream.

use tokio::net::{TcpListener, TcpStream};
// provide asynchronous timer
use tokio::time::{sleep, timeout};

// for logging
use tracing::{info, info_span, warn, Instrument};

// unique identifier of sessions or connections
use uuid::Uuid;

// imported Data_Structes and utilities from shared.rs
use crate::shared::{proxy, ClientMessage, ServerMessage, CONTROL_PORT};

/// State structure for the server.
pub struct Server {
    /// The minimum TCP port that can be forwarded.
    pub min_port: u16,

    /// Concurrent map of IDs to incoming connections.
    conns: Arc<DashMap<Uuid, TcpStream>>,
}

/// implementation of the server
impl Server {
    
    /// Create a new server with a specified minimum port number.
    pub fn new(min_port: u16) -> Server {
        Server {
            min_port,
            conns: Arc::new(DashMap::new()),
        }
    }

    /// Start the server, listening for new connections.
    pub async fn listen(self) -> Result<()> {
        let this = Arc::new(self);
        // binds to all available network interfaces (0.0.0.0) on the specified control port
        let addr = SocketAddr::from(([0, 0, 0, 0], CONTROL_PORT));

        // * binds the TcpListener to the specified address and port, 
        // * enabling it to accept incoming TCP connections. 
        // * It returns a TcpListener instance representing the listening socket.
        let listener = TcpListener::bind(&addr).await?;
        info!(?addr, "server listening");
        
        //loop continuously accepts incoming connections and handles them asynchronously.
  
        loop {
            let (stream, addr) = listener.accept().await?;
            let this = Arc::clone(&this);
            // each new asynchronous task handles an incoming connection independently by tokio::spawn
            tokio::spawn(
                async move {
                    info!("incoming connection");
                    if let Err(err) = this.handle_connection(stream).await {
                        warn!(%err, "connection exited with error");
                    } else {
                        info!("connection exited");
                    }
                }
                .instrument(info_span!("control", ?addr)),
            );
        }
    }
    
        //handls each incoming connection. 
       //It parses messages received from clients ,
      //  performs appropriate actions based on the message type.
     
    async fn handle_connection(&self, stream: TcpStream) -> Result<()> {

        // Wraps the incoming TCP stream 
        // improve performance by reducing the number of read operations
        let mut stream = BufReader::new(stream);

        let mut buffer = Vec::new();

        // next_mp is to parse the message  from the stream
        let msg = next_mp(&mut stream, &mut buffer).await?;

        // this handles the messages
        match msg {
            Some(ClientMessage::Hello(port)) => {
                if port < self.min_port {
                    warn!(?port, "client port number too low");
                    return Ok(());
                }
                info!(?port, "new client");
                let listener = TcpListener::bind(("::", port)).await?;
                loop {
                    if send_mp(&mut stream, ServerMessage::HeartBeat)
                        .await
                        .is_err()
                    {
                        // Assume that the TCP connection has been dropped.
                        return Ok(());
                    }
                    const TIMEOUT: Duration = Duration::from_millis(500);
                    if let Ok(result) = timeout(TIMEOUT, listener.accept()).await {
                        let (stream2, addr) = result?;
                        info!(?addr, ?port, "new connection");

                        let id = Uuid::new_v4();
                        let conns = Arc::clone(&self.conns);
                        conns.insert(id, stream2);
                        tokio::spawn(async move {
                            // Remove stale entries to avoid memory leaks.
                            sleep(Duration::from_secs(10)).await;
                            if conns.remove(&id).is_some() {
                                warn!(?id, "removed stale connection");
                            }
                        });
                        send_mp(&mut stream, ServerMessage::Connection(id)).await?;
                    }
                }
            }
            Some(ClientMessage::Accept(id)) => {
                info!(?id, "forwarding connection");
                match self.conns.remove(&id) {
                    Some((_, stream2)) => proxy(stream, stream2).await?,
                    None => warn!(?id, "missing connection ID"),
                }
                Ok(())
            }
            None => {
                warn!("unexpected EOF");
                Ok(())
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server::new(1024)
    }
}

/// Read the next null-delimited MessagePack instruction from a stream.
async fn next_mp<T: DeserializeOwned>(
    reader: &mut (impl AsyncBufRead + Unpin),
    buf: &mut Vec<u8>,
) -> Result<Option<T>> {
    buf.clear();
    reader.read_until(0, buf).await?;
    if buf.is_empty() {
        return Ok(None);
    }
    if buf.last() == Some(&0) {
        buf.pop();
    }
    Ok(rmp_serde::from_slice(buf).context("failed to parse MessagePack")?)
}

/// Send a null-terminated MessagePack instruction on a stream.
async fn send_mp<T: Serialize>(writer: &mut (impl AsyncWrite + Unpin), msg: T) -> Result<()> {
    let msg = rmp_serde::to_vec(&msg)?;
    writer.write_all(&msg).await?;
    writer.write_all(&[0]).await?;
    Ok(())
}