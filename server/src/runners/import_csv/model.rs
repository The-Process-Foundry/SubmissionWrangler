//! CSV formatted data model for the original submission log

use rust_decimal::Decimal;
use std::collections::HashMap;

use wrangler_common::model::{
  address::Address, context_edge::ContextEdge, organization::Organization,
};

#[derive(Clone, Debug)]
pub struct OrganizationRow {
  pub guid: uuid::Uuid,
  pub source_id: i32,
  pub pretty_id: String,
  pub name: String,
  pub parent: Option<uuid::Uuid>,
  pub children: String,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub department: Option<String>,
  pub address1: String,
  pub address2: Option<String>,
  pub city: String,
  pub state: String,
  pub zip: String,
  pub terms: String,
  pub cash_credit: Option<String>,
  pub raw: String,
}

impl Into<Organization> for OrganizationRow {
  fn into(self) -> Organization {
    Organization {
      guid: self.guid,
      pretty_id: self.pretty_id,
      name: self.name,
      parent: None,
      children: HashMap::new(),
      addresses: ContextEdge::new(),
    }
  }
}

impl Into<Address> for OrganizationRow {
  fn into(self) -> Address {
    Address {
      guid: uuid::Uuid::new_v4(),
      street1: self.address1,
      street2: self.address2,
      street3: None,
      city: self.city,
      state: self.state,
      zip: self.zip,
    }
  }
}

pub struct SubmissionRow {
  pub guid: uuid::Uuid,
  pub accession_number: String,
  pub submitting_org: String,
  pub submitted_by: String,
  pub category: String,
  pub line_items: String,
  pub species: String,
  pub pet_name: Option<String>,
  pub diagnosis: Option<String>,
  pub total: Decimal,
  pub received_on: Option<String>,
  pub finalized_on: Option<String>,
  pub billed_on: Option<String>,
  pub paid_on: Option<String>,
  pub deposited_on: Option<String>,
  pub invoice_number: Option<i32>,
}
