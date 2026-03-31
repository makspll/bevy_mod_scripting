use std::{fs, path::Path};

use anyhow::anyhow;
use regex::Regex;
use serde::Deserialize;

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
        let expected = r#"Benchmarking benchmark: build_time/bevy_mod_scripting_world::blocking_comp   time:   [1.150 s 1.150 s 1.150 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_world::codegen   time:   [0.340 s 0.340 s 0.340 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_world::total   time:   [1.490 s 1.490 s 1.490 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_display::blocking_comp   time:   [1.390 s 1.390 s 1.390 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_display::codegen   time:   [0.540 s 0.540 s 0.540 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_display::total   time:   [1.930 s 1.930 s 1.930 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_bindings_domain::blocking_comp   time:   [0.720 s 0.720 s 0.720 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_bindings_domain::codegen   time:   [0.220 s 0.220 s 0.220 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_bindings_domain::total   time:   [0.940 s 0.940 s 0.940 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_asset::blocking_comp   time:   [1.870 s 1.870 s 1.870 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_asset::codegen   time:   [0.850 s 0.850 s 0.850 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_asset::total   time:   [2.720 s 2.720 s 2.720 s]
Benchmarking benchmark: build_time/ladfile::blocking_comp   time:   [3.570 s 3.570 s 3.570 s]
Benchmarking benchmark: build_time/ladfile::codegen   time:   [6.840 s 6.840 s 6.840 s]
Benchmarking benchmark: build_time/ladfile::total   time:   [10.410 s 10.410 s 10.410 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_script::blocking_comp   time:   [1.410 s 1.410 s 1.410 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_script::codegen   time:   [2.830 s 2.830 s 2.830 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_script::total   time:   [4.240 s 4.240 s 4.240 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_bindings::blocking_comp   time:   [24.070 s 24.070 s 24.070 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_bindings::codegen   time:   [8.880 s 8.880 s 8.880 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_bindings::total   time:   [32.950 s 32.950 s 32.950 s]
Benchmarking benchmark: build_time/lua_language_server_lad_backend::blocking_comp   time:   [3.500 s 3.500 s 3.500 s]
Benchmarking benchmark: build_time/lua_language_server_lad_backend::codegen   time:   [4.230 s 4.230 s 4.230 s]
Benchmarking benchmark: build_time/lua_language_server_lad_backend::total   time:   [7.730 s 7.730 s 7.730 s]
Benchmarking benchmark: build_time/mdbook_lad_preprocessor::blocking_comp   time:   [3.880 s 3.880 s 3.880 s]
Benchmarking benchmark: build_time/mdbook_lad_preprocessor::codegen   time:   [4.890 s 4.890 s 4.890 s]
Benchmarking benchmark: build_time/mdbook_lad_preprocessor::total   time:   [8.770 s 8.770 s 8.770 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_core::blocking_comp   time:   [8.140 s 8.140 s 8.140 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_core::codegen   time:   [32.510 s 32.510 s 32.510 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_core::total   time:   [40.650 s 40.650 s 40.650 s]
Benchmarking benchmark: build_time/bevy_math_bms_bindings::blocking_comp   time:   [15.110 s 15.110 s 15.110 s]
Benchmarking benchmark: build_time/bevy_math_bms_bindings::codegen   time:   [111.500 s 111.500 s 111.500 s]
Benchmarking benchmark: build_time/bevy_math_bms_bindings::total   time:   [126.610 s 126.610 s 126.610 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_rhai::blocking_comp   time:   [9.320 s 9.320 s 9.320 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_rhai::codegen   time:   [65.510 s 65.510 s 65.510 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_rhai::total   time:   [74.830 s 74.830 s 74.830 s]
Benchmarking benchmark: build_time/bevy_reflect_bms_bindings::blocking_comp   time:   [81.840 s 81.840 s 81.840 s]
Benchmarking benchmark: build_time/bevy_reflect_bms_bindings::codegen   time:   [442.670 s 442.670 s 442.670 s]
Benchmarking benchmark: build_time/bevy_reflect_bms_bindings::total   time:   [524.510 s 524.510 s 524.510 s]
Benchmarking benchmark: build_time/bevy_input_bms_bindings::blocking_comp   time:   [5.830 s 5.830 s 5.830 s]
Benchmarking benchmark: build_time/bevy_input_bms_bindings::codegen   time:   [70.850 s 70.850 s 70.850 s]
Benchmarking benchmark: build_time/bevy_input_bms_bindings::total   time:   [76.680 s 76.680 s 76.680 s]
Benchmarking benchmark: build_time/bevy_color_bms_bindings::blocking_comp   time:   [4.420 s 4.420 s 4.420 s]
Benchmarking benchmark: build_time/bevy_color_bms_bindings::codegen   time:   [43.690 s 43.690 s 43.690 s]
Benchmarking benchmark: build_time/bevy_color_bms_bindings::total   time:   [48.110 s 48.110 s 48.110 s]
Benchmarking benchmark: build_time/bevy_time_bms_bindings::blocking_comp   time:   [2.290 s 2.290 s 2.290 s]
Benchmarking benchmark: build_time/bevy_time_bms_bindings::codegen   time:   [27.680 s 27.680 s 27.680 s]
Benchmarking benchmark: build_time/bevy_time_bms_bindings::total   time:   [29.970 s 29.970 s 29.970 s]
Benchmarking benchmark: build_time/bevy_transform_bms_bindings::blocking_comp   time:   [3.430 s 3.430 s 3.430 s]
Benchmarking benchmark: build_time/bevy_transform_bms_bindings::codegen   time:   [33.310 s 33.310 s 33.310 s]
Benchmarking benchmark: build_time/bevy_transform_bms_bindings::total   time:   [36.740 s 36.740 s 36.740 s]
Benchmarking benchmark: build_time/bevy_ecs_bms_bindings::blocking_comp   time:   [3.480 s 3.480 s 3.480 s]
Benchmarking benchmark: build_time/bevy_ecs_bms_bindings::codegen   time:   [35.010 s 35.010 s 35.010 s]
Benchmarking benchmark: build_time/bevy_ecs_bms_bindings::total   time:   [38.490 s 38.490 s 38.490 s]
Benchmarking benchmark: build_time/bevy_core_pipeline_bms_bindings::blocking_comp   time:   [4.270 s 4.270 s 4.270 s]
Benchmarking benchmark: build_time/bevy_core_pipeline_bms_bindings::codegen   time:   [19.850 s 19.850 s 19.850 s]
Benchmarking benchmark: build_time/bevy_core_pipeline_bms_bindings::total   time:   [24.120 s 24.120 s 24.120 s]
Benchmarking benchmark: build_time/ladfile_builder::blocking_comp   time:   [2.940 s 2.940 s 2.940 s]
Benchmarking benchmark: build_time/ladfile_builder::codegen   time:   [13.500 s 13.500 s 13.500 s]
Benchmarking benchmark: build_time/ladfile_builder::total   time:   [16.440 s 16.440 s 16.440 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_functions::blocking_comp   time:   [12.570 s 12.570 s 12.570 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_functions::codegen   time:   [55.070 s 55.070 s 55.070 s]
Benchmarking benchmark: build_time/bevy_mod_scripting_functions::total   time:   [67.640 s 67.640 s 67.640 s]
Benchmarking benchmark: build_time/script_integration_test_harness::blocking_comp   time:   [29.130 s 29.130 s 29.130 s]
Benchmarking benchmark: build_time/script_integration_test_harness::codegen   time:   [21.660 s 21.660 s 21.660 s]
Benchmarking benchmark: build_time/script_integration_test_harness::total   time:   [50.790 s 50.790 s 50.790 s]
Benchmarking benchmark: build_time/bevy_mod_scripting::blocking_comp   time:   [5.700 s 5.700 s 5.700 s]
Benchmarking benchmark: build_time/bevy_mod_scripting::codegen   time:   [0.650 s 0.650 s 0.650 s]
Benchmarking benchmark: build_time/bevy_mod_scripting::total   time:   [6.350 s 6.350 s 6.350 s]
Benchmarking benchmark: build_time/total::blocking_comp   time:   [230.030 s 230.030 s 230.030 s]
Benchmarking benchmark: build_time/total::codegen   time:   [1003.080 s 1003.080 s 1003.080 s]
Benchmarking benchmark: build_time/total::total   time:   [1233.110 s 1233.110 s 1233.110 s]
"#;
        pretty_assertions::assert_str_eq!(metrics, expected);
    }
}
