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
  pub db: GraphDb,

  /// Where to run docker based commands
  pub docker: Option<Docker>,
}

impl Workspace {
  fn init_db(conf: &GraphDb) -> GraphDb {
    // Connect to to the graph database
    info!("Initializing connection to Neo4j: {:?}", conf);

    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .unwrap();

    rt.block_on(GraphDb::open(conf.driver.clone(), &conf.db_name.clone()))
      .unwrap()
  }

  // Starts all the services up using the internal configuration
  pub fn init(config: WorkspaceConfig) -> Result<Workspace> {
    // Start the logger
    info!("Logging to {:?}", config.locations.log);

    let db = Workspace::init_db(&config.services.wrangler_db);

    let workspace = Workspace {
      config,
      db,
      docker: None,
    };
    Ok(workspace)
  }
}

impl fmt::Display for Workspace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
