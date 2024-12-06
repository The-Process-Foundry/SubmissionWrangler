//! Load the old style submission log into the database

use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};
use tracing::info;

use crate::workspace::Workspace;
use wrangler_common::{longrunner::TaskState, model::organization::Organization};

mod model;

pub fn runner(state: Arc<RwLock<TaskState>>, ctx: Workspace) {
  info!("Runner has started");
  info!("Importing Organizations");
  // Read the rows
  let mut org_reader = csv::ReaderBuilder::new()
    .delimiter(',' as u8)
    .has_headers(true)
    .from_path("/home/dfogelson/Foundry/FHL/wrangler/data/Organizations.csv")
    .unwrap();

  // Read the org file
  let org_rows = org_reader
    .records()
    .fold(HashMap::new(), |mut acc, result| {
      let record = result.expect("Did not get a CSV record");

      let org = model::OrganizationRow {
        guid: uuid::Uuid::new_v4(),
        source_id: record.get(0).unwrap().parse().unwrap(),
        pretty_id: record.get(1).unwrap().to_string(),
        name: record.get(2).unwrap().to_string(),
        parent: None,
        children: record.get(4).unwrap().to_string(),
        department: record.get(7).map(|value| value.to_string()),
        email: record.get(8).map(|value| value.to_string()),
        phone: record.get(9).map(|value| value.to_string()),
        address1: record.get(11).unwrap().to_string(),
        address2: record.get(12).map(|value| value.to_string()),
        city: record.get(13).unwrap().to_string(),
        state: record.get(14).unwrap().to_string(),
        zip: record.get(15).unwrap().to_string(),
        terms: record.get(16).unwrap().to_string(),
        cash_credit: record.get(17).map(|value| value.to_string()),
        raw: format!("{:#?}", record),
      };

      acc.insert(org.source_id, org);
      acc
    });

  info!("Read {} Organizations", org_rows.len());
  // For each row, convert to an organization and add to the hashmap
  let mut orgs: HashMap<i32, Arc<RwLock<Organization>>> = HashMap::new();
  for (key, value) in org_rows.iter() {
    info!("    key: {}, name {}", key, value.name);
    let mut org: Organization = value.clone().into();
    let _ = org.addresses.add(
      Arc::new(RwLock::new(value.clone().into())),
      "Primary".to_string(),
      1,
    );
    orgs.insert(key.clone(), Arc::new(RwLock::new(org)));
  }

  // For each row, if children is not empty
  for (key, value) in org_rows.iter() {
    if value.children.len() < 3 {
      continue;
    }
    info!("Found a child for {}", key);
    let child_regex = regex::Regex::new(r"\s*(?<value>\d{1,3})\s*,?").unwrap();

    // Get the parent
    let org = orgs.get(key).unwrap();

    for (_, [value]) in child_regex
      .captures_iter(&value.children)
      .map(|c| c.extract())
    {
      // - Add the hashmap item to the parent org
      let child = orgs.get(&value.parse().unwrap()).unwrap();
      let guid = child.read().unwrap().guid;

      org.write().unwrap().children.insert(guid, child.clone());

      // let reader = child.read().unwrap();
      // let pretty_id = reader.pretty_id.clone();
      // info!("Got a child value of {} - {}", value, pretty_id);
      // let addr = reader.addresses.try_heaviest(None).unwrap().clone();
      // let addr = addr.read().unwrap();
      // info!(
      //   "\t{} {}, {} {}",
      //   addr.street1, addr.city, addr.state, addr.zip
      // );
      // drop(reader);
      // drop(addr);

      // - Add the parent to the child node
      let mut child = child.write().unwrap();
      child.parent = Some(org.clone());
    }
  }
}
