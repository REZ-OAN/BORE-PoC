// shared data_structures, utilities and protocol definitions

// used for handling errors and provides additional context when an error occurs
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt};
use uuid::Uuid;

pub const CONTROL_PORT: u16 = 7045;

// message from the client to the control connection
#[derive(Serialize, Deserialize)]
pub enum ClientMessage {

    // Client Message specifying a port to forward
    Hello(u16),

    // Accepts an incoming TCP connection, using this stream as a proxy
    Accept(Uuid),
}
// message from the server to the control connection
#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    /// Response to a client's initial message, with actual public port.
    Hello(u16),
    // to test the client is still reachable or not
    HeartBeat,

    // asks the client to accept a forwarded TCP connection
    Connection(Uuid),

    /// Indicates a server error that terminates the connection.
    Error(String),
}

/// copy data bidirectionally between two asynchronous read/write streams (stream1 and stream2).
pub async fn proxy<S1, S2>(stream1: S1, stream2: S2) -> io::Result<()>
where
    S1: AsyncRead + AsyncWrite + Unpin,
    S2: AsyncRead + AsyncWrite + Unpin,
{
    let (mut s1_read, mut s1_write) = io::split(stream1);
    let (mut s2_read, mut s2_write) = io::split(stream2);
    tokio::try_join!(
        io::copy(&mut s1_read, &mut s2_write),
        io::copy(&mut s2_read, &mut s1_write),
    )?;
    Ok(())
}

/// Read the next null-delimited JSON instruction from a stream.
pub async fn recv_json<T: DeserializeOwned>(
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
    Ok(serde_json::from_slice(buf).context("failed to parse JSON")?)
}

/// Send a null-terminated JSON instruction on a stream.
pub async fn send_json<T: Serialize>(writer: &mut (impl AsyncWrite + Unpin), msg: T) -> Result<()> {
    let msg = serde_json::to_vec(&msg)?;
    writer.write_all(&msg).await?;
    writer.write_all(&[0]).await?;
    Ok(())
}
