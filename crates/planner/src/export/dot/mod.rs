mod escape;
mod label;

use std::io::{self, Write};

use tracing::instrument;

use recipes::{Database, ResourceId};

use crate::icons::IconResolver;
use crate::line::{EdgeEndpoint, ProductionEdge, ProductionLine};

use self::label::{edge_label, machine_label, sink_label, source_label};

#[instrument(level = "debug", skip_all)]
pub fn write_dot<W: Write>(
    line: &ProductionLine,
    db: &Database,
    resolver: Option<&dyn IconResolver>,
    out: &mut W,
) -> io::Result<()> {
    writeln!(out, "digraph ProductionLine {{")?;
    writeln!(out, "    rankdir=LR;")?;
    writeln!(out, "    node [shape=plaintext];")?;
    writeln!(out, "    edge [fontsize=10];")?;
    writeln!(out)?;
    write_machine_nodes(line, db, resolver, out)?;
    write_source_nodes(line, db, resolver, out)?;
    write_sink_nodes(line, db, resolver, out)?;
    writeln!(out)?;
    write_edges(line, db, resolver, out)?;
    writeln!(out, "}}")
}

fn write_machine_nodes<W: Write>(
    line: &ProductionLine,
    db: &Database,
    resolver: Option<&dyn IconResolver>,
    out: &mut W,
) -> io::Result<()> {
    for node in &line.nodes {
        let label = machine_label(node, db, resolver);
        writeln!(out, "    {} [label=<{}>];", node_dot_id(node.id), label)?;
    }
    Ok(())
}

fn write_source_nodes<W: Write>(
    line: &ProductionLine,
    db: &Database,
    resolver: Option<&dyn IconResolver>,
    out: &mut W,
) -> io::Result<()> {
    for (resource, rate) in &line.raw_inputs {
        let label = source_label(resource, *rate, db, resolver);
        writeln!(
            out,
            "    {} [label=<{}>, style=dashed];",
            source_dot_id(resource),
            label,
        )?;
    }
    Ok(())
}

fn write_sink_nodes<W: Write>(
    line: &ProductionLine,
    db: &Database,
    resolver: Option<&dyn IconResolver>,
    out: &mut W,
) -> io::Result<()> {
    for (resource, rate) in &line.outputs {
        let label = sink_label(resource, *rate, db, resolver);
        writeln!(
            out,
            "    {} [label=<{}>, style=bold];",
            sink_dot_id(resource),
            label,
        )?;
    }
    Ok(())
}

fn write_edges<W: Write>(
    line: &ProductionLine,
    db: &Database,
    _resolver: Option<&dyn IconResolver>,
    out: &mut W,
) -> io::Result<()> {
    for edge in &line.edges {
        let from = endpoint_from(edge);
        let to = endpoint_to(edge);
        let label = edge_label(edge, db);
        writeln!(out, "    {from} -> {to} [label=<{label}>];")?;
    }
    Ok(())
}

fn endpoint_from(edge: &ProductionEdge) -> String {
    match edge.from {
        EdgeEndpoint::External => source_dot_id(&edge.resource),
        EdgeEndpoint::Node(id) => node_dot_id(id),
    }
}

fn endpoint_to(edge: &ProductionEdge) -> String {
    match edge.to {
        EdgeEndpoint::External => sink_dot_id(&edge.resource),
        EdgeEndpoint::Node(id) => node_dot_id(id),
    }
}

fn node_dot_id(id: crate::line::NodeId) -> String {
    format!("n{}", id.index())
}

fn source_dot_id(resource: &ResourceId) -> String {
    format!("\"src:{}\"", resource.as_str())
}

fn sink_dot_id(resource: &ResourceId) -> String {
    format!("\"sink:{}\"", resource.as_str())
}
