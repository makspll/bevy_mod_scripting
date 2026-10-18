# Script integration test harness

Tests are script files under `assets/tests/**`, driven by `.bmsscenario` files. Add a script (per language) plus a scenario rather than a Rust test when checking script-facing behaviour.

* Run: `cargo test --test script_tests --features lua54,rune -- <name>` (`rune` also runs the `.rn` scripts). Slow first build, run it in the background.
* Look at an existing scenario with both languages before writing a new one; copy its structure.
* Languages differ in syntax and indexing; check a sibling test in another language.
