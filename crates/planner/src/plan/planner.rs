use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use tracing::{instrument, trace};

use recipes::{
    CraftingCategory, Database, MachineId, Recipe, RecipeId, ResourceId,
};

use crate::config::PlanConfig;
use crate::error::{Error, Result};
use crate::line::{EdgeEndpoint, NodeId, ProductionEdge, ProductionLine, ProductionNode};
use crate::rate::Rate;

const SURPLUS_EPSILON: f64 = 1e-9;

pub(super) struct Planner<'a> {
    db: &'a Database,
    config: &'a PlanConfig,
    nodes: Vec<ProductionNode>,
    flows: HashMap<FlowKey, f64>,
    producer_for: HashMap<ResourceId, NodeId>,
    surplus: HashMap<ResourceId, f64>,
    raw_inputs: HashMap<ResourceId, f64>,
    visiting: HashSet<RecipeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FlowKey {
    from: EdgeEndpoint,
    to: EdgeEndpoint,
    resource: ResourceId,
}

impl<'a> Planner<'a> {
    pub fn new(db: &'a Database, config: &'a PlanConfig) -> Self {
        Self {
            db,
            config,
            nodes: Vec::new(),
            flows: HashMap::new(),
            producer_for: HashMap::new(),
            surplus: HashMap::new(),
            raw_inputs: HashMap::new(),
            visiting: HashSet::new(),
        }
    }

    pub fn add_target(&mut self, resource: ResourceId, rate: Rate) -> Result<()> {
        self.demand(resource, rate.as_per_second(), EdgeEndpoint::External)
    }

    pub fn finish(mut self) -> ProductionLine {
        self.flush_surplus_to_external();
        let edges = self
            .flows
            .into_iter()
            .map(|(key, rate)| ProductionEdge {
                from: key.from,
                to: key.to,
                resource: key.resource,
                rate: Rate::per_second(rate),
            })
            .collect::<Vec<_>>();
        let raw_inputs = self
            .raw_inputs
            .into_iter()
            .map(|(r, rate)| (r, Rate::per_second(rate)))
            .collect();
        let outputs = compute_outputs(&edges);
        ProductionLine {
            nodes: self.nodes,
            edges,
            raw_inputs,
            outputs,
        }
    }

    fn flush_surplus_to_external(&mut self) {
        let surplus: Vec<(ResourceId, f64)> = self
            .surplus
            .drain()
            .filter(|(_, r)| *r > SURPLUS_EPSILON)
            .collect();
        for (resource, rate) in surplus {
            let producer = self.producer_for[&resource];
            self.add_flow(EdgeEndpoint::Node(producer), EdgeEndpoint::External, &resource, rate);
        }
    }

    #[instrument(level = "trace", skip(self, consumer), fields(resource = %resource, rate))]
    fn demand(&mut self, resource: ResourceId, rate: f64, consumer: EdgeEndpoint) -> Result<()> {
        if rate <= SURPLUS_EPSILON {
            return Ok(());
        }
        let (from_surplus, remaining) = self.draw_from_surplus(&resource, rate);
        if from_surplus > 0.0 {
            let producer = self.producer_for[&resource];
            trace!(from_surplus, "satisfied partially from surplus");
            self.add_flow(EdgeEndpoint::Node(producer), consumer, &resource, from_surplus);
        }
        if remaining <= SURPLUS_EPSILON {
            return Ok(());
        }
        if self.is_raw(&resource) {
            self.record_raw(&resource, remaining, consumer);
            return Ok(());
        }
        self.demand_from_recipe(resource, remaining, consumer)
    }

    fn draw_from_surplus(&mut self, resource: &ResourceId, rate: f64) -> (f64, f64) {
        let available = self.surplus.get(resource).copied().unwrap_or(0.0);
        let from_surplus = available.min(rate);
        if from_surplus > 0.0 {
            self.surplus.insert(resource.clone(), available - from_surplus);
        }
        (from_surplus, rate - from_surplus)
    }

    fn is_raw(&self, resource: &ResourceId) -> bool {
        if self.config.raw.contains(resource) {
            return true;
        }
        self.db
            .recipes_producing(resource.name())
            .next()
            .is_none()
    }

    fn record_raw(&mut self, resource: &ResourceId, rate: f64, consumer: EdgeEndpoint) {
        trace!(?resource, rate, "raw input");
        *self.raw_inputs.entry(resource.clone()).or_default() += rate;
        self.add_flow(EdgeEndpoint::External, consumer, resource, rate);
    }

    fn demand_from_recipe(
        &mut self,
        resource: ResourceId,
        rate: f64,
        consumer: EdgeEndpoint,
    ) -> Result<()> {
        let recipe_id = self.pick_recipe(&resource)?;
        let recipe = self
            .db
            .recipe(&recipe_id)
            .ok_or_else(|| Error::UnknownRecipe(recipe_id.clone()))?
            .clone();
        if !recipe.produces(resource.name()) {
            return Err(Error::RecipeDoesNotProduce(recipe_id, resource));
        }
        if self.visiting.contains(&recipe_id) {
            return Err(Error::Cycle(recipe_id));
        }
        let node_id = self.ensure_node(&recipe)?;
        let yield_per_run = recipe.expected_yield(resource.name());
        if yield_per_run <= 0.0 {
            return Err(Error::NoYield(recipe_id, resource));
        }
        let new_runs = rate / yield_per_run;
        self.add_runs(node_id, &recipe, new_runs);
        self.bank_coproducts(&recipe, &resource, new_runs);
        self.add_flow(EdgeEndpoint::Node(node_id), consumer, &resource, rate);
        self.recurse_ingredients(&recipe, new_runs, node_id)?;
        Ok(())
    }

    fn ensure_node(&mut self, recipe: &Recipe) -> Result<NodeId> {
        if let Some(&id) = self.producer_for.get(&recipe.products[0].resource) {
            if self.nodes[id.index()].recipe == recipe.id {
                return Ok(id);
            }
        }
        let machine_id = self.pick_machine(&recipe.category)?;
        let id = NodeId::new(self.nodes.len());
        trace!(node = %id, recipe = %recipe.id, machine = %machine_id, "creating production node");
        self.nodes.push(ProductionNode {
            id,
            recipe: recipe.id.clone(),
            machine: machine_id,
            runs_per_second: 0.0,
            machines_needed: 0.0,
        });
        for product in &recipe.products {
            self.producer_for.insert(product.resource.clone(), id);
        }
        Ok(id)
    }

    fn add_runs(&mut self, node_id: NodeId, recipe: &Recipe, runs: f64) {
        let node = &mut self.nodes[node_id.index()];
        node.runs_per_second += runs;
        let machine = self
            .db
            .machine(&node.machine)
            .expect("machine was picked from db");
        let cps = machine.crafts_per_second(recipe.crafting_time);
        node.machines_needed = if cps > 0.0 { node.runs_per_second / cps } else { 0.0 };
    }

    fn bank_coproducts(&mut self, recipe: &Recipe, primary: &ResourceId, new_runs: f64) {
        for product in &recipe.products {
            if &product.resource == primary {
                continue;
            }
            let produced = product.expected_amount() * new_runs;
            if produced > 0.0 {
                *self.surplus.entry(product.resource.clone()).or_default() += produced;
            }
        }
    }

    fn recurse_ingredients(
        &mut self,
        recipe: &Recipe,
        new_runs: f64,
        node_id: NodeId,
    ) -> Result<()> {
        self.visiting.insert(recipe.id.clone());
        for ingredient in &recipe.ingredients {
            let ing_rate = ingredient.amount * new_runs;
            self.demand(
                ingredient.resource.clone(),
                ing_rate,
                EdgeEndpoint::Node(node_id),
            )?;
        }
        self.visiting.remove(&recipe.id);
        Ok(())
    }

    fn pick_recipe(&self, resource: &ResourceId) -> Result<RecipeId> {
        if let Some(id) = self.config.recipe_for.get(resource) {
            return Ok(id.clone());
        }
        let candidates: Vec<RecipeId> = self
            .db
            .recipes_producing(resource.name())
            .map(|r| r.id.clone())
            .collect();
        match candidates.len() {
            0 => Err(Error::NoRecipe(resource.clone())),
            1 => Ok(candidates.into_iter().next().unwrap()),
            _ => Err(Error::AmbiguousRecipe {
                resource: resource.clone(),
                candidates,
            }),
        }
    }

    fn pick_machine(&self, category: &CraftingCategory) -> Result<MachineId> {
        if let Some(id) = self.config.machine_for_category.get(category) {
            return Ok(id.clone());
        }
        self.db
            .machines_for_category(category)
            .max_by(|a, b| {
                a.crafting_speed
                    .partial_cmp(&b.crafting_speed)
                    .unwrap_or(Ordering::Equal)
            })
            .map(|m| m.id.clone())
            .ok_or_else(|| Error::NoMachineForCategory(category.clone()))
    }

    fn add_flow(&mut self, from: EdgeEndpoint, to: EdgeEndpoint, resource: &ResourceId, rate: f64) {
        let key = FlowKey {
            from,
            to,
            resource: resource.clone(),
        };
        *self.flows.entry(key).or_default() += rate;
    }
}

fn compute_outputs(edges: &[ProductionEdge]) -> HashMap<ResourceId, Rate> {
    let mut outputs: HashMap<ResourceId, Rate> = HashMap::new();
    for edge in edges {
        if matches!(edge.to, EdgeEndpoint::External) {
            *outputs.entry(edge.resource.clone()).or_insert(Rate::ZERO) += edge.rate;
        }
    }
    outputs
}
