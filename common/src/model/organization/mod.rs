// This is the definition of an Organization: a business entity consisting of at least one person

use super::local::*;
use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

#[derive(Clone, Debug)]
pub struct Organization {
  /// A globally unique identifier for the Organization
  pub guid: Uuid,
  /// A unique user friendly identifier used in reporting
  pub pretty_id: String,
  /// The full name of the organization
  pub name: String,
  pub parent: Option<Arc<RwLock<Organization>>>,
  pub children: HashMap<Uuid, Arc<RwLock<Organization>>>,
}

#[derive(Clone, Debug)]
pub enum OrganizationField {
  Guid,
  PrettyId,
  Name,
  Parent,
  Children,
}

#[derive(Clone, Debug)]
pub enum OrganizationFieldValue {
  Guid(Uuid),
  PrettyId(String),
  Name(String),
  Parent(Option<Arc<RwLock<Organization>>>),
  Children(HashMap<Uuid, Arc<RwLock<Organization>>>),
}

impl Organization {
  pub fn sample(name: &str) -> Organization {
    Organization {
      guid: Uuid::new_v4(),
      pretty_id: name[0..4].to_string(),
      name: name.to_string(),
      parent: None,
      children: HashMap::new(),
    }
  }
}

impl Accessible for Organization {
  type Struct = Organization;

  type Field = OrganizationField;

  type FieldValue = OrganizationFieldValue;

  fn fields() -> Vec<Self::Field> {
    vec![
      OrganizationField::Guid,
      OrganizationField::PrettyId,
      OrganizationField::Name,
      OrganizationField::Parent,
      OrganizationField::Children,
    ]
  }

  fn get(&self, field: Self::Field) -> Self::FieldValue {
    match field {
      OrganizationField::Guid => OrganizationFieldValue::Guid(self.guid.clone()),
      OrganizationField::PrettyId => OrganizationFieldValue::PrettyId(self.pretty_id.clone()),
      OrganizationField::Name => OrganizationFieldValue::Name(self.name.clone()),
      OrganizationField::Parent => OrganizationFieldValue::Parent(self.parent.clone()),
      OrganizationField::Children => OrganizationFieldValue::Children(self.children.clone()),
    }
  }

  fn set(&mut self, value: Self::FieldValue) {
    match value {
      OrganizationFieldValue::Guid(inner) => self.guid = inner.clone(),
      OrganizationFieldValue::PrettyId(inner) => self.pretty_id = inner.clone(),
      OrganizationFieldValue::Name(inner) => self.name = inner.clone(),
      OrganizationFieldValue::Parent(inner) => self.parent = inner.clone(),
      OrganizationFieldValue::Children(inner) => self.children = inner.clone(),
    }
  }
}
