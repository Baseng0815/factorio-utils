mod edge;
mod node;

pub use edge::{EdgeEndpoint, ProductionEdge};
pub use node::{NodeId, ProductionNode};

use std::collections::HashMap;

use prototypes::ResourceId;

use crate::rate::Rate;

#[derive(Debug, Clone)]
pub struct ProductionLine {
    pub nodes: Vec<ProductionNode>,
    pub edges: Vec<ProductionEdge>,
    pub raw_inputs: HashMap<ResourceId, Rate>,
    pub outputs: HashMap<ResourceId, Rate>,
}

impl ProductionLine {
    pub fn node(&self, id: NodeId) -> &ProductionNode {
        &self.nodes[id.index()]
    }

    pub fn incoming(&self, id: NodeId) -> impl Iterator<Item = &ProductionEdge> {
        self.edges
            .iter()
            .filter(move |e| matches!(e.to, EdgeEndpoint::Node(n) if n == id))
    }

    pub fn outgoing(&self, id: NodeId) -> impl Iterator<Item = &ProductionEdge> {
        self.edges
            .iter()
            .filter(move |e| matches!(e.from, EdgeEndpoint::Node(n) if n == id))
    }
}

impl std::fmt::Display for ProductionLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProductionLine {{ nodes: {}, edges: {}, raw_inputs: {}, outputs: {} }}",
            self.nodes.len(),
            self.edges.len(),
            self.raw_inputs.len(),
            self.outputs.len(),
        )
    }
}
