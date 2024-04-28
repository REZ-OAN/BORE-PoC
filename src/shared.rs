// shared data_structures, utilities and protocol definitions

use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncRead, AsyncWrite};
use uuid::Uuid;

pub const CONTROL_PORT: u16 = 7570;

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
    // to test the client is still reachable or not
    HeartBeat,

    // asks the client to accept a forwarded TCP connection
    Connection(Uuid),
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