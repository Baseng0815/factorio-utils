use tracing::trace;
use z3::ast::Int;
use z3::Context;

use planner::ProductionLine;

#[derive(Debug)]
pub struct MachineVars<'ctx> {
    pub x: Int<'ctx>,
    pub y: Int<'ctx>,
}

pub fn declare<'ctx>(ctx: &'ctx Context, line: &ProductionLine) -> Vec<MachineVars<'ctx>> {
    line.nodes
        .iter()
        .map(|n| {
            let i = n.id.index();
            trace!(node = %n.id, machine = %n.machine, "declaring x_{i}, y_{i}");
            MachineVars {
                x: Int::new_const(ctx, format!("x_{i}")),
                y: Int::new_const(ctx, format!("y_{i}")),
            }
        })
        .collect()
}
