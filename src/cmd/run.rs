use std::borrow::Cow;
use std::sync::atomic::Ordering;
use std::{mem, str};

use async_stream::try_stream;
use futures::io::{AsyncReadExt, AsyncWriteExt};
use futures::stream::{Stream, StreamExt};
use futures::{AsyncRead, AsyncWrite};
use ql2::query::QueryType;
use ql2::response::{ErrorType, ResponseType};
use reql_macros::CommandOptions;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::trace;

use crate::arguments::{Args, Durability, Format, ReadMode};
use crate::constants::{DATA_SIZE, DEFAULT_RETHINKDB_DBNAME, HEADER_SIZE, TOKEN_SIZE};
use crate::proto::{Payload, Query};
use crate::{err, Command, Connection, Result, Session};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct Response {
    t: i32,
    e: Option<i32>,
    pub(crate) r: Value,
    b: Option<Value>,
    p: Option<Value>,
    n: Option<Value>,
}

impl Response {
    fn new() -> Self {
        Self {
            t: ResponseType::SuccessAtom as i32,
            e: None,
            r: Value::Array(Vec::new()),
            b: None,
            p: None,
            n: None,
        }
    }
}

#[derive(Debug, Clone, CommandOptions, Serialize, Default, PartialEq, PartialOrd)]
pub struct RunOption {
    /// One of three possible values affecting
    /// the consistency guarantee for the query (default: `ReadMode::Single`).
    /// - `ReadMode::Single` (the default) returns values that are in memory
    /// (but not necessarily written to disk) on the primary replica.
    /// - `ReadMode::Majority` will only return values that are safely
    /// committed on disk on a majority of replicas.
    /// This requires sending a message to every replica on each read,
    /// so it is the slowest but most consistent.
    /// - `ReadMode::Outdated` will return values that are in memory
    /// on an arbitrarily-selected replica.
    /// This is the fastest but least consistent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_mode: Option<ReadMode>,
    /// what format to return times in (default: `Format::Native`).
    /// Set this to `Format::Raw`
    /// if you want times returned as JSON objects for exporting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<Format>,
    /// whether or not to return a profile
    /// of the query’s execution (default: `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<bool>,
    /// possible values are `Durability::Hard` and `Durability::Soft`.
    /// In soft durability mode RethinkDB will acknowledge
    /// the write immediately after receiving it,
    /// but before the write has been committed to disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durability: Option<Durability>,
    /// what format to return `grouped_data` and
    /// `grouped_streams` in (default: `Format::Native`).
    /// Set this to `Format::Raw` if you want the raw pseudotype.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noreply: Option<bool>,
    /// the database to run this query against as a string.
    /// The default is the database specified in
    /// the `db` [connection](crate::connection::Connection)
    /// method (which defaults to `test`).
    /// The database may also be specified with the db command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db: Option<Db>,
    /// the maximum numbers of array elements
    /// that can be returned by a query (default: 100,000).
    /// This affects all ReQL commands that return arrays.
    /// Note that it has no effect on the size of arrays
    /// being **written** to the database;
    /// those always have an upper limit of 100,000 elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_limit: Option<usize>,
    /// what format to return binary data in (default: `Format::Native`).
    /// Set this to `Format::Raw` if you want the raw pseudotype.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_format: Option<Format>,
    /// minimum number of rows to wait for before batching
    /// a result set (default: 8). This is an usize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_batch_rows: Option<usize>,
    /// maximum number of rows to wait for before batching
    /// a result set (default: unlimited). This is an usize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_rows: Option<usize>,
    /// maximum number of bytes to wait for before batching
    /// a result set (default: 1MB). This is an usize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_bytes: Option<usize>,
    /// maximum number of seconds to wait before batching
    /// a result set (default: 0.5).
    /// This is a f64 and may be specified to the microsecond.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_seconds: Option<f64>,
    /// factor to scale the other parameters down by on the first batch (default: 4).
    /// For example, with this set to 8 and `max_batch_rows` set to 80,
    /// on the first batch `max_batch_rows` will be adjusted to 10 (80 / 8).
    /// This allows the first batch to return faster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_batch_scaledown_factor: Option<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Db(pub Cow<'static, str>);

impl RunOption {
    async fn default_db(self, session: &Session) -> RunOption {
        let session_db = session.inner.db.lock().await;
        if self.db.is_none() && *session_db != DEFAULT_RETHINKDB_DBNAME {
            return self.db(&*session_db);
        }
        self
    }
}

pub trait RunArg {
    fn into_run_opts(self) -> Result<(Connection, RunOption)>;
}

impl RunArg for &Session {
    fn into_run_opts(self) -> Result<(Connection, RunOption)> {
        let conn = self.connection()?;
        Ok((conn, Default::default()))
    }
}

impl RunArg for Connection {
    fn into_run_opts(self) -> Result<(Connection, RunOption)> {
        Ok((self, Default::default()))
    }
}

impl RunArg for Args<(&Session, RunOption)> {
    fn into_run_opts(self) -> Result<(Connection, RunOption)> {
        let Args((session, options)) = self;
        let conn = session.connection()?;
        Ok((conn, options))
    }
}

impl RunArg for Args<(Connection, RunOption)> {
    fn into_run_opts(self) -> Result<(Connection, RunOption)> {
        let Args(arg) = self;
        Ok(arg)
    }
}

impl RunArg for &mut Session {
    fn into_run_opts(self) -> Result<(Connection, RunOption)> {
        self.connection()?.into_run_opts()
    }
}

impl RunArg for Args<(&mut Session, RunOption)> {
    fn into_run_opts(self) -> Result<(Connection, RunOption)> {
        let Args((session, options)) = self;
        let conn = session.connection()?;

        Args((conn, options)).into_run_opts()
    }
}

pub(crate) fn new<A, T>(query: Command, arg: A) -> impl Stream<Item = Result<T>>
where
    A: RunArg,
    T: Unpin + DeserializeOwned,
{
    try_stream! {
        let (mut conn, mut opts) = arg.into_run_opts()?;
        opts = opts.default_db(&conn.session).await;
        let change_feed = query.change_feed();
        if change_feed {
            conn.session.inner.mark_change_feed();
        }
        let noreply = opts.noreply.unwrap_or_default();
        let mut payload = Payload(QueryType::Start, Some(Query(&query)), opts);

        loop {
            let (response_type, resp) = conn.request(&payload, noreply).await?;
            trace!("yielding response; token: {}", conn.token);

            match response_type {
                ResponseType::SuccessAtom | ResponseType::ServerInfo => {
                    for val in serde_json::from_value::<Vec<T>>(resp.r)? {
                        yield val;
                    }
                    break;
                }
                ResponseType::SuccessSequence => {
                    yield serde_json::from_value::<T>(resp.r)?;
                    break;
                }
                ResponseType::SuccessPartial => {
                    if conn.closed() {
                        // reopen so we can use the connection in future
                        conn.set_closed(false);
                        trace!("connection closed; token: {}", conn.token);
                        break;
                    }
                    payload = Payload(QueryType::Continue, None, Default::default());
                    // for val in serde_json::from_value::<Vec<T>>(resp.r)? {
                    //     yield val;
                    // }
                    yield serde_json::from_value::<T>(resp.r)?;
                    continue;
                }
                ResponseType::WaitComplete => { break; }
                typ => {
                    let msg = error_message(resp.r)?;
                    match typ {
                        // This feed has been closed by conn.close().
                        ResponseType::ClientError if change_feed && msg.contains("not in stream cache") => { break; }
                        _ => Err(response_error(typ, resp.e, msg))?,
                    }
                }
            }
        }
    }
}

impl Payload<'_> {
    fn encode(&self, token: u64) -> Result<Vec<u8>> {
        let bytes = self.to_bytes()?;
        let data_len = bytes.len();
        let mut buf = Vec::with_capacity(HEADER_SIZE + data_len);
        buf.extend_from_slice(&token.to_le_bytes());
        buf.extend_from_slice(&(data_len as u32).to_le_bytes());
        buf.extend_from_slice(&bytes);
        Ok(buf)
    }
}

impl Connection {
    fn send_response(&self, db_token: u64, resp: Result<(ResponseType, Response)>) {
        if let Some(tx) = self.session.inner.channels.get(&db_token) {
            if let Err(error) = tx.unbounded_send(resp) {
                if error.is_disconnected() {
                    self.session.inner.channels.remove(&db_token);
                }
            }
        }
    }

    pub(crate) async fn request<'a>(
        &mut self,
        query: &'a Payload<'a>,
        noreply: bool,
    ) -> Result<(ResponseType, Response)> {
        self.submit(query, noreply).await;
        match self.rx.lock().await.next().await {
            Some(resp) => resp,
            None => Ok((ResponseType::SuccessAtom, Response::new())),
        }
    }

    async fn submit<'a>(&self, query: &'a Payload<'a>, noreply: bool) {
        let mut db_token = self.token;
        let result = self.exec(query, noreply, &mut db_token).await;
        self.send_response(db_token, result);
    }

    async fn exec<'a>(
        &self,
        query: &'a Payload<'a>,
        noreply: bool,
        db_token: &mut u64,
    ) -> Result<(ResponseType, Response)> {
        let buf = query.encode(self.token)?;
        let mut stream = self.session.inner.stream.lock().await;
        let tls_stream = mem::take(&mut stream.tls_stream);

        trace!("sending query; token: {}, payload: {}", self.token, query);
        if let Some(tcp_stream) = tls_stream {
            self.tcp_ops(tcp_stream, buf, noreply, db_token).await
        } else {
            self.tcp_ops(stream.stream.clone(), buf, noreply, db_token)
                .await
        }
    }

    async fn tcp_ops<T>(
        &self,
        mut stream: T,
        buf: Vec<u8>,
        noreply: bool,
        db_token: &mut u64,
    ) -> Result<(ResponseType, Response)>
    where
        T: Unpin + AsyncWrite + AsyncReadExt + AsyncRead + AsyncReadExt,
    {
        stream.write_all(&buf).await?;
        trace!("query sent; token: {}", self.token);

        if noreply {
            return Ok((ResponseType::SuccessAtom, Response::new()));
        }

        trace!("reading header; token: {}", self.token);
        let mut header = [0u8; HEADER_SIZE];
        stream.read_exact(&mut header).await?;

        let mut buf = [0u8; TOKEN_SIZE];
        buf.copy_from_slice(&header[..TOKEN_SIZE]);
        *db_token = {
            let token = u64::from_le_bytes(buf);
            trace!("db_token: {}", token);
            if token > self.session.inner.token.load(Ordering::SeqCst) {
                self.session.inner.mark_broken();
                return Err(err::ReqlDriverError::ConnectionBroken.into());
            }
            token
        };

        let mut buf = [0u8; DATA_SIZE];
        buf.copy_from_slice(&header[TOKEN_SIZE..]);
        let len = u32::from_le_bytes(buf) as usize;
        trace!(
            "header read; token: {}, db_token: {}, response_len: {}",
            self.token,
            db_token,
            len
        );

        trace!("reading body; token: {}", self.token);
        let mut buf = vec![0u8; len];
        stream.read_exact(&mut buf).await?;

        trace!(
            "body read; token: {}, db_token: {}, body: {}",
            self.token,
            db_token,
            super::bytes_to_string(&buf),
        );

        let resp = serde_json::from_slice::<Response>(&buf)?;
        trace!("response successfully parsed; token: {}", self.token,);

        let response_type = ResponseType::from_i32(resp.t).ok_or_else(|| {
            err::ReqlDriverError::Other(format!("unknown response type `{}`", resp.t))
        })?;

        if let Some(error_type) = resp.e {
            let msg = error_message(resp.r)?;
            return Err(response_error(response_type, Some(error_type), msg));
        }

        Ok((response_type, resp))
    }
}

fn error_message(response: Value) -> Result<String> {
    let messages = serde_json::from_value::<Vec<String>>(response)?;
    Ok(messages.join(" "))
}

fn response_error(
    response_type: ResponseType,
    error_type: Option<i32>,
    msg: String,
) -> err::ReqlError {
    match response_type {
        ResponseType::ClientError => err::ReqlDriverError::Other(msg).into(),
        ResponseType::CompileError => err::ReqlError::Compile(msg),
        ResponseType::RuntimeError => match error_type.map(ErrorType::from_i32).ok_or_else(|| {
            err::ReqlDriverError::Other(format!("unexpected runtime error: {}", msg))
        }) {
            Ok(Some(ErrorType::Internal)) => err::ReqlRuntimeError::Internal(msg).into(),
            Ok(Some(ErrorType::ResourceLimit)) => err::ReqlRuntimeError::ResourceLimit(msg).into(),
            Ok(Some(ErrorType::QueryLogic)) => err::ReqlRuntimeError::QueryLogic(msg).into(),
            Ok(Some(ErrorType::NonExistence)) => err::ReqlRuntimeError::NonExistence(msg).into(),
            Ok(Some(ErrorType::OpFailed)) => err::ReqlAvailabilityError::OpFailed(msg).into(),
            Ok(Some(ErrorType::OpIndeterminate)) => {
                err::ReqlAvailabilityError::OpIndeterminate(msg).into()
            }
            Ok(Some(ErrorType::User)) => err::ReqlRuntimeError::User(msg).into(),
            Ok(Some(ErrorType::PermissionError)) => err::ReqlRuntimeError::Permission(msg).into(),
            Err(error) => error.into(),
            _ => err::ReqlDriverError::Other(format!("unexpected runtime error: {}", msg)).into(),
        },
        _ => err::ReqlDriverError::Other(format!("unexpected response: {}", msg)).into(),
    }
}
