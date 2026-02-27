//! Binary for generating scenario schema

use std::io::{self, Write};

use bevy_mod_scripting_test_scenario_syntax::serialize_schema;

fn main() -> io::Result<()> {
    let output = serialize_schema()?;
    let mut stdout = std::io::stdout();
    stdout.write_all(output.as_bytes())
}
