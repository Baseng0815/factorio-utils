use z3::ast::Int;
use z3::{Context, Solver};

use crate::config::LayoutConfig;

use super::vars::MachineVars;

pub fn add<'ctx>(
    ctx: &'ctx Context,
    solver: &Solver<'ctx>,
    vars: &[MachineVars<'ctx>],
    footprints: &[(u32, u32)],
    config: &LayoutConfig,
) {
    let zero = Int::from_i64(ctx, 0);
    let w_grid = Int::from_i64(ctx, config.width as i64);
    let h_grid = Int::from_i64(ctx, config.height as i64);
    for (v, &(w, h)) in vars.iter().zip(footprints.iter()) {
        let wi = Int::from_i64(ctx, w as i64);
        let hi = Int::from_i64(ctx, h as i64);
        let x_plus_w = Int::add(ctx, &[&v.x, &wi]);
        let y_plus_h = Int::add(ctx, &[&v.y, &hi]);
        solver.assert(&zero.le(&v.x));
        solver.assert(&zero.le(&v.y));
        solver.assert(&x_plus_w.le(&w_grid));
        solver.assert(&y_plus_h.le(&h_grid));
    }
}
