use dashmap::DashMap;
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::lock::Mutex;
use futures::TryFutureExt;
use ql2::query::QueryType;
use ql2::response::ResponseType;
use serde_json::json;
use std::borrow::Cow;
use std::ops::Drop;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::time;
use tracing::trace;

use super::cmd::run::Response;
use super::cmd::StaticString;
use crate::cmd::TcpStreamConnection;
use crate::proto::{Payload, Query};
use crate::types::ServerInfoResponse;
use crate::{err, r, ReqlDriverError, Result};

type Sender = UnboundedSender<Result<(ResponseType, Response)>>;
type Receiver = UnboundedReceiver<Result<(ResponseType, Response)>>;

#[derive(Debug)]
pub(crate) struct InnerSession {
    pub(crate) db: Mutex<Cow<'static, str>>,
    pub(crate) stream: Mutex<TcpStreamConnection>,
    pub(crate) channels: DashMap<u64, Sender>,
    pub(crate) token: AtomicU64,
    pub(crate) broken: AtomicBool,
    pub(crate) change_feed: AtomicBool,
}

impl InnerSession {
    pub(crate) fn token(&self) -> u64 {
        let token = self
            .token
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1))
            .unwrap();
        if token == u64::MAX {
            self.mark_broken();
        }
        token
    }

    pub(crate) fn mark_broken(&self) {
        self.broken.store(true, Ordering::SeqCst);
    }

    pub(crate) fn broken(&self) -> Result<()> {
        if self.broken.load(Ordering::SeqCst) {
            return Err(err::ReqlDriverError::ConnectionBroken.into());
        }
        Ok(())
    }

    pub(crate) fn mark_change_feed(&self) {
        self.change_feed.store(true, Ordering::SeqCst);
    }

    pub(crate) fn unmark_change_feed(&self) {
        self.change_feed.store(false, Ordering::SeqCst);
    }

    pub(crate) fn is_change_feed(&self) -> bool {
        self.change_feed.load(Ordering::SeqCst)
    }

    pub(crate) fn change_feed(&self) -> Result<()> {
        if self.change_feed.load(Ordering::SeqCst) {
            return Err(err::ReqlDriverError::ConnectionLocked.into());
        }
        Ok(())
    }
}

/// The connection object returned by `r.connection()`
#[derive(Debug, Clone)]
pub struct Session {
    pub(crate) inner: Arc<InnerSession>,
}

impl Session {
    #[doc(hidden)]
    pub fn connection(&self) -> Result<Connection> {
        self.inner.broken()?;
        self.inner.change_feed()?;
        let token = self.inner.token();
        let (tx, rx) = mpsc::unbounded();
        self.inner.channels.insert(token, tx);
        Ok(Connection::new(self.clone(), rx, token))
    }

    pub async fn reconnect(
        &self,
        noreply_wait: bool,
        timeout: Option<std::time::Duration>,
    ) -> Result<()> {
        let future = self
            .close(noreply_wait)
            .and_then(|_| async { self.connection() });

        if let Some(timeout) = timeout {
            time::timeout(timeout, future).await.unwrap()?;
        } else {
            future.await?;
        }

        Ok(())
    }

    pub async fn use_(&mut self, db_name: &'static str) {
        *self.inner.db.lock().await = db_name.static_string();
    }

    /// `noreply_wait` ensures that previous queries with
    /// the `noreply` flag have been processed by the server.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// result.close()
    /// ```
    ///
    /// ## Note
    ///
    /// Note that this guarantee only applies to queries run on the given connection.
    ///
    /// ## Examples
    ///
    /// We have previously run queries with the `noreply` argument set to `true`.
    /// Now wait until the server has processed them.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     
    ///     conn.noreply_wait().await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [run](crate::Command::run)
    /// - [sync](crate::Command::sync)
    pub async fn noreply_wait(&self) -> Result<()> {
        let mut conn = self.connection()?;
        let payload = Payload(QueryType::NoreplyWait, None, Default::default());
        trace!(
            "waiting for noreply operations to finish; token: {}",
            conn.token
        );
        let (typ, _) = conn.request(&payload, false).await?;
        trace!(
            "session.noreply_wait() run; token: {}, response type: {:?}",
            conn.token,
            typ,
        );
        Ok(())
    }

    /// Return information about the server being used by a connection.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// result.server() -> response
    /// ```
    ///
    /// Where:
    /// - server: [ServerInfoResponse](crate::types::ServerInfoResponse)
    ///
    /// ## Examples
    ///
    /// Return server information.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::types::ServerInfoResponse;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     let response: ServerInfoResponse = conn.server().await?;
    ///
    ///     assert_eq!(response.id.to_string(), "404bef53-4b2c-433f-9184-bc3f7bda4a15");
    ///     assert_eq!(response.name, Some("amadeus".to_string()));
    ///     assert_eq!(response.proxy, false);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn server(&self) -> Result<ServerInfoResponse> {
        let mut conn = self.connection()?;
        let payload = Payload(QueryType::ServerInfo, None, Default::default());
        trace!("retrieving server information; token: {}", conn.token);
        let (typ, resp) = conn.request(&payload, false).await?;
        trace!(
            "session.server() run; token: {}, response type: {:?}",
            conn.token,
            typ,
        );
        let mut vec = serde_json::from_value::<Vec<ServerInfoResponse>>(resp.r)?;
        let info = vec
            .pop()
            .ok_or_else(|| ReqlDriverError::Other("server info is empty".into()))?;
        Ok(info)
    }

    /// Close a cursor.
    ///
    /// # Command syntax
    ///
    /// ```text
    /// result.close()
    /// ```
    ///
    /// # Description
    ///
    /// Closing a result cancels the corresponding query and
    /// frees the memory associated with the open request.
    ///
    /// ## Examples
    ///
    /// Close a result.
    ///
    /// ```
    /// use reql_rust::prelude::Converter;
    /// use reql_rust::{r, Result};
    ///
    /// async fn example() -> Result<()> {
    ///     let conn = r.connection().connect().await?;
    ///     
    ///     conn.close().await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Related commands
    pub async fn close(&self, noreply_wait: bool) -> Result<()> {
        self.connection()?.close(noreply_wait).await
    }

    #[doc(hidden)]
    pub fn is_broken(&self) -> bool {
        self.inner.broken.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub(crate) session: Session,
    pub(crate) rx: Arc<Mutex<Receiver>>,
    pub(crate) token: u64,
    pub(crate) closed: Arc<AtomicBool>,
}

impl Connection {
    fn new(session: Session, rx: Receiver, token: u64) -> Connection {
        Connection {
            session,
            token,
            rx: Arc::new(Mutex::new(rx)),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn close(&mut self, noreply_wait: bool) -> Result<()> {
        if !self.session.inner.is_change_feed() {
            trace!(
                "ignoring conn.close() called on a normal connection; token: {}",
                self.token
            );
            return Ok(());
        }

        self.set_closed(true);

        let arg = if noreply_wait {
            Some(r.expr(json!({ "noreply": false })))
        } else {
            None
        };

        let payload = Payload(QueryType::Stop, arg.as_ref().map(Query), Default::default());
        trace!("closing a changefeed; token: {}", self.token);
        let (typ, _) = self.request(&payload, false).await?;
        self.session.inner.unmark_change_feed();
        trace!(
            "conn.close() run; token: {}, response type: {:?}",
            self.token,
            typ,
        );
        Ok(())
    }

    pub(crate) fn closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    pub(crate) fn set_closed(&self, closed: bool) {
        self.closed.store(closed, Ordering::SeqCst);
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.session.inner.channels.remove(&self.token);
        if self.session.inner.is_change_feed() {
            self.session.inner.unmark_change_feed();
        }
    }
}
