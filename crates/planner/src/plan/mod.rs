mod planner;

use tracing::{info, instrument};

use prototypes::Database;

use crate::config::PlanRequest;
use crate::error::Result;
use crate::line::ProductionLine;

#[instrument(level = "info", skip_all)]
pub fn plan(db: &Database, request: &PlanRequest) -> Result<ProductionLine> {
    info!(targets = request.targets.len(), "planning production line");
    let mut planner = planner::Planner::new(db, &request.config);
    for (resource, rate) in &request.targets {
        planner.add_target(resource.clone(), *rate)?;
    }
    let line = planner.finish();
    info!(
        nodes = line.nodes.len(),
        edges = line.edges.len(),
        raw_inputs = line.raw_inputs.len(),
        outputs = line.outputs.len(),
        "production line planned",
    );
    Ok(line)
}
