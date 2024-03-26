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
BEVY_VERSION=0.13.1
GLAM_VERSION=0.25.0
CODEGEN_PATH=${PWD}/target/codegen
BEVY_PATH=${CODEGEN_PATH}/bevy
GLAM_PATH=${CODEGEN_PATH}/glam
OUTPUT_PATH=${CODEGEN_PATH}/output
GENERATED_SRC_PATH=./crates/bevy_script_api/src/providers
GEN_BEVY_FEATURES=bevy_asset,bevy_gltf,bevy_animation,bevy_core_pipeline,bevy_ui,bevy_pbr,bevy_render,bevy_text,bevy_sprite,file_watcher,multi-threaded

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
	rm -rf ${OUTPUT_PATH}/* 
	cd ${BEVY_PATH} && git fetch --tags && git checkout v${BEVY_VERSION}

generate_bevy:
	cd ${BEVY_PATH} && cargo +${NIGHTLY_VERSION} bevy-api-gen generate --output ${OUTPUT_PATH} -v --template-args '{ "self_is_bevy_script_api": true}' --features ${GEN_BEVY_FEATURES} --no-default-features

collect_bevy:
	cd ${BEVY_PATH} && cargo +${NIGHTLY_VERSION} bevy-api-gen collect --output ${OUTPUT_PATH} -v --template-args '{ "self_is_bevy_script_api": true}'

deletion_confirmation:
	@echo -n "This action will delete ALL files in directories: '${GENERATED_SRC_PATH}' amd ${OUTPUT_PATH} (y/N) "
	@read ans && [ $${ans:-N} = y ]

install_generated_files:
	mkdir ${GENERATED_SRC_PATH} || true
	rm -rf ${GENERATED_SRC_PATH}/* || true
	find ${OUTPUT_PATH} -name "*.rs" -exec cp {} ${GENERATED_SRC_PATH} \;

generate: deletion_confirmation install_bevy_api_gen prepare_api_gen generate_bevy collect_bevy install_generated_files
