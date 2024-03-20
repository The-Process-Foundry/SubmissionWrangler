//! A port of the internal database used by InvoicerUI. This is used to build an explicit graph from
//! the FHL submission log.

use rust_decimal::prelude::*;

/// Temporary flattened model used for reading/writing csv data
mod reader {
  use rust_decimal::Decimal;

  #[derive(Clone, Debug)]
  pub struct Organization {
    pub guid: uuid::Uuid,
    pub source_id: i32,
    pub pretty_id: String,
    pub name: String,
    pub parent: Option<uuid::Uuid>,
    pub children: String,
    pub raw: String,
  }

  pub struct Submission {
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
}

mod model {
  use rust_decimal::Decimal;
  use std::{collections::HashMap, sync::Arc};

  #[derive(Clone, Debug)]
  pub struct Person {
    pub guid: uuid::Uuid,
    pub salutation: Option<String>,
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
  }

  #[derive(Clone, Debug)]
  pub struct Address {
    pub guid: uuid::Uuid,
    street1: String,
    street2: Option<String>,
    street3: Option<String>,
  }

  #[derive(Clone, Debug)]
  pub struct Organization {
    pub guid: uuid::Uuid,
    pub source_id: i32,
    pub pretty_id: String,
    pub name: String,
    pub parent: Option<Arc<Organization>>,
    pub children: HashMap<String, Arc<Organization>>,
    pub submissions: HashMap<String, Arc<Submission>>,
    pub location: Option<Arc<Address>>,
    pub balance: Decimal,
  }

  #[derive(Clone, Debug)]
  pub struct Submission {
    pub guid: uuid::Uuid,
    pub accession_number: String,
    pub submitting_org: Arc<Organization>,
    pub submitted_by: Option<Arc<Person>>,
    pub category: String,
    pub total: Decimal,
    pub finalized_on: String,
  }

  #[derive(Clone, Debug)]
  pub struct LineItem {
    pub guid: uuid::Uuid,
    pub name: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub started_on: Option<String>,
    pub finished_on: Option<String>,
    pub paid: bool,
  }

  impl LineItem {
    pub fn total(&self) -> Decimal {
      self.quantity * self.price
    }

    pub fn finish(&mut self) -> Result<(), String> {
      self.finished_on = Some("3/10/2024".to_string());
      Ok(())
    }
  }

  #[derive(Clone, Debug)]
  pub enum PaymentType {
    Check,
    ACH,
    Cash,
    /// A use
    Credit,
    CreditCard,
    Discount,
  }
  #[derive(Clone, Debug)]
  pub struct Payment {
    pub guid: uuid::Uuid,
    pub _type: PaymentType,
    /// The organization that handles the accounting, which may be different from the one submitting work
    pub payer: Arc<Organization>,
    pub amount: Decimal,
    pub received: Option<String>,
    pub deposited: Option<String>,
  }

  #[derive(Clone, Debug)]
  pub struct Invoice {
    pub guid: uuid::Uuid,
    pub date: Option<String>,
    pub total: Decimal,
    /// The amount of unpaid value on the invoice. This number is always positive.
    pub balance: Decimal,
    /// The Organization that contains the accounting department. This should be the parent that handles
    /// the money as opposed to the specific organization that sent the specific submission
    pub billed_to: Arc<Organization>,
    pub items: HashMap<uuid::Uuid, Arc<LineItem>>,
  }

  impl Invoice {
    pub fn add(&mut self, line_item: LineItem) -> Result<(), String> {
      self.balance += line_item.total();
      self.items.add(line_item.guid, line_item);
      Ok(())
    }

    /// Apply the organization's balance to the individual line items. This invoice will be flagged as
    /// unpaid until the balance is zero.
    pub fn pay(&mut self) -> Result<(), String> {
      for (_, &mut item) in self.items.iter() {
        if item.amount <= self.billed_to.balance {
          item.paid;
          self.balance -= item.amount;
          self.billed_to.balance -= item.amount;
        }
      }
      Ok(())
    }
  }
}

mod grapht {
  use neo4rs::*;
  use std::sync::Arc;

  use super::model;

  pub struct Neo4jConfig {
    pub uri: String,
    pub username: String,
    pub password: String,
  }

  #[derive(Clone)]
  pub struct Neo4jConnection {
    // /// Tokio::Runtime - an async loop for temporarily making the connection synchronous.
    // rt: tokio::runtime::Runtime,
    graph: Arc<Graph>,
  }

  impl Neo4jConnection {
    pub async fn connect(config: Neo4jConfig) -> Neo4jConnection {
      let graph = Graph::new(&config.uri, config.username, config.password)
        .await
        .unwrap();

      Neo4jConnection {
        graph: Arc::new(graph),
      }
    }

    pub async fn exec(&self, orgs: Vec<String>) -> Result<(), String> {
      // let orgs = orgs[0..50].to_vec();
      // println!("Neo4j insert Organization: {:#?}", orgs);

      let mut txn = self.graph.clone().start_txn().await.unwrap();
      match txn.run_queries(orgs).await {
        Err(err) => match &err {
          Error::UnexpectedMessage(msg) => {
            println!("{:#?}", msg);
            panic!("Unexpected Message in query")
          }
          _ => panic!("Unexpected syntax error in Query: {:?}", err),
        },

        txn => (),
      }
      match txn.commit().await {
        Err(err) => {
          println!("{:#?}", err);
          panic!("Failed to insert everything")
        }
        txn => Ok(()),
      } //or txn.rollback().await.unwrap()
    }

    pub async fn query(&self, q: &'static str) -> Result<(), String> {
      let graph = self.graph.clone();
      tokio::spawn(async move {
        println!("Running a query");
        let mut result = graph
          .execute(query(&q)) // .param("name", "Mark"))
          .await
          .unwrap();
        println!(r#"Finished query - printing results:"#);
        while let Ok(Some(row)) = result.next().await {
          println!("Query Result: {:#?}", row);
        }
      });
      Ok(())
    }
  }
}

use grapht::*;
use model::*;

use std::collections::HashMap;

fn load_orgs(file_name: &str) -> HashMap<i32, reader::Organization> {
  // Create a CSV parser that reads data from stdin.
  let mut rdr = csv::ReaderBuilder::new()
    .delimiter('\t' as u8)
    .has_headers(true)
    .from_path(file_name)
    .unwrap();

  // Loop over each record.
  rdr.records().fold(HashMap::new(), |mut acc, result| {
    // An error may occur, so abort the program in an unfriendly way.
    // We will make this more friendly later!
    let record = result.expect("a CSV record");
    // Print a debug version of the record.
    // println!("{:?}) {:?}", i, record);
    let org = reader::Organization {
      guid: uuid::Uuid::new_v4(),
      source_id: record.get(0).unwrap().parse().unwrap(),
      pretty_id: record.get(1).unwrap().to_string(),
      name: record.get(2).unwrap().to_string(),
      parent: None,
      children: record.get(4).unwrap().to_string(),
      raw: format!("{:#?}", record),
    };
    let old_guid = record
      .get(0)
      .expect("The old guid was not in field #0")
      .parse()
      .expect(&format!(
        "Could not convert record {:?} into an i32",
        org.pretty_id
      ));
    acc.insert(old_guid, org);
    acc
  })
}

fn load_subs(file_name: &str) -> HashMap<String, reader::Submission> {
  // Create a CSV parser that reads data from stdin.
  let mut rdr = csv::ReaderBuilder::new()
    .delimiter('\t' as u8)
    .has_headers(true)
    .from_path(file_name)
    .unwrap();

  // Loop over each record.
  rdr.records().fold(HashMap::new(), |mut acc, result| {
    // An error may occur, so abort the program in an unfriendly way.
    // We will make this more friendly later!
    match result {
      Ok(record) => {
        let sub = reader::Submission {
          guid: uuid::Uuid::new_v4(),
          accession_number: record.get(0).unwrap().to_string(),
          submitting_org: record.get(2).unwrap().to_string(),
          submitted_by: record.get(3).unwrap().to_string(),
          category: record.get(4).unwrap().to_string(),
          line_items: record.get(5).unwrap().to_string(),
          species: record.get(6).unwrap().to_string(),
          pet_name: match record.get(8).unwrap() {
            "" => None,
            val => Some(val.to_string()),
          },
          received_on: match record.get(9).unwrap() {
            "" => None,
            val => Some(val.to_string()),
          },
          finalized_on: match record.get(10).unwrap() {
            "" => None,
            val => Some(val.to_string()),
          },
          diagnosis: match record.get(17).unwrap() {
            "" => None,
            val => Some(val.to_string()),
          },
          total: {
            let mut total = Decimal::from_str(record.get(19).unwrap()).unwrap();
            total.rescale(2);
            total
          },
          billed_on: match record.get(21).unwrap() {
            "" => None,
            val => Some(val.to_string()),
          },
          paid_on: match record.get(23).unwrap() {
            "" => None,
            val => Some(val.to_string()),
          },
          deposited_on: match record.get(25).unwrap() {
            "" => None,
            val => Some(val.to_string()),
          },
          invoice_number: match record.get(27).unwrap() {
            "" => None,
            val => Some(val.parse().unwrap()),
          },
        };
        println!("Accession number: {}", sub.accession_number);
        acc.insert(sub.accession_number.clone(), sub);
        acc
      }
      Err(err) => match err.into_kind() {
        csv::ErrorKind::UnequalLengths {
          pos,
          expected_len,
          len,
        } => {
          println!(
            "Random submission with bad length {} ({}) at {:?}",
            len, expected_len, pos
          );
          acc
        }
        _ => panic!("Bad data in csv"),
      },
    }
  })
}

async fn insert_orgs(
  conn: &grapht::Neo4jConnection,
  orgs: &HashMap<i32, reader::Organization>,
) -> Result<(), String> {
  // Read the orgs and insert each one
  let inserts: Vec<String> = orgs
    .into_iter()
    .map(|(old_id, org)| {
      format!(
        "
        MERGE (o:Organization {{
          id: {},
          guid: '{:?}',
          pretty_id: '{}',
          name: '{:?}'
        }})",
        old_id,
        org.guid,
        org.pretty_id,
        // FIXME: This escape doesn't work. Need to use a named param in the insert
        org.name.replace("'", "\\'")
      )
    })
    .collect();

  conn.exec(inserts).await
}

async fn map_child(conn: &grapht::Neo4jConnection, parent: reader::Organization, child_id: &str) {
  let query = format!(
    " MATCH (p:Organization {{source_id: '{}'}})
      MATCH (c:Organization {{source_id: '{}'}})
      MERGE (p)-[:PARENT_OF]->(c)
      MERGE (c)-[:CHILD_OF]->(p)
    ",
    parent.source_id, child_id
  );
  conn.exec(vec![query]).await.unwrap()
}

async fn map_children(
  conn: &grapht::Neo4jConnection,
  orgs: &HashMap<i32, reader::Organization>,
) -> Result<(), String> {
  println!("Adding in relationships to the orgs");

  for org in orgs.values() {
    if org.children.len() > 2 {
      let mut children = org.children.chars();
      children.next();
      children.next_back();
      let children: Vec<i32> = {
        let child_ids = children.as_str().replace(' ', "");
        child_ids
          .split(',')
          .map(|child_id| child_id.parse::<i32>().unwrap())
          .collect()
      };

      println!("Org {} has children: {:#?}", org.pretty_id, children);
      // Get the child element
      for child_id in children {
        map_child(&conn, org.clone(), &child_id.to_string()).await
      }
    }
  }
  Ok(())
}

async fn map_subs(
  conn: &grapht::Neo4jConnection,
  subs: &HashMap<String, reader::Submission>,
) -> Result<(), String> {
  println!("Mapping in the subs to the orgs");

  let mut invoices: HashMap<i32, Invoice> = HashMap::new();
  let mut payments: HashMap<i32, Payment> = HashMap::new();

  for sub in subs.values() {
    let query = format!(
      "MATCH (o:Organization {{ pretty_id: '{}'}})
       CREATE (s:Submission {{
          guid: '{}',
          accession_number: '{}',
          category: '{}',
          total: '{}'
       }})
       MERGE (o)-[:Submitted]->(s)
       MERGE (o)<-[:SubmittedBy]-(s)
      ",
      sub.submitting_org, sub.guid, sub.accession_number, sub.category, sub.total
    );
    conn.exec(vec![query]).await.unwrap()
  }
  Ok(())
}

// fn add_invoice(invoices: &mut HashMap<i32, Invoice>) {
//   let mut org =
// }

async fn map_line_items(
  conn: &grapht::Neo4jConnection,
  subs: &HashMap<String, reader::Submission>,
) -> Result<(), String> {
  println!("Mapping the line items to the subs");
  let line_re = regex::Regex::new(
    r#"(?x)
        \[
          (?<name>"[^"]+")
          \s*,\s*
          (?<quantity>\d+\.?\d*)
          \s*,\s*
          (?<price>\d+\.?\d*)
          \s*
        \]\s*,?\s*
    "#,
  )
  .unwrap();

  let invoices: HashMap<String, String>;
  let payments: HashMap<String, String>;

  for sub in subs.values() {
    println!(
      "Processing line items from sub: {}: {}",
      sub.accession_number, sub.line_items
    );
    // Deserialize the line items
    let mut total: Decimal = Decimal::from_f32(0.0).unwrap();
    for (_, [name, quantity, price]) in line_re.captures_iter(&sub.line_items).map(|c| c.extract())
    {
      let item = LineItem {
        guid: uuid::Uuid::new_v4(),
        name: name.to_string(),
        quantity: Decimal::from_str(quantity).unwrap(),
        price: Decimal::from_str(price).unwrap(),
        started_on: sub.received_on.clone(),
        finished_on: sub.finalized_on.clone(),
        paid: sub.paid_on.is_some(),
      };

      total += item.quantity * item.price;

      let create_line_item = [
        "CREATE (l:Service {".to_string(),
        format!("guid: '{}',", item.guid),
        format!("name: '{}',", item.name),
        format!("quantity: {}", item.quantity),
        format!("price: {}", item.price),
        match item.started_on {
          Some(date) => format!("started_on: '{}'", date),
          None => "".to_string(),
        },
        match item.finished_on {
          Some(date) => format!("finished_on: '{}',", date),
          None => "".to_string(),
        },
        format!("paid: {}", item.paid),
        ")".to_string(),
      ]
      .join("");

      let query = format!(
        "MATCH (s:Submission {{accession_number: '{}'}})
         {}
         (s)-[:LineItem]->(l)
         (l)-[:BelongsTo]->(s)
        ",
        sub.accession_number, create_line_item
      );
    }
    // Verify individual items total the submission total
    match &total == &sub.total {
      true => (),
      false => println!(
        "--> Line item totals didn't match {}: {} != {}",
        sub.accession_number, sub.total, total
      ),
    }
  }
  Ok(())
}

#[tokio::main]
async fn main() {
  println!("Starting insert");

  let config = Neo4jConfig {
    uri: "127.0.0.1:7687".to_string(),
    username: "neo4j".to_string(),
    password: "neo_pass".to_string(),
  };

  let conn = grapht::Neo4jConnection::connect(config).await;

  // Clear the database
  conn
    .exec(vec!["MATCH (n) DETACH DELETE n".to_string()])
    .await
    .unwrap();

  // Load the orgs
  let orgs_file = "data/organizations.tsv";
  let orgs = load_orgs(orgs_file);

  let subs_file = "data/submissions_2023.tsv";
  let subs = load_subs(subs_file);

  insert_orgs(&conn, &orgs).await.unwrap();
  map_children(&conn, &orgs).await.unwrap();
  map_subs(&conn, &subs).await.unwrap();
  map_line_items(&conn, &subs).await.unwrap();

  // let show_subs = "
  //   MATCH (o:Organization {pretty_id: 'WOW'})-[Submitted]->(s:Submission)
  //   RETURN s
  // ";
  // conn.query(show_subs).await.unwrap();

  tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
  println!("\n---> Finished Running.\n\n");
}
