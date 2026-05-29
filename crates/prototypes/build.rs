use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;

use serde_json::Value;

const DUMP_PATH: &str = "resources/data-raw-dump.json";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Origin {
    Base,
    SpaceAge,
}

fn main() {
    println!("cargo:rerun-if-changed={DUMP_PATH}");
    println!("cargo:rerun-if-changed=build.rs");

    let json = load_dump();
    let items = classify_iconned(&json, "item");
    let fluids = classify_iconned(&json, "fluid");
    let machines = classify_machines(&json);
    let recipes = classify_recipes(&json, &items, &fluids);
    let categories = classify_categories(&json, &recipes, &machines);

    let mut out = String::from("// Auto-generated from resources/data-raw-dump.json by build.rs.\n// DO NOT EDIT.\n\n");
    emit_impl(&mut out, "ItemId", &items);
    emit_impl(&mut out, "FluidId", &fluids);
    emit_impl(&mut out, "RecipeId", &recipes);
    emit_impl(&mut out, "MachineId", &machines);
    emit_impl(&mut out, "CraftingCategory", &categories);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    fs::write(out_dir.join("vanilla_ids.rs"), out).expect("write vanilla_ids.rs");
}

fn load_dump() -> Value {
    let file = File::open(DUMP_PATH).unwrap_or_else(|e| panic!("open {DUMP_PATH}: {e}"));
    serde_json::from_reader(BufReader::new(file)).expect("parse dump")
}

fn classify_iconned(json: &Value, key: &str) -> BTreeMap<String, Origin> {
    let mut out = BTreeMap::new();
    if let Some(obj) = json.get(key).and_then(Value::as_object) {
        for (name, value) in obj {
            out.insert(name.clone(), origin_from_icon(value));
        }
    }
    out
}

fn origin_from_icon(value: &Value) -> Origin {
    if let Some(p) = value.get("icon").and_then(Value::as_str) {
        return classify_path(p);
    }
    if let Some(layers) = value.get("icons").and_then(Value::as_array) {
        if let Some(p) = layers.first().and_then(|l| l.get("icon")).and_then(Value::as_str) {
            return classify_path(p);
        }
    }
    Origin::Base
}

fn classify_path(path: &str) -> Origin {
    if path.contains("__space-age__")
        || path.contains("__quality__")
        || path.contains("__elevated-rails__")
    {
        Origin::SpaceAge
    } else {
        Origin::Base
    }
}

fn classify_machines(json: &Value) -> BTreeMap<String, Origin> {
    let mut out = BTreeMap::new();
    for key in ["assembling-machine", "furnace", "rocket-silo", "mining-drill"] {
        if let Some(obj) = json.get(key).and_then(Value::as_object) {
            for (name, value) in obj {
                out.insert(name.clone(), origin_from_icon(value));
            }
        }
    }
    out
}

fn classify_recipes(
    json: &Value,
    items: &BTreeMap<String, Origin>,
    fluids: &BTreeMap<String, Origin>,
) -> BTreeMap<String, Origin> {
    let mut out = BTreeMap::new();
    let Some(recipes) = json.get("recipe").and_then(Value::as_object) else {
        return out;
    };
    for (name, value) in recipes {
        out.insert(name.clone(), recipe_origin(value, items, fluids));
    }
    out
}

fn recipe_origin(
    value: &Value,
    items: &BTreeMap<String, Origin>,
    fluids: &BTreeMap<String, Origin>,
) -> Origin {
    if value.get("icon").is_some() || value.get("icons").is_some() {
        return origin_from_icon(value);
    }
    if let Some(results) = value.get("results").and_then(Value::as_array) {
        for result in results {
            let ty = result.get("type").and_then(Value::as_str).unwrap_or("item");
            let n = result.get("name").and_then(Value::as_str).unwrap_or("");
            let lookup = match ty {
                "fluid" => fluids.get(n),
                _ => items.get(n),
            };
            if let Some(o) = lookup {
                return *o;
            }
        }
    }
    Origin::Base
}

fn classify_categories(
    json: &Value,
    recipes: &BTreeMap<String, Origin>,
    machines: &BTreeMap<String, Origin>,
) -> BTreeMap<String, Origin> {
    let mut out: BTreeMap<String, Origin> = BTreeMap::new();
    if let Some(rs) = json.get("recipe").and_then(Value::as_object) {
        for (rname, r) in rs {
            let cat = r.get("category").and_then(Value::as_str).unwrap_or("crafting");
            let origin = recipes.get(rname).copied().unwrap_or(Origin::Base);
            merge_origin(&mut out, cat.to_owned(), origin);
        }
    }
    for key in ["assembling-machine", "furnace", "rocket-silo"] {
        if let Some(ms) = json.get(key).and_then(Value::as_object) {
            for (mname, m) in ms {
                let origin = machines.get(mname).copied().unwrap_or(Origin::Base);
                if let Some(cats) = m.get("crafting_categories").and_then(Value::as_array) {
                    for c in cats {
                        if let Some(cs) = c.as_str() {
                            merge_origin(&mut out, cs.to_owned(), origin);
                        }
                    }
                }
            }
        }
    }
    out
}

fn merge_origin(out: &mut BTreeMap<String, Origin>, key: String, candidate: Origin) {
    let merged = match (out.get(&key).copied(), candidate) {
        (None, c) => c,
        (Some(Origin::Base), _) | (_, Origin::Base) => Origin::Base,
        _ => Origin::SpaceAge,
    };
    out.insert(key, merged);
}

fn emit_impl(out: &mut String, ty: &str, map: &BTreeMap<String, Origin>) {
    writeln!(out, "impl crate::{ty} {{").unwrap();
    let mut seen = BTreeMap::new();
    for (name, origin) in map {
        let Some(ident) = to_ident(name) else { continue };
        if seen.insert(ident.clone(), name.clone()).is_some() {
            continue;
        }
        if *origin == Origin::SpaceAge {
            writeln!(out, "    #[cfg(feature = \"space-age\")]").unwrap();
        }
        writeln!(
            out,
            "    pub const {ident}: Self = Self::from_static(\"{name}\");"
        )
        .unwrap();
    }
    writeln!(out, "}}\n").unwrap();
}

fn to_ident(name: &str) -> Option<String> {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_uppercase());
        } else if c == '-' || c == '_' || c == '.' {
            out.push('_');
        } else {
            return None;
        }
    }
    if out.is_empty() {
        return None;
    }
    if out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    Some(out)
}
