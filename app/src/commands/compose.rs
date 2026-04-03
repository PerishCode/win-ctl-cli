use crate::{commands::window, core::RuntimeConfig};
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposeDocument {
    pub kind: String,
    #[serde(rename = "steps")]
    pub steps: Vec<ComposeStep>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposeStep {
    pub target: String,
    pub query: String,
    #[serde(default)]
    pub input: Option<ComposeInput>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposeInput {
    pub hwnd: isize,
}

#[derive(Debug, serde::Serialize)]
pub struct ComposeOutput {
    pub kind: &'static str,
    pub results: Vec<ComposeResult>,
}

#[derive(Debug, serde::Serialize)]
pub struct ComposeResult {
    pub target: &'static str,
    pub query: &'static str,
    pub value: ComposeValue,
}

fn compose_query_name(query: &str) -> &'static str {
    match query {
        "bounds" => "bounds",
        "class" => "class",
        "pid" => "pid",
        _ => unreachable!(),
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(tag = "kind", content = "value")]
pub enum ComposeValue {
    Bounds(window::ActiveWindowBounds),
    Class(String),
    Pid(u32),
}

pub fn run(runtime: &RuntimeConfig, file: &Path) -> Result<(), String> {
    let contents = fs::read_to_string(file)
        .map_err(|err| format!("failed to read compose file {}: {err}", file.display()))?;
    let doc: ComposeDocument = serde_json::from_str(&contents)
        .map_err(|err| format!("failed to parse compose file {}: {err}", file.display()))?;

    if doc.kind != "compose" {
        return Err(String::from("compose document kind must be \"compose\""));
    }

    if doc.steps.is_empty() {
        return Err(String::from("compose document must contain one step"));
    }

    if doc.steps.len() != 1 {
        return Err(String::from("compose smoke path supports exactly one step"));
    }

    let step = &doc.steps[0];
    let query = step.query.clone();
    let (target, value) = match step.target.as_str() {
        "window.active" => {
            let value = match query.as_str() {
                "bounds" => ComposeValue::Bounds(window::active_bounds(runtime)?),
                "class" => ComposeValue::Class(window::active_class(runtime)?),
                "pid" => ComposeValue::Pid(window::active_pid(runtime)?),
                _ => {
                    return Err(String::from(
                        "only queries \"bounds\", \"class\", and \"pid\" are supported for target \"window.active\"",
                    ));
                }
            };
            ("window.active", value)
        }
        "window.bounds" => {
            if query != "bounds" {
                return Err(String::from(
                    "only query \"bounds\" is supported for target \"window.bounds\"",
                ));
            }
            let hwnd = step
                .input
                .as_ref()
                .ok_or_else(|| {
                    String::from("target \"window.bounds\" requires input { \"hwnd\" }")
                })?
                .hwnd;
            (
                "window.bounds",
                ComposeValue::Bounds(window::window_bounds_for_compose(runtime, hwnd)?),
            )
        }
        _ => {
            return Err(String::from(
                "only { \"target\": \"window.active\", ... } and { \"target\": \"window.bounds\", ... } are supported",
            ));
        }
    };
    let output = ComposeOutput {
        kind: "compose",
        results: vec![ComposeResult {
            target,
            query: compose_query_name(query.as_str()),
            value,
        }],
    };

    println!(
        "{}",
        serde_json::to_string(&output).map_err(|err| err.to_string())?
    );
    Ok(())
}
