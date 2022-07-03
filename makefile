# Create the benchmark executable with debugging symbols, but do not run it. We
# don't want valgrind to profile the compiler, so we have the "--no-run" flag. We
# also need debugging symbols so valgrind can track down source code
# appropriately. It blows my mind to this day that compiling with optimizations +
# debugging symbols is a thing. For so long I thought they were mutually
# exclusive.
# RUSTFLAGS="-g" cargo bench  --no-run

# Let's say this was the executable
EXEC="./target/release/"

# Now identify a single test that you want profiled. Test identifiers are
# printed in the console output, so I'll use the one that I posted earlier
T_ID="Hello/World"

TIME="10"
# Have valgrind profile criterion running our benchmark for 10 seconds


# # valgrind outputs a callgrind.out.<pid>. We can analyze this with kcachegrind
# kcachegrind

comp_benches:
	RUSTFLAGS="-g" cargo bench --no-run 

valgrind:
	valgrind --tool=callgrind \
			--dump-instr=yes \
			--collect-jumps=yes \
			--simulate-cache=yes \
			${EXEC} --bench  ${T_ID} 
generate_api:
	cargo run --features=lua54 -- \
	--json "api_gen/bevy.json" \
	--json "api_gen/glam.json" \
	--json "api_gen/bevy_ecs.json" \
	--config api_gen/api_gen_config.toml \
	> bevy_mod_scripting/src/rlua_host/api/generated.rs