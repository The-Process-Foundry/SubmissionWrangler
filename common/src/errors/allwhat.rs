//! A set of tools for working with a pre-defined error

use core::fmt::{Debug, Display};

pub trait ErrorKind: Debug + Clone {
  fn name(&self) -> &str;

  fn description(&self) -> &str;
}

#[derive(Clone)]
pub struct AllWhat<Kind: ErrorKind> {
  /// A domain specific error
  kind: Kind,

  /// Optional grouping to capture handled errors that caused this error
  inner: Option<Vec<AllWhat<Kind>>>,
  /// A human readable message added at the spot where the message was generated
  context: Option<String>,
  /// A separate message that contains information only useful to a developer
  dev_context: Option<String>,
}

impl<KIND: ErrorKind> AllWhat<KIND> {
  pub fn set_context(self, ctx: &str) -> Self {
    AllWhat {
      context: Some(ctx.to_string()),
      ..self
    }
  }

  pub fn set_dev_context(self, ctx: &str) -> Self {
    AllWhat {
      dev_context: Some(ctx.to_string()),
      ..self
    }
  }

  pub fn render_context(&self) -> String {
    match (&self.context, &self.dev_context) {
      (Some(ctx), Some(dev)) => format!("{}. {}", ctx, dev),

      (Some(ctx), None) => ctx.to_string(),
      (None, Some(dev)) => dev.to_string(),
      (None, None) => self.kind.description().to_string(),
    }
  }

  /// Take a list of results and aggregate
  pub fn flatten<VALUE: Clone + Debug>(
    aggregate_kind: KIND,
    children: Vec<Result<VALUE, AllWhat<KIND>>>,
  ) -> Result<Vec<VALUE>, AllWhat<KIND>>
  where
    KIND: ErrorKind,
  {
    let (failed, successes): (Vec<_>, Vec<_>) =
      children.into_iter().partition(|child| child.is_err());

    match failed.is_empty() {
      true => Ok(
        successes
          .iter()
          .map(|value| value.clone().unwrap())
          .collect(),
      ),
      false => Err(AllWhat {
        kind: aggregate_kind,
        inner: Some(
          failed
            .iter()
            .map(|error| error.clone().unwrap_err())
            .collect(),
        ),
        context: None,
        dev_context: None,
      }),
    }
  }
}

impl<Kind: ErrorKind> Debug for AllWhat<Kind> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // If it has children, make it a struct, otherwise, use a one-liner
    match self.inner.clone() {
      Some(inner) => f
        .debug_struct(self.kind.name())
        .field("Context", &self.render_context())
        .field("Children", &inner)
        .finish(),

      None => write!(f, "{}: {}", self.kind.name(), &self.render_context()),
    }
  }
}

impl<Kind: ErrorKind> Display for AllWhat<Kind> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // Print the description by default, but override it with the user context
    let ctx = match &self.context {
      Some(ctx) => ("Context", &ctx[..]),
      None => ("Description", self.kind.description()),
    };

    // If it has children, make it a struct, otherwise, use a one-liner
    match self.inner.clone() {
      Some(inner) => f
        .debug_struct(self.kind.name())
        .field(ctx.0, &ctx.1)
        .field("Children", &inner)
        .finish(),

      None => write!(f, "{}: {}", self.kind.name(), ctx.1),
    }
  }
}

impl<K: ErrorKind> From<K> for AllWhat<K> {
  fn from(kind: K) -> Self {
    AllWhat {
      kind,
      inner: None,
      context: None,
      dev_context: None,
    }
  }
}

pub trait ResultPlus: Sized {
  // A successful output value
  type VALUE: Clone;
  // The domain error type
  type KIND: ErrorKind;

  fn set_context(self, msg: &str) -> Self;
}

impl<O: Clone, K: ErrorKind> ResultPlus for std::result::Result<O, AllWhat<K>> {
  type VALUE = O;
  type KIND = K;

  fn set_context(self, msg: &str) -> Self {
    match self {
      Ok(inner) => Ok(inner),
      Err(err) => Err(err.set_context(msg)),
    }
  }
}
