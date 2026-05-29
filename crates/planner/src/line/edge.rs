use prototypes::ResourceId;

use crate::rate::Rate;

use super::node::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeEndpoint {
    External,
    Node(NodeId),
}

impl std::fmt::Display for EdgeEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::External => f.write_str("external"),
            Self::Node(id) => write!(f, "{id}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProductionEdge {
    pub from: EdgeEndpoint,
    pub to: EdgeEndpoint,
    pub resource: ResourceId,
    pub rate: Rate,
}

impl std::fmt::Display for ProductionEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} --[{} {}]--> {}",
            self.from, self.rate, self.resource, self.to,
        )
    }
}
