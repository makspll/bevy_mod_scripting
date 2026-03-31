use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use regex::Regex;
use serde::Deserialize;

/// Matches the embedded `var data = {...};`
fn extract_timings_json(html: &str) -> Option<String> {
    let re = Regex::new(r"var data = (\{.*?\});").ok()?;
    let caps = re.captures(html)?;
    Some(caps.get(1)?.as_str().to_string())
}

#[derive(Debug, Deserialize)]
struct Timing {
    crate_id: String,
    duration: f64,
    #[serde(default)]
    phases: Vec<Phase>,
}

#[derive(Debug, Deserialize)]
struct Phase {
    name: String,
    duration: f64,
}

#[derive(Debug, Deserialize)]
struct CargoTimings {
    timings: Vec<Timing>,
}

#[derive(Debug, Deserialize)]
struct Unit {
    name: String,
    target: String,
    duration: f64,
    rmeta_time: Option<f64>,
}

/// Extract `const UNIT_DATA = [...]`
fn extract_unit_data(html: &str) -> Option<String> {
    let re = Regex::new(r"(?s)const UNIT_DATA = (\[.*?\]);").ok()?;
    let caps = re.captures(html)?;
    Some(caps.get(1)?.as_str().to_string())
}

pub fn timings_html_to_criterion(html: &str) -> Result<String, anyhow::Error> {
    let json = extract_unit_data(&html).ok_or(anyhow!("UNIT_DATA not found in timings HTML"))?;

    let units: Vec<Unit> = serde_json::from_str(&json)?;

    Ok(convert(units))
}

fn convert(units: Vec<Unit>) -> String {
    let mut out = String::new();
    let mut total_rmeta = 0.0;
    let mut total_codegen = 0.0;
    for u in units {
        let crate_name = normalize(&u.name, &u.target);
        if crate_name.contains("(test)") {
            continue;
        }

        let rmeta_time = if let Some(rmeta_time) = u.rmeta_time {
            rmeta_time
        } else {
            continue;
        };

        // Phase split (best-effort)
        let total_compile_time_s = u.duration;
        let codegen_time = u.duration - rmeta_time;

        let meta_name = format!("{}::blocking_comp", crate_name);
        let codegen_name = format!("{}::codegen", crate_name);
        let total_time_name = format!("{}::total", crate_name);
        total_rmeta += rmeta_time;
        total_codegen += codegen_time;

        write_metric(&mut out, &meta_name, rmeta_time);
        write_metric(&mut out, &codegen_name, codegen_time);
        write_metric(&mut out, &total_time_name, total_compile_time_s);
    }
    write_metric(&mut out, &format!("total::blocking_comp"), total_rmeta);
    write_metric(&mut out, &format!("total::codegen"), total_codegen);
    write_metric(
        &mut out,
        &format!("total::total"),
        total_rmeta + total_codegen,
    );

    out
}

fn write_metric(out: &mut String, name: &str, seconds: f64) {
    use std::fmt::Write;

    let _ = writeln!(
        out,
        "Benchmarking benchmark: build_time/{name}   time:   [{:.3} s {:.3} s {:.3} s]",
        seconds, seconds, seconds
    );
}

fn normalize(name: &str, target: &str) -> String {
    if target.trim().is_empty() {
        name.to_string()
    } else {
        format!("{} [{}]", name, target.trim())
    }
}

pub fn read_cargo_timings_report(
    build_dir: &Path,
    crates_in_workspace: Vec<String>,
) -> Result<String, anyhow::Error> {
    let html = fs::read_to_string(
        &build_dir
            .join("target")
            .join("cargo-timings")
            .join("cargo-timing.html"),
    )?;
    let criterion = process_cargo_timings_report(&html, crates_in_workspace)?;
    Ok(criterion)
}

fn process_cargo_timings_report(
    html: &str,
    crates_in_workspace: Vec<String>,
) -> Result<String, anyhow::Error> {
    let criterion = timings_html_to_criterion(&html)?;
    Ok(criterion)
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{benches::process_cargo_timings_report, read_cargo_timings_report};

    pub const TEST_TIMINGS: &str = include_str!("../test_assets/cargo_timings_cut.html");

    #[test]
    fn regression_test() {
        let res = process_cargo_timings_report(
            TEST_TIMINGS,
            vec![
                "bevy_mod_scripting_world".to_owned(),
                "bevy_mod_scripting".to_owned(),
            ],
        );

        let metrics = res.unwrap();
        let expected = "";
        pretty_assertions::assert_str_eq!(metrics, expected);
    }
}
