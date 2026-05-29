use z3::ast::{Bool, Int};
use z3::{Context, Solver};

use super::vars::MachineVars;

pub fn add<'ctx>(
    ctx: &'ctx Context,
    solver: &Solver<'ctx>,
    vars: &[MachineVars<'ctx>],
    footprints: &[(u32, u32)],
) {
    for i in 0..vars.len() {
        for j in (i + 1)..vars.len() {
            let disj = pairwise(ctx, &vars[i], &vars[j], footprints[i], footprints[j]);
            solver.assert(&disj);
        }
    }
}

fn pairwise<'ctx>(
    ctx: &'ctx Context,
    a: &MachineVars<'ctx>,
    b: &MachineVars<'ctx>,
    (aw, ah): (u32, u32),
    (bw, bh): (u32, u32),
) -> Bool<'ctx> {
    let aw_i = Int::from_i64(ctx, aw as i64);
    let ah_i = Int::from_i64(ctx, ah as i64);
    let bw_i = Int::from_i64(ctx, bw as i64);
    let bh_i = Int::from_i64(ctx, bh as i64);
    let a_right = Int::add(ctx, &[&a.x, &aw_i]);
    let b_right = Int::add(ctx, &[&b.x, &bw_i]);
    let a_bottom = Int::add(ctx, &[&a.y, &ah_i]);
    let b_bottom = Int::add(ctx, &[&b.y, &bh_i]);
    let left = a_right.le(&b.x);
    let right = b_right.le(&a.x);
    let above = a_bottom.le(&b.y);
    let below = b_bottom.le(&a.y);
    Bool::or(ctx, &[&left, &right, &above, &below])
}
