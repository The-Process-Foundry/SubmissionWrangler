//! Add a query DSL for speaking to the graphs. This is based on GQL and is a pretty close to direct
//! mapping of the language.

///
pub struct _Match {
  optional: bool,
}

pub enum _Clause {
  Match,
  Where,
  Return,
  Set,
  Delete,
  Merge,
  With,
}
