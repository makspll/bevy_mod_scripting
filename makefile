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

FLAGS=
PACKAGE=bevy_mod_scripting
TEST_NAME=
# # valgrind outputs a callgrind.out.<pid>. We can analyze this with kcachegrind
# kcachegrind

build_test_in_package:
	@RUSTFLAGS=-g cargo test --no-run  --all-targets --features ${TEST_FEATURES} --package ${PACKAGE} $(TEST_NAME) --message-format=json | jq -r 'first(select(.executable != null and .target.kind == ["test"])) | .executable' | xargs -I@ ln -fs @ ./target/debug/test_binary

run_lua_tests:
	cargo test --features=lua54 --package bevy_mod_scripting_lua --test lua_tests -- $(TEST_NAME)


comp_benches:
	RUSTFLAGS="-g" cargo bench --no-run 

valgrind:
	valgrind --tool=callgrind \
			--dump-instr=yes \
			--collect-jumps=yes \
			--simulate-cache=yes \
			${EXEC} --bench  ${T_ID} 
