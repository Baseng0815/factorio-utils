pub mod bounds;
pub mod overlap;
pub mod vars;

use tracing::{info, instrument, warn};
use z3::{Config, Context, Params, SatResult, Solver};

use blueprint::World;
use planner::ProductionLine;
use prototypes::Database;

use crate::config::LayoutConfig;
use crate::decode;
use crate::error::{Error, Result};
use crate::footprint::footprint_for;

#[instrument(level = "info", skip_all, fields(
    nodes = line.nodes.len(),
    width = config.width,
    height = config.height,
))]
pub fn solve(db: &Database, line: &ProductionLine, config: &LayoutConfig) -> Result<World> {
    let footprints = collect_footprints(db, line)?;
    let z3_cfg = Config::new();
    let ctx = Context::new(&z3_cfg);
    let vars = vars::declare(&ctx, line);
    let solver = Solver::new(&ctx);
    apply_timeout(&ctx, &solver, config);
    bounds::add(&ctx, &solver, &vars, &footprints, config);
    overlap::add(&ctx, &solver, &vars, &footprints);
    match solver.check() {
        SatResult::Sat => {
            info!("layout SAT, decoding model");
            let model = solver.get_model().ok_or(Error::ModelEval)?;
            decode::build_world(db, line, &footprints, &vars, &model)
        }
        SatResult::Unsat => {
            warn!(
                width = config.width,
                height = config.height,
                "layout UNSAT for grid",
            );
            Err(Error::Unsat {
                width: config.width,
                height: config.height,
            })
        }
        SatResult::Unknown => {
            warn!("z3 returned UNKNOWN (likely timed out)");
            Err(Error::Timeout)
        }
    }
}

fn apply_timeout<'ctx>(ctx: &'ctx Context, solver: &Solver<'ctx>, config: &LayoutConfig) {
    let Some(timeout) = config.timeout else {
        return;
    };
    let millis = timeout.as_millis().min(u32::MAX as u128) as u32;
    let mut params = Params::new(ctx);
    params.set_u32("timeout", millis);
    solver.set_params(&params);
}

fn collect_footprints(db: &Database, line: &ProductionLine) -> Result<Vec<(u32, u32)>> {
    line.nodes
        .iter()
        .map(|n| footprint_for(db, &n.machine))
        .collect()
}
