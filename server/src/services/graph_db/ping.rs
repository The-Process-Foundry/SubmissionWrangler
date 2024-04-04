//! A standardized object to be stored as a heartbeat/ping in a graph database

use chrono::NaiveDateTime;
use uuid::Uuid;
use wrangler_common::model::Accessible;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct Ping {
  updated: NaiveDateTime,
  guid: Uuid,
}

impl Ping {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum PingField {
  Updated,
  Guid,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum PingFieldValue {
  Updated(NaiveDateTime),
  Guid(Uuid),
}

impl Accessible for Ping {
  type Struct = Ping;
  type Field = PingField;
  type FieldValue = PingFieldValue;

  fn fields() -> Vec<Self::Field> {
    vec![PingField::Updated, PingField::Guid]
  }

  fn get(&self, field: Self::Field) -> Self::FieldValue {
    match field {
      PingField::Updated => PingFieldValue::Updated(self.updated.clone()),
      PingField::Guid => PingFieldValue::Guid(self.guid.clone()),
    }
  }
  fn set(&mut self, value: Self::FieldValue) {
    match value {
      PingFieldValue::Updated(inner) => self.updated = inner.clone(),
      PingFieldValue::Guid(inner) => self.guid = inner.clone(),
    }
  }
}
