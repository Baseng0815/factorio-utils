use prototypes::{Database, IconRef, MachineId, ResourceId};

use crate::icons::IconResolver;
use crate::line::{ProductionEdge, ProductionNode};
use crate::rate::Rate;

use super::escape::escape_html;

const ICON_CELL: u32 = 48;

pub(super) fn machine_label(
    node: &ProductionNode,
    db: &Database,
    resolver: Option<&dyn IconResolver>,
) -> String {
    let icon = lookup_machine_icon(&node.machine, db);
    let icon_cell = icon_cell(icon, resolver);
    let count = format!("{:.2} ×", node.machines_needed);
    let machine_name = escape_html(node.machine.as_str());
    let recipe_name = escape_html(node.recipe.as_str());
    let runs = format!("{:.3} runs/s", node.runs_per_second);
    table(&[
        &icon_cell,
        &cell(&format!("{count} {machine_name}")),
        &cell(&recipe_name),
        &cell(&runs),
    ])
}

pub(super) fn source_label(
    resource: &ResourceId,
    rate: Rate,
    db: &Database,
    resolver: Option<&dyn IconResolver>,
) -> String {
    resource_label(resource, rate, db, resolver, "raw")
}

pub(super) fn sink_label(
    resource: &ResourceId,
    rate: Rate,
    db: &Database,
    resolver: Option<&dyn IconResolver>,
) -> String {
    resource_label(resource, rate, db, resolver, "out")
}

fn resource_label(
    resource: &ResourceId,
    rate: Rate,
    db: &Database,
    resolver: Option<&dyn IconResolver>,
    tag: &str,
) -> String {
    let icon = lookup_resource_icon(resource, db);
    let icon_cell = icon_cell(icon, resolver);
    let name = escape_html(resource.as_str());
    let rate_text = escape_html(&rate.to_string());
    table(&[
        &icon_cell,
        &cell(&format!("[{tag}] {name}")),
        &cell(&rate_text),
    ])
}

pub(super) fn edge_label(edge: &ProductionEdge, db: &Database) -> String {
    let resource = escape_html(edge.resource.as_str());
    let rate = escape_html(&edge.rate.to_string());
    let icon = lookup_resource_icon(&edge.resource, db);
    let icon_marker = if icon.is_some() { "•" } else { "" };
    format!("{icon_marker} {resource}<BR/>{rate}").trim_start().to_owned()
}

fn lookup_machine_icon<'a>(id: &MachineId, db: &'a Database) -> Option<&'a IconRef> {
    db.machines.get(id).and_then(|m| m.icon.as_ref())
}

fn lookup_resource_icon<'a>(resource: &ResourceId, db: &'a Database) -> Option<&'a IconRef> {
    match resource {
        ResourceId::Item(id) => db.items.get(id).and_then(|i| i.icon.as_ref()),
        ResourceId::Fluid(id) => db.fluids.get(id).and_then(|f| f.icon.as_ref()),
    }
}

fn icon_cell(icon: Option<&IconRef>, resolver: Option<&dyn IconResolver>) -> String {
    let Some(icon) = icon else { return String::new() };
    let Some(resolver) = resolver else { return String::new() };
    let Some(path) = resolver.resolve(icon) else { return String::new() };
    format!(
        r#"<TR><TD FIXEDSIZE="TRUE" WIDTH="{ICON_CELL}" HEIGHT="{ICON_CELL}"><IMG SRC="{}" SCALE="TRUE"/></TD></TR>"#,
        escape_html(&path.to_string_lossy()),
    )
}

fn cell(content: &str) -> String {
    format!("<TR><TD>{content}</TD></TR>")
}

fn table(rows: &[&str]) -> String {
    let mut s = String::from(r#"<TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0" CELLPADDING="4">"#);
    for row in rows {
        s.push_str(row);
    }
    s.push_str("</TABLE>");
    s
}
