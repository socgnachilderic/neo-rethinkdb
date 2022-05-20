//! Create a new connection to the database server

use super::args::Args;
use super::{bytes_to_string, StaticString};
use crate::constants::{
    BUFFER_SIZE,
    NULL_BYTE,
    PROTOCOL_VERSION,
    DEFAULT_RETHINKDB_DBNAME,
    DEFAULT_RETHINKDB_PORT,
    DEFAULT_RETHINKDB_HOSTNAME,
    DEFAULT_RETHINKDB_USER,
    DEFAULT_RETHINKDB_PASSWORD, DEFAULT_AUTHENTICATION_METHOD
};
use crate::{err, InnerSession, Result, Session};
use async_net::{AsyncToSocketAddrs, TcpStream};
use dashmap::DashMap;
use futures::io::{AsyncReadExt, AsyncWriteExt};
use futures::lock::Mutex;
use ql2::version_dummy::Version;
use scram::client::{ScramClient, ServerFinal, ServerFirst};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;
use tracing::trace;

/// Options accepted by [crate::r::connection]
/// 
/// An asynchronous Connection to connect RethinkDB Database Server.
/// 
/// # Example
/// ```
/// use reql_rust::r;
/// 
/// let conn = r.connection()
///     .with_host("127.0.0.1")
///     .with_port(28015)
///     .with_db("test")
///     .with_user("admin", "")
///     .connect();
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ConnectionBuilder {
    /// Host of the RethinkDB instance. The default value is `localhost`.
    pub host: Cow<'static, str>,

    /// The driver port, by default `28015`.
    pub port: u16,

    /// The database used if not explicitly specified in a query, by default `test`.
    pub db: Cow<'static, str>,

    /// The user account to connect as (default `admin`).
    pub user: Cow<'static, str>,

    /// The password for the user account to connect as (default `""`, empty).
    pub password: Cow<'static, str>,
}

impl ConnectionBuilder {
    /**
     * This method initialize and launch connection.
     * 
     * This is the same as `Connection::default()`.
     * 
     * Default parameters are:
     * - `host` : localhost
     * - `port` : 28015
     * - `user` : admin
     * - `password` : ""
     */
    pub fn connection() -> Self {
        Self::default()
    }

    /// This method connect to database
    pub async fn connect(self) -> Result<Session> {
        let stream = TcpStream::connect((self.host.as_ref(), self.port)).await?;
        let inner = InnerSession {
            stream: Mutex::new(handshake(stream, &self).await?),
            db: Mutex::new(self.db),
            channels: DashMap::new(),
            token: AtomicU64::new(0),
            broken: AtomicBool::new(false),
            change_feed: AtomicBool::new(false),
        };

        Ok(Session {
            inner: Arc::new(inner),
        })
    }

    /// This method set database host
    pub fn with_host(mut self, host: &'static str) -> Self {
        self.host = host.static_string();
        self
    }

    /// This method set database port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// This method set database name
    pub fn with_db(mut self, db: &'static str) -> Self {
        self.db = Cow::from(db);
        self
    }

    /// This method set database user
    pub fn with_user(mut self, user: &'static str, password: &'static str) -> Self {
        self.user = user.static_string();
        self.password = password.static_string();
        self
    }
}

impl Default for ConnectionBuilder {
    fn default() -> Self {
        Self {
            host: DEFAULT_RETHINKDB_HOSTNAME.static_string(),
            port: DEFAULT_RETHINKDB_PORT,
            db: DEFAULT_RETHINKDB_DBNAME.static_string(),
            user: DEFAULT_RETHINKDB_USER.static_string(),
            password: DEFAULT_RETHINKDB_PASSWORD.static_string(),
        }
    }
}

/// The arguments accepted by [crate::r::connection]
pub trait Arg {
    type ToAddrs: AsyncToSocketAddrs;

    fn into_connect_opts(self) -> (Option<Self::ToAddrs>, ConnectionBuilder);
}

impl Arg for () {
    type ToAddrs = SocketAddr;

    fn into_connect_opts(self) -> (Option<Self::ToAddrs>, ConnectionBuilder) {
        (None, Default::default())
    }
}

impl Arg for ConnectionBuilder {
    type ToAddrs = SocketAddr;

    fn into_connect_opts(self) -> (Option<Self::ToAddrs>, ConnectionBuilder) {
        (None, self)
    }
}

impl<'a> Arg for &'a str {
    type ToAddrs = (&'a str, u16);

    fn into_connect_opts(self) -> (Option<Self::ToAddrs>, ConnectionBuilder) {
        let opts = ConnectionBuilder::default();
        (Some((self, opts.port)), opts)
    }
}

impl<T> Arg for Args<(T, ConnectionBuilder)>
where
    T: AsyncToSocketAddrs,
{
    type ToAddrs = T;

    fn into_connect_opts(self) -> (Option<Self::ToAddrs>, ConnectionBuilder) {
        let Args((addr, opts)) = self;
        (Some(addr), opts)
    }
}

// Performs the actual handshake
//
// This method optimises message exchange as suggested in the RethinkDB
// documentation by sending message 3 right after message 1, without waiting
// for message 2 first.
async fn handshake(mut stream: TcpStream, opts: &ConnectionBuilder) -> Result<TcpStream> {
    trace!("sending supported version to RethinkDB");
    stream
        .write_all(&(Version::V10 as i32).to_le_bytes())
        .await?; // message 1

    let scram = ScramClient::new(opts.user.as_ref(), opts.password.as_ref(), None);
    let (scram, msg) = client_first(scram)?;
    trace!("sending client first message");
    stream.write_all(&msg).await?; // message 3

    let mut buf = [0u8; BUFFER_SIZE];

    trace!("receiving message(s) from RethinkDB");
    stream.read(&mut buf).await?; // message 2
    let (len, resp) = bytes(&buf, 0);
    trace!("received server info; info: {}", bytes_to_string(resp));
    ServerInfo::validate(resp)?;

    let offset = len + 1;
    let resp = if offset < BUFFER_SIZE && buf[offset] != NULL_BYTE {
        bytes(&buf, offset).1
    } else {
        trace!("reading auth response");
        stream.read(&mut buf).await?; // message 4
        bytes(&buf, 0).1
    };
    trace!("received auth response");
    let info = AuthResponse::from_slice(resp)?;
    let auth = match info.authentication {
        Some(auth) => auth,
        None => {
            let msg = String::from("server did not send authentication info");
            return Err(err::Driver::Other(msg).into());
        }
    };

    let (scram, msg) = client_final(scram, &auth)?;
    trace!("sending client final message");
    stream.write_all(&msg).await?; // message 5

    trace!("reading server final message");
    stream.read(&mut buf).await?; // message 6
    let resp = bytes(&buf, 0).1;
    trace!("received server final message");
    server_final(scram, resp)?;

    trace!("client connected successfully");

    Ok(stream)
}

fn bytes(buf: &[u8], offset: usize) -> (usize, &[u8]) {
    let len = (&buf[offset..])
        .iter()
        .take_while(|x| **x != NULL_BYTE)
        .count();
    let max = offset + len;
    (max, &buf[offset..max])
}

// We are going to use &str for `server_version` because it is safe to do so.
// Unfortunately, the other fields that are using String, are doing so because
// because they can potentially contain an escaped double quote which is not
// supported by serde in &str.
#[derive(Serialize, Deserialize, Debug)]
struct ServerInfo<'a> {
    success: bool,
    min_protocol_version: usize,
    max_protocol_version: usize,
    server_version: &'a str,
}

impl ServerInfo<'_> {
    fn validate(resp: &[u8]) -> Result<()> {
        let info = serde_json::from_slice::<ServerInfo>(resp)?;
        if !info.success {
            return Err(err::Runtime::Internal(bytes_to_string(resp)).into());
        }
        #[allow(clippy::absurd_extreme_comparisons)]
        if PROTOCOL_VERSION < info.min_protocol_version
            || info.max_protocol_version < PROTOCOL_VERSION
        {
            let msg = format!(
                "unsupported protocol version {version}, expected between {min} and {max}",
                version = PROTOCOL_VERSION,
                min = info.min_protocol_version,
                max = info.max_protocol_version,
            );
            return Err(err::Driver::Other(msg).into());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequest {
    protocol_version: usize,
    authentication_method: &'static str,
    authentication: String,
}

fn client_first(scram: ScramClient<'_>) -> Result<(ServerFirst<'_>, Vec<u8>)> {
    let (scram, client_first) = scram.client_first();
    let ar = AuthRequest {
        protocol_version: PROTOCOL_VERSION,
        authentication_method: DEFAULT_AUTHENTICATION_METHOD,
        authentication: client_first,
    };
    let mut msg = serde_json::to_vec(&ar)?;
    msg.push(NULL_BYTE);
    Ok((scram, msg))
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthConfirmation {
    authentication: String,
}

fn client_final(scram: ServerFirst<'_>, auth: &str) -> Result<(ServerFinal, Vec<u8>)> {
    let scram = scram
        .handle_server_first(auth)
        .map_err(|x| x.to_string())
        .map_err(err::Driver::Other)?;
    let (scram, client_final) = scram.client_final();
    let conf = AuthConfirmation {
        authentication: client_final,
    };
    let mut msg = serde_json::to_vec(&conf)?;
    msg.push(NULL_BYTE);
    Ok((scram, msg))
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
    success: bool,
    authentication: Option<String>,
    error_code: Option<usize>,
    error: Option<String>,
}

impl AuthResponse {
    fn from_slice(resp: &[u8]) -> Result<Self> {
        let info = serde_json::from_slice::<AuthResponse>(resp)?;
        if !info.success {
            // If error code is between 10 and 20, this is an auth error
            if let Some(10..=20) = info.error_code {
                if let Some(msg) = info.error {
                    return Err(err::Driver::Auth(msg).into());
                }
            }
            return Err(err::Runtime::Internal(bytes_to_string(resp)).into());
        }
        Ok(info)
    }
}

fn server_final(scram: ServerFinal, resp: &[u8]) -> Result<()> {
    let info = AuthResponse::from_slice(resp)?;
    if let Some(auth) = info.authentication {
        if let Err(error) = scram.handle_server_final(&auth) {
            return Err(err::Driver::Other(error.to_string()).into());
        }
    }
    Ok(())
}
