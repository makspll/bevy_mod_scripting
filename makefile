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
NIGHTLY_VERSION=nightly-2024-01-24
BEVY_VERSION=0.11.2
GLAM_VERSION=0.24.1
CODEGEN_PATH=${PWD}/target/codegen
BEVY_PATH=${CODEGEN_PATH}/bevy
GLAM_PATH=${CODEGEN_PATH}/glam
OUTPUT_PATH=${CODEGEN_PATH}/output
build_test_in_package:
	@cargo test --no-run --lib --workspace $(TEST_NAME)
	@export OUTPUT=$$(find ./target/debug/deps/ -regex ".*${PACKAGE}[^.]*" -printf "%T@\t%Tc %6k KiB %p\n" | sort -n -r | awk '{print $$NF}' | head -1); \
	mv $${OUTPUT} ./target/debug/test_binary && echo "Using: $${OUTPUT}" && ls -v ./target/debug/ | grep "test_binary"

comp_benches:
	RUSTFLAGS="-g" cargo bench --no-run 

valgrind:
	valgrind --tool=callgrind \
			--dump-instr=yes \
			--collect-jumps=yes \
			--simulate-cache=yes \
			${EXEC} --bench  ${T_ID} 

install_bevy_api_gen:
	rustup install ${NIGHTLY_VERSION}
	cargo +${NIGHTLY_VERSION} install --path ./crates/bevy_api_gen

prepare_api_gen:
	mkdir ${CODEGEN_PATH} || true
	git clone https://github.com/bevyengine/bevy --branch v${BEVY_VERSION} --depth 1 ${BEVY_PATH} || true

generate_bevy:
	cd ${BEVY_PATH} && cargo +${NIGHTLY_VERSION} bevy-api-gen generate --output ${OUTPUT_PATH} -v

generate: install_bevy_api_gen prepare_api_gen generate_glam