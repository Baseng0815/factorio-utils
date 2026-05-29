use prototypes::{MachineId, RecipeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl NodeId {
    pub fn new(idx: usize) -> Self {
        Self(idx)
    }

    pub fn index(self) -> usize {
        self.0
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ProductionNode {
    pub id: NodeId,
    pub recipe: RecipeId,
    pub machine: MachineId,
    pub runs_per_second: f64,
    pub machines_needed: f64,
}

impl std::fmt::Display for ProductionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:.2} × {} (recipe {}, {:.3} runs/s)",
            self.id, self.machines_needed, self.machine, self.recipe, self.runs_per_second,
        )
    }
}
