//! The shape and relationship of application data

use std::{collections::HashMap, sync::Arc};

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

pub mod address;
use address::Address;

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

pub enum ModelPart {
  Node(ModelNode),
  Edge(ModelEdge),
}

/// Containers for the objects defined in by the model
pub enum ModelValue {
  Organization(Arc<Organization>),
  OrganizationParent(Arc<Organization>, Arc<Organization>),
}

/// Contain all the instances of a given type of node
pub struct ModelNodes<T>
where
  T: Accessible,
{
  lookup: HashMap<uuid::Uuid, Arc<T>>,
}

impl<T> ModelNodes<T>
where
  T: Accessible,
{
  pub fn list(&self) -> Vec<Arc<T>> {
    self
      .lookup
      .values()
      .map(|node: &Arc<T>| node.clone())
      .collect()
  }
}

/// This is a local copy of the graph. Could be Grapht
pub struct SubmissionLog {
  organizations: ModelNodes<Organization>,
}

impl SubmissionLog {
  pub fn list(&self, part: ModelPart) -> Vec<ModelValue> {
    match part {
      ModelPart::Node(ModelNode::Organization) => self
        .organizations
        .list()
        .iter()
        .map(|org| ModelValue::Organization(org.clone()))
        .collect(),
      ModelPart::Node(_) => todo!("Can only list Organizations so far"),
      ModelPart::Edge(_) => todo!("Cannot list edges yet"),
    }
  }
}

/// Items used in the majority of model objects
mod local {
  pub use super::Accessible;

  pub use uuid::Uuid;
}
