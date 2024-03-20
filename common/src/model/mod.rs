//! The shape and relationship of application data

pub trait Unwrapable: std::fmt::Debug + Clone {
  /// A generic unwrap function to convert single item enum value into the inner value
  fn unwrap<Value>(&self) -> Value;
}

pub trait Accessible {
  type Struct;
  type Field: Clone;
  type FieldValue: std::fmt::Debug + Clone;

  /// Get an iterable containing all of the fields
  fn fields() -> Vec<Self::Field>;

  /// Use a discriminated union to re
  fn get(&self, field: Self::Field) -> Self::FieldValue;

  fn set(&mut self, value: Self::FieldValue);
}

pub mod organization;
use organization::Organization;

/// Identifiers for the desired object defined in the model
pub enum ModelNode {
  Organization,
  Submission,
  LineItem,
  Invoice,
  Payment,
  Address,
}

/// Identifiers for the desired link defined in the model
pub enum ModelEdge {
  OrganizationParent,
}

/// Containers for the objects defined in by the model
pub enum ModelValue {
  Organizations(Vec<Organization>),
  OrganizationParent(Organization, Organization),
}

/// Items used in the majority of model objects
mod local {
  pub use super::Accessible;

  pub use uuid::Uuid;
}
