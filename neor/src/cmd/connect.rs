#![allow(clippy::unused_io_amount)]

//! Create a new connection to the database server

use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;
use std::time::Duration;

use async_native_tls::{Certificate, TlsConnector};
use async_net::TcpStream;
use dashmap::DashMap;
use futures::channel::oneshot;
use futures::lock::Mutex;
use tokio::task;
use tokio::time;

use crate::constants::{
    DEFAULT_RETHINKDB_DBNAME, DEFAULT_RETHINKDB_HOSTNAME, DEFAULT_RETHINKDB_PASSWORD,
    DEFAULT_RETHINKDB_PORT, DEFAULT_RETHINKDB_USER, RETHINKDB_DRIVER_NAME,
};
use crate::err::ReqlDriverError;
use crate::{InnerSession, Result, Session, StaticString, TcpStreamConnection};

#[derive(Debug)]
#[non_exhaustive]
pub struct ConnectionCommand {
    /// Host of the RethinkDB instance. The default value is `localhost`.
    host: Cow<'static, str>,

    /// The driver port, by default `28015`.
    port: u16,

    /// The database used if not explicitly specified in a query, by default `test`.
    db: Cow<'static, str>,

    /// The user account to connect as (default `admin`).
    user: Cow<'static, str>,

    /// The password for the user account to connect as (default `""`, empty).
    password: Cow<'static, str>,

    timeout: Option<Duration>,

    tls_connector: Option<TlsConnector>,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SslContext<'a> {
    pub ca_certs: &'a str,
    pub auth_key: Option<&'a str>,
}

impl ConnectionCommand {
    /// This method connect to database
    pub async fn connect(self) -> Result<Session> {
        if let Some(timeout) = self.timeout {
            let (sender, reciever) = oneshot::channel();

            task::spawn(async move { sender.send(self.create_session().await) });

            let session = time::timeout(timeout, reciever)
                .await
                .unwrap_or_else(|_| {
                    panic!(
                        "It took {} seconds to open the connection",
                        timeout.as_secs_f32()
                    )
                })
                .expect("The connection has been closed");

            session
        } else {
            self.create_session().await
        }
    }

    /// This method set database host
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into().static_string();
        self
    }

    /// This method set database port
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// This method set database name
    pub fn dbname(mut self, dbname: impl Into<String>) -> Self {
        self.db = Cow::from(dbname.into());
        self
    }

    /// This method set database user
    pub fn user<U, P>(mut self, user: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        self.user = user.into().static_string();
        self.password = password.into().static_string();
        self
    }

    /// Timeout period in seconds for the connection to be opened
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// This method set ssl connection
    pub fn ssl_context(mut self, ssl_context: SslContext) -> Self {
        let mut file = File::open(ssl_context.ca_certs).unwrap();
        let mut certificate = Vec::new();

        file.read_to_end(&mut certificate).unwrap();

        let certificate = if let Ok(cert) = Certificate::from_pem(&certificate) {
            cert
        } else {
            Certificate::from_der(&certificate).unwrap()
        };

        self.tls_connector = Some(TlsConnector::new().add_root_certificate(certificate));

        self
    }

    /// This method builds a connection from an uri
    pub fn from_uri(mut self, uri: impl Into<String>) -> Result<Self> {
        let db_url = url::Url::parse(uri.into().as_str())?;

        if db_url.scheme() != RETHINKDB_DRIVER_NAME {
            return Err(ReqlDriverError::DriverUrl(format!(
                "Driver scheme is not '{}'",
                RETHINKDB_DRIVER_NAME
            ))
            .into());
        } else {
            self.user = db_url.username().to_string().static_string();
            self.host = db_url
                .host_str()
                .ok_or_else(|| ReqlDriverError::DriverUrl("Host not found.".to_string()))?
                .to_string()
                .static_string();
            self.port = db_url
                .port()
                .ok_or_else(|| ReqlDriverError::DriverUrl("Port not found.".to_string()))?;
            self.db = db_url
                .path_segments()
                .ok_or_else(|| ReqlDriverError::DriverUrl("DB Name not found.".to_string()))?
                .next()
                .unwrap()
                .to_string()
                .static_string();
            self.password = db_url
                .password()
                .unwrap_or(DEFAULT_RETHINKDB_PASSWORD)
                .to_string()
                .static_string();

            Ok(self)
        }
    }

    async fn create_session(self) -> Result<Session> {
        let stream = TcpStream::connect((self.host.as_ref(), self.port)).await?;
        let mut stream = TcpStreamConnection {
            tls_stream: if let Some(connector) = &self.tls_connector {
                let stream = connector
                    .connect(self.host.as_ref(), stream.clone())
                    .await?;
                Some(stream)
            } else {
                None
            },
            stream,
        };

        if let Some(tcp_stream) = stream.tls_stream {
            stream.tls_stream = Some(tools::handshake(tcp_stream, &self).await?);
        } else {
            stream.stream = tools::handshake(stream.stream, &self).await?;
        }

        let inner = InnerSession {
            stream: Mutex::new(stream),
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
}

impl Default for ConnectionCommand {
    fn default() -> Self {
        Self {
            host: DEFAULT_RETHINKDB_HOSTNAME.static_string(),
            port: DEFAULT_RETHINKDB_PORT,
            db: DEFAULT_RETHINKDB_DBNAME.static_string(),
            user: DEFAULT_RETHINKDB_USER.static_string(),
            password: DEFAULT_RETHINKDB_PASSWORD.static_string(),
            timeout: None,
            tls_connector: None,
        }
    }
}

mod tools {
    use futures::io::{AsyncReadExt, AsyncWriteExt};
    use futures::{AsyncRead, AsyncWrite};
    use ql2::version_dummy::Version;
    use scram::client::{ScramClient, ServerFinal, ServerFirst};
    use serde::{Deserialize, Serialize};
    use tracing::trace;

    use super::ConnectionCommand;
    use crate::cmd::bytes_to_string;
    use crate::constants::{
        BUFFER_SIZE, DEFAULT_AUTHENTICATION_METHOD, NULL_BYTE, PROTOCOL_VERSION,
    };
    use crate::{err, Result};

    // Performs the actual handshake
    //
    // This method optimises message exchange as suggested in the RethinkDB
    // documentation by sending message 3 right after message 1, without waiting
    // for message 2 first.
    pub async fn handshake<T>(mut stream: T, opts: &ConnectionCommand) -> Result<T>
    where
        T: Unpin + AsyncWrite + AsyncReadExt + AsyncRead + AsyncReadExt,
    {
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
                return Err(err::ReqlDriverError::Other(msg).into());
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
                return Err(err::ReqlRuntimeError::Internal(bytes_to_string(resp)).into());
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
                return Err(err::ReqlDriverError::Other(msg).into());
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
            .map_err(err::ReqlDriverError::Other)?;
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
                        return Err(err::ReqlDriverError::Auth(msg).into());
                    }
                }
                return Err(err::ReqlRuntimeError::Internal(bytes_to_string(resp)).into());
            }
            Ok(info)
        }
    }

    fn server_final(scram: ServerFinal, resp: &[u8]) -> Result<()> {
        let info = AuthResponse::from_slice(resp)?;
        if let Some(auth) = info.authentication {
            if let Err(error) = scram.handle_server_final(&auth) {
                return Err(err::ReqlDriverError::Other(error.to_string()).into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::err::{ReqlDriverError, ReqlError};

    use super::ConnectionCommand;

    #[tokio::test]
    async fn test_default_connection() {
        execute_test(ConnectionCommand::default()).await
    }

    #[tokio::test]
    async fn test_custom_connection() {
        let connection_command = ConnectionCommand::default()
            .host("127.0.0.1")
            .port(28015)
            .user("admin", "")
            .dbname("test");

        execute_test(connection_command).await
    }

    async fn execute_test(connection_command: ConnectionCommand) {
        let db_expected = connection_command.db.clone();

        match connection_command.connect().await {
            Ok(session) => {
                let db_obtained = &session.inner.db.lock().await;
                assert!(db_obtained.eq(&db_expected));
            }
            Err(err) => {
                if let ReqlError::Driver(err) = err {
                    match err {
                        ReqlDriverError::Io(err, msg) => {
                            assert!(std::io::ErrorKind::ConnectionRefused.eq(&err), "{}", msg)
                        }
                        ReqlDriverError::Auth(msg) => assert!(true, "{}", msg),
                        _ => (),
                    }
                }
            }
        };
    }
}
