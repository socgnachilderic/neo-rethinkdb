use std::sync::Arc;
use std::{error, fmt, io};

/// The most generic error message in ReQL
#[derive(Debug, Clone)]
pub enum ReqlError {
    Compile(String),
    Runtime(ReqlRuntimeError),
    Driver(ReqlDriverError),
}

impl error::Error for ReqlError {}

impl fmt::Display for ReqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Compile(msg) => write!(f, "compile error; {}", msg),
            Self::Runtime(msg) => write!(f, "runtime error; {}", msg),
            Self::Driver(msg) => write!(f, "client error; {}", msg),
        }
    }
}

/// The parent class of all runtime errors
///
/// All errors on the server unrelated to compilation. Programs may use this to catch any runtime
/// error, but the server will always return a more specific error class.
#[derive(Debug, Clone)]
pub enum ReqlRuntimeError {
    /// The query contains a logical impossibility, such as adding a number to a string.
    QueryLogic(String),
    NonExistence(String),
    ResourceLimit(String),
    User(String),
    Internal(String),
    Availability(ReqlAvailabilityError),
    Permission(String),
}

impl From<ReqlRuntimeError> for ReqlError {
    fn from(err: ReqlRuntimeError) -> ReqlError {
        ReqlError::Runtime(err)
    }
}

impl fmt::Display for ReqlRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::QueryLogic(msg) => write!(f, "query logic; {}", msg),
            Self::NonExistence(msg) => write!(f, "non-existence error; {}", msg),
            Self::ResourceLimit(msg) => write!(f, "resource limit error; {}", msg),
            Self::User(msg) => write!(f, "user error; {}", msg),
            Self::Internal(msg) => write!(f, "internal error; {}", msg),
            Self::Availability(msg) => write!(f, "availability error; {}", msg),
            Self::Permission(msg) => write!(f, "permission error; {}", msg),
        }
    }
}

/// A server in the cluster is unavailable
///
/// The parent class of `OpFailedError` and `OpIndeterminateError`. Programs may use this
/// to catch any availability error, but the server will always return one of this class’s
/// children.
#[derive(Debug, Clone)]
pub enum ReqlAvailabilityError {
    OpFailed(String),
    OpIndeterminate(String),
}

impl From<ReqlAvailabilityError> for ReqlError {
    fn from(err: ReqlAvailabilityError) -> ReqlError {
        ReqlRuntimeError::Availability(err).into()
    }
}

impl fmt::Display for ReqlAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OpFailed(msg) => write!(f, "operation failed; {}", msg),
            Self::OpIndeterminate(msg) => write!(f, "operation indeterminate; {}", msg),
        }
    }
}

/// An error has occurred within the driver
///
/// This may be a driver bug, or it may be an unfulfillable command, such as an unserializable
/// query.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ReqlDriverError {
    Auth(String),
    ConnectionBroken,
    ConnectionLocked,
    Io(io::ErrorKind, String),
    Json(Arc<serde_json::Error>),
    Other(String),
    Time(String),
    Tls(String),
}

impl From<ReqlDriverError> for ReqlError {
    fn from(err: ReqlDriverError) -> ReqlError {
        ReqlError::Driver(err)
    }
}

impl fmt::Display for ReqlDriverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Auth(msg) => write!(f, "auth error; {}", msg),
            Self::ConnectionBroken => write!(f, "connection broken"),
            Self::ConnectionLocked => write!(
                f,
                "another query is running a changefeed on this connection"
            ),
            Self::Io(_, error) => write!(f, "{}", error),
            Self::Json(error) => write!(f, "{}", error),
            Self::Other(msg) => write!(f, "{}", msg),
            Self::Time(error) => write!(f, "{}", error),
            Self::Tls(error) => write!(f, "{}", error),
        }
    }
}

impl From<io::Error> for ReqlError {
    fn from(err: io::Error) -> ReqlError {
        ReqlDriverError::Io(err.kind(), err.to_string()).into()
    }
}

impl From<serde_json::Error> for ReqlError {
    fn from(err: serde_json::Error) -> ReqlError {
        ReqlDriverError::Json(Arc::new(err)).into()
    }
}

impl From<async_native_tls::Error> for ReqlError {
    fn from(err: async_native_tls::Error) -> Self {
        ReqlDriverError::Tls(err.to_string()).into()
    }
}

impl From<time::error::ComponentRange> for ReqlError {
    fn from(err: time::error::ComponentRange) -> Self {
        ReqlDriverError::Time(err.to_string()).into()
    }
}

impl From<time::error::Parse> for ReqlError {
    fn from(err: time::error::Parse) -> Self {
        ReqlDriverError::Time(err.to_string()).into()
    }
}

impl From<time::error::InvalidFormatDescription> for ReqlError {
    fn from(err: time::error::InvalidFormatDescription) -> Self {
        ReqlDriverError::Time(err.to_string()).into()
    }
}

impl From<time::error::Format> for ReqlError {
    fn from(err: time::error::Format) -> Self {
        ReqlDriverError::Time(err.to_string()).into()
    }
}

