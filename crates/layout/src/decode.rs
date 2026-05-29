use tracing::trace;
use z3::ast::Int;
use z3::Model;

use blueprint::entity::{AssemblingMachine, Entity, EntityKind, Furnace, MiningDrill};
use blueprint::world::EntityName;
use blueprint::{Position, World};
use planner::{ProductionLine, ProductionNode};
use prototypes::{Database, MachineKind};

use crate::error::{Error, Result};
use crate::solver::vars::MachineVars;

pub fn build_world<'ctx>(
    db: &Database,
    line: &ProductionLine,
    footprints: &[(u32, u32)],
    vars: &[MachineVars<'ctx>],
    model: &Model<'ctx>,
) -> Result<World> {
    let mut world = World::new();
    for (node, (v, &(w, h))) in line
        .nodes
        .iter()
        .zip(vars.iter().zip(footprints.iter()))
    {
        let x = eval_i64(model, &v.x)?;
        let y = eval_i64(model, &v.y)?;
        let position = Position::new(x as f64 + w as f64 / 2.0, y as f64 + h as f64 / 2.0);
        let kind = kind_for_node(db, node)?;
        let name = EntityName::from(node.machine.clone());
        trace!(node = %node.id, machine = %node.machine, x, y, "placed");
        world.add(Entity::new(name, position, kind));
    }
    Ok(world)
}

fn eval_i64<'ctx>(model: &Model<'ctx>, ast: &Int<'ctx>) -> Result<i64> {
    model
        .eval(ast, true)
        .and_then(|v| v.as_i64())
        .ok_or(Error::ModelEval)
}

fn kind_for_node(db: &Database, node: &ProductionNode) -> Result<EntityKind> {
    let m = db
        .machines
        .get(&node.machine)
        .ok_or_else(|| Error::UnknownMachine(node.machine.clone()))?;
    match m.kind {
        MachineKind::AssemblingMachine => Ok(AssemblingMachine::default()
            .with_recipe(node.recipe.clone())
            .into()),
        MachineKind::Furnace => Ok(Furnace::default().into()),
        MachineKind::MiningDrill => Ok(MiningDrill::default().into()),
        kind => Err(Error::UnsupportedMachineKind {
            kind,
            machine: node.machine.clone(),
        }),
    }
}
