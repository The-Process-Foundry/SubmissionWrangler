//! An enumeration of all the possible errors that the server can generate

pub mod allwhat;
pub use allwhat::*;

#[derive(Debug, Clone)]
pub enum WranglerErrorKind {
  /// Errors that were raised by internal code and are not explicitly handled
  UnrecognizedError,

  /// A generic error when checking correctness of incoming data
  ValidationError,

  /// Problems concerning database connectivity.
  GraphDbError,

  /// Problem regarding IO
  IOError,

  /// Thread error raised by Tokio
  TokioError,
}

impl core::fmt::Display for WranglerErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl allwhat::ErrorKind for WranglerErrorKind {
  fn name(&self) -> &str {
    match self {
      Self::UnrecognizedError => "UnrecognizedError",
      Self::ValidationError => "ValidationError",
      Self::GraphDbError => "GraphDbError",
      Self::IOError => "IOError",
      Self::TokioError => "TokioError",
    }
  }

  fn description(&self) -> &str {
    match self {
      Self::UnrecognizedError => "An error that was not captured and converted automatically",
      Self::ValidationError => "ValidationError",
      Self::GraphDbError => "Graph Database Connectivity Error",
      Self::IOError => "An IO Error",
      Self::TokioError => "An issue caused managing threads via Tokio",
    }
  }
}

// --------------------   Conversions   -----------------------
impl From<std::io::Error> for AllWhat<WranglerErrorKind> {
  fn from(err: std::io::Error) -> Self {
    let result: AllWhat<WranglerErrorKind> = WranglerErrorKind::IOError.into();
    result.set_dev_context(&format!("From <std::io::Error>:\n{:#?}", err))
  }
}

// Cannot import this as it doesn't seem to work for the client side
// impl From<neo4rs::Error> for AllWhat<WranglerErrorKind> {
//   fn from(err: neo4rs::Error) -> Self {
//     let result: AllWhat<WranglerErrorKind> = WranglerErrorKind::GraphDbError.into();
//     result.set_dev_context(&format!("From <neo4rs>:\n{:#?}", err))
//   }
// }
