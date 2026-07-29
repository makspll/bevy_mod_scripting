use std::{fs, path::Path};

use anyhow::anyhow;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Phase {
    start: f64,
    end: f64,
}

#[derive(Debug, Deserialize)]
struct Unit {
    name: String,
    target: String,
    duration: f64,
    #[serde(default)]
    sections: Option<Vec<(String, Phase)>>, // new format
}

/// Extract `const UNIT_DATA = [...]`
fn extract_unit_data(html: &str) -> Option<String> {
    let re = Regex::new(r"(?s)const UNIT_DATA = (\[.*?\]);").ok()?;
    let caps = re.captures(html)?;
    Some(caps.get(1)?.as_str().to_string())
}

pub fn timings_html_to_criterion(
    html: &str,
    include_crates: Vec<String>,
) -> Result<String, anyhow::Error> {
    let json = extract_unit_data(&html).ok_or(anyhow!("UNIT_DATA not found in timings HTML"))?;

    let units: Vec<Unit> = serde_json::from_str(&json)?;

    Ok(convert(units, include_crates))
}

fn convert(units: Vec<Unit>, include_crates: Vec<String>) -> String {
    let mut out = String::new();
    let mut total_rmeta = 0.0;
    let mut total_codegen = 0.0;
    for u in units {
        let crate_name = normalize(&u.name, &u.target);
        if crate_name.contains("(test)") || !include_crates.contains(&crate_name) {
            continue;
        }

        let frontend = u.sections
            .iter()
            .flat_map(|i| i.iter())
            .find(|(n, _)| n == "frontend")
            .map(|(_, p)| p.end - p.start);

        let codegen = u.sections
            .iter()
            .flat_map(|i| i.iter())
            .find(|(n, _)| n == "codegen")
            .map(|(_, p)| p.end - p.start);


        let meta_name = format!("{}::blocking_comp", crate_name);
        let codegen_name = format!("{}::codegen", crate_name);
        let total_time_name = format!("{}::total", crate_name);

        if let Some(frontend) = frontend {
            write_metric(&mut out, &meta_name, frontend);
            total_rmeta += frontend;
        }
        if let Some(codegen) = codegen {
            write_metric(&mut out, &codegen_name, codegen);
            total_codegen += codegen;
        }
        write_metric(&mut out, &total_time_name, u.duration);
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
        "build_time/{name}   time:   [{:.3} s {:.3} s {:.3} s]",
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
    include_crates: Vec<String>,
) -> Result<String, anyhow::Error> {
    let html = fs::read_to_string(
        &build_dir
            .join("target")
            .join("cargo-timings")
            .join("cargo-timing.html"),
    )?;
    let criterion = process_cargo_timings_report(&html, include_crates)?;
    Ok(criterion)
}

fn process_cargo_timings_report(
    html: &str,
    include_crates: Vec<String>,
) -> Result<String, anyhow::Error> {
    let criterion = timings_html_to_criterion(&html, include_crates)?;
    Ok(criterion)
}

#[cfg(test)]
mod test {

    use crate::benches::process_cargo_timings_report;

    pub const TEST_TIMINGS: &str = include_str!("../test_assets/cargo_timings_cut.html");

    #[test]
    fn regression_test() {
        let res = process_cargo_timings_report(
            TEST_TIMINGS,
            vec![
                "bevy_mod_scripting_world".to_owned(),
                "bevy_mod_scripting_display".to_owned(),
                "bevy_mod_scripting_bindings_domain".to_owned(),
                "bevy_mod_scripting_asset".to_owned(),
                "ladfile".to_owned(),
                "bevy_mod_scripting_script".to_owned(),
                "bevy_mod_scripting_bindings".to_owned(),
                "lua_language_server_lad_backend".to_owned(),
                "mdbook_lad_preprocessor".to_owned(),
                "bevy_mod_scripting_core".to_owned(),
                "bevy_math_bms_bindings".to_owned(),
                "bevy_mod_scripting_rhai".to_owned(),
                "bevy_reflect_bms_bindings".to_owned(),
                "bevy_input_bms_bindings".to_owned(),
                "bevy_color_bms_bindings".to_owned(),
                "bevy_time_bms_bindings".to_owned(),
                "bevy_transform_bms_bindings".to_owned(),
                "bevy_ecs_bms_bindings".to_owned(),
                "bevy_core_pipeline_bms_bindings".to_owned(),
                "ladfile_builder".to_owned(),
                "bevy_mod_scripting_functions".to_owned(),
                "script_integration_test_harness".to_owned(),
                "bevy_mod_scripting".to_owned(),
            ],
        );

        let metrics = res.unwrap();
        let expected = r#"build_time/bevy_mod_scripting_world::blocking_comp   time:   [0.680 s 0.680 s 0.680 s]
build_time/bevy_mod_scripting_world::codegen   time:   [3.120 s 3.120 s 3.120 s]
build_time/bevy_mod_scripting_world::total   time:   [3.800 s 3.800 s 3.800 s]
build_time/bevy_mod_scripting_display::blocking_comp   time:   [0.700 s 0.700 s 0.700 s]
build_time/bevy_mod_scripting_display::codegen   time:   [1.740 s 1.740 s 1.740 s]
build_time/bevy_mod_scripting_display::total   time:   [2.440 s 2.440 s 2.440 s]
build_time/bevy_mod_scripting_bindings_domain::blocking_comp   time:   [0.740 s 0.740 s 0.740 s]
build_time/bevy_mod_scripting_bindings_domain::codegen   time:   [0.390 s 0.390 s 0.390 s]
build_time/bevy_mod_scripting_bindings_domain::total   time:   [1.130 s 1.130 s 1.130 s]
build_time/bevy_mod_scripting_asset::blocking_comp   time:   [0.990 s 0.990 s 0.990 s]
build_time/bevy_mod_scripting_asset::codegen   time:   [1.900 s 1.900 s 1.900 s]
build_time/bevy_mod_scripting_asset::total   time:   [2.890 s 2.890 s 2.890 s]
build_time/bevy_mod_scripting_script::blocking_comp   time:   [0.610 s 0.610 s 0.610 s]
build_time/bevy_mod_scripting_script::codegen   time:   [9.880 s 9.880 s 9.880 s]
build_time/bevy_mod_scripting_script::total   time:   [10.490 s 10.490 s 10.490 s]
build_time/bevy_mod_scripting_bindings::blocking_comp   time:   [17.610 s 17.610 s 17.610 s]
build_time/bevy_mod_scripting_bindings::codegen   time:   [61.200 s 61.200 s 61.200 s]
build_time/bevy_mod_scripting_bindings::total   time:   [78.810 s 78.810 s 78.810 s]
build_time/bevy_mod_scripting_core::blocking_comp   time:   [5.310 s 5.310 s 5.310 s]
build_time/bevy_mod_scripting_core::codegen   time:   [43.800 s 43.800 s 43.800 s]
build_time/bevy_mod_scripting_core::total   time:   [49.110 s 49.110 s 49.110 s]
build_time/ladfile::blocking_comp   time:   [1.620 s 1.620 s 1.620 s]
build_time/ladfile::codegen   time:   [16.620 s 16.620 s 16.620 s]
build_time/ladfile::total   time:   [18.240 s 18.240 s 18.240 s]
build_time/bevy_mod_scripting_rhai::blocking_comp   time:   [4.380 s 4.380 s 4.380 s]
build_time/bevy_mod_scripting_rhai::codegen   time:   [78.440 s 78.440 s 78.440 s]
build_time/bevy_mod_scripting_rhai::total   time:   [82.820 s 82.820 s 82.820 s]
build_time/bevy_core_pipeline_bms_bindings::blocking_comp   time:   [1.060 s 1.060 s 1.060 s]
build_time/bevy_core_pipeline_bms_bindings::codegen   time:   [18.890 s 18.890 s 18.890 s]
build_time/bevy_core_pipeline_bms_bindings::total   time:   [19.950 s 19.950 s 19.950 s]
build_time/bevy_time_bms_bindings::blocking_comp   time:   [0.940 s 0.940 s 0.940 s]
build_time/bevy_time_bms_bindings::codegen   time:   [27.640 s 27.640 s 27.640 s]
build_time/bevy_time_bms_bindings::total   time:   [28.580 s 28.580 s 28.580 s]
build_time/bevy_color_bms_bindings::blocking_comp   time:   [1.770 s 1.770 s 1.770 s]
build_time/bevy_color_bms_bindings::codegen   time:   [46.810 s 46.810 s 46.810 s]
build_time/bevy_color_bms_bindings::total   time:   [48.580 s 48.580 s 48.580 s]
build_time/bevy_ecs_bms_bindings::blocking_comp   time:   [1.320 s 1.320 s 1.320 s]
build_time/bevy_ecs_bms_bindings::codegen   time:   [40.040 s 40.040 s 40.040 s]
build_time/bevy_ecs_bms_bindings::total   time:   [41.360 s 41.360 s 41.360 s]
build_time/bevy_transform_bms_bindings::blocking_comp   time:   [1.460 s 1.460 s 1.460 s]
build_time/bevy_transform_bms_bindings::codegen   time:   [33.890 s 33.890 s 33.890 s]
build_time/bevy_transform_bms_bindings::total   time:   [35.350 s 35.350 s 35.350 s]
build_time/bevy_math_bms_bindings::blocking_comp   time:   [6.310 s 6.310 s 6.310 s]
build_time/bevy_math_bms_bindings::codegen   time:   [179.600 s 179.600 s 179.600 s]
build_time/bevy_math_bms_bindings::total   time:   [185.910 s 185.910 s 185.910 s]
build_time/bevy_input_bms_bindings::blocking_comp   time:   [2.340 s 2.340 s 2.340 s]
build_time/bevy_input_bms_bindings::codegen   time:   [89.130 s 89.130 s 89.130 s]
build_time/bevy_input_bms_bindings::total   time:   [91.470 s 91.470 s 91.470 s]
build_time/bevy_reflect_bms_bindings::blocking_comp   time:   [35.240 s 35.240 s 35.240 s]
build_time/bevy_reflect_bms_bindings::codegen   time:   [397.330 s 397.330 s 397.330 s]
build_time/bevy_reflect_bms_bindings::total   time:   [432.570 s 432.570 s 432.570 s]
build_time/lua_language_server_lad_backend::blocking_comp   time:   [1.130 s 1.130 s 1.130 s]
build_time/lua_language_server_lad_backend::codegen   time:   [9.310 s 9.310 s 9.310 s]
build_time/lua_language_server_lad_backend::total   time:   [10.440 s 10.440 s 10.440 s]
build_time/bevy_mod_scripting_functions::blocking_comp   time:   [4.310 s 4.310 s 4.310 s]
build_time/bevy_mod_scripting_functions::codegen   time:   [120.900 s 120.900 s 120.900 s]
build_time/bevy_mod_scripting_functions::total   time:   [125.210 s 125.210 s 125.210 s]
build_time/ladfile_builder::blocking_comp   time:   [1.440 s 1.440 s 1.440 s]
build_time/ladfile_builder::codegen   time:   [13.780 s 13.780 s 13.780 s]
build_time/ladfile_builder::total   time:   [15.220 s 15.220 s 15.220 s]
build_time/bevy_mod_scripting::blocking_comp   time:   [0.400 s 0.400 s 0.400 s]
build_time/bevy_mod_scripting::codegen   time:   [0.560 s 0.560 s 0.560 s]
build_time/bevy_mod_scripting::total   time:   [0.960 s 0.960 s 0.960 s]
build_time/script_integration_test_harness::blocking_comp   time:   [5.370 s 5.370 s 5.370 s]
build_time/script_integration_test_harness::codegen   time:   [80.190 s 80.190 s 80.190 s]
build_time/script_integration_test_harness::total   time:   [85.560 s 85.560 s 85.560 s]
build_time/mdbook_lad_preprocessor::blocking_comp   time:   [1.020 s 1.020 s 1.020 s]
build_time/mdbook_lad_preprocessor::codegen   time:   [11.680 s 11.680 s 11.680 s]
build_time/mdbook_lad_preprocessor::total   time:   [12.700 s 12.700 s 12.700 s]
build_time/total::blocking_comp   time:   [96.750 s 96.750 s 96.750 s]
build_time/total::codegen   time:   [1286.840 s 1286.840 s 1286.840 s]
build_time/total::total   time:   [1383.590 s 1383.590 s 1383.590 s]
"#;
        pretty_assertions::assert_str_eq!(metrics, expected);
    }
}
