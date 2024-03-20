//! A workspace that aggregates all the configured connections the calls would need
//!
//! The configuration should be serializable so that it can be stored locally and run on startup.

use crate::local::*;

use super::services::docker::Docker;
use crate::services::graph_db::*;
use wrangler_common::configuration::primitives::path::*;

#[derive(Debug, Clone)]
pub struct ServiceConfigs {
  /// A set of logger sinks for capturing tracing events
  logger: Option<()>,

  /// A graph database to store the submission data
  wrangler_db: GraphDb,

  /// Where to run docker based commands
  docker: Option<Docker>,
}

impl Default for ServiceConfigs {
  fn default() -> ServiceConfigs {
    ServiceConfigs {
      logger: None,
      wrangler_db: GraphDb::default(),
      docker: None,
    }
  }
}

/// Places where the user would like files to be organized
#[derive(Debug, Clone)]
pub struct Locations {
  // A path to store logging data
  log: PathConfig,
}

impl Default for Locations {
  fn default() -> Locations {
    Locations {
      log: PathConfig::new("./local/logs"),
    }
  }
}

#[derive(Default, Debug, Clone)]
pub struct WorkspaceConfig {
  /// Various paths to use as roots for service configurations
  locations: Locations,

  /// Explicit configurations for each needed service
  services: ServiceConfigs,
}

/// A singleton designed to give context for all the available tools to a given application
#[derive(Debug, Clone, Default)]
pub struct Workspace {
  /// Standard OS level configurations, such as logging.
  config: WorkspaceConfig,

  /// A graph database to store the submission data
  wrangler_db: Option<GraphDb>,

  /// Where to run docker based commands
  docker: Option<Docker>,
}

impl Workspace {
  // Starts all the services up using the internal configuration
  pub fn init(config: WorkspaceConfig) -> Result<()> {
    // Start the logger
    println!("Logging to {:?}", config.locations.log);

    // Connect to to the graph database
    println!(
      "Initializing connection to Neo4j: {:?}",
      config.services.wrangler_db
    );

    Ok(())
  }
}

impl fmt::Display for Workspace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
