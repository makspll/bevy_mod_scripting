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
NIGHTLY_VERSION=2023-11-29
BEVY_VERSION=0.11.2
GLAM_VERSION=0.24.1
BEVY_PATH=./target/bevy
TEALR_PATH=./target/tealr
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
generate_api:
	cd bevy_api_gen && cargo run --release -- \
	--filters "bevy*","uuid*","glam*","core*","std*","alloc*" \
	--excludes "bevy_ecs*" \
	--json "../target/doc/uuid.json" \
	--json "../target/doc/bevy_ptr.json" \
	--json "../target/doc/bevy_app.json" \
	--json "../target/doc/bevy_diagnostic.json" \
	--json "../target/doc/bevy_asset.json" \
	--json "../target/doc/bevy_ecs.json" \
	--json "../target/doc/bevy_pbr.json" \
	--json "../target/doc/bevy_render.json" \
	--json "../target/doc/bevy_math.json" \
	--json "../target/doc/bevy_transform.json" \
	--json "../target/doc/bevy_sprite.json" \
	--json "../target/doc/bevy_ui.json" \
	--json "../target/doc/bevy_animation.json" \
	--json "../target/doc/bevy_core.json" \
	--json "../target/doc/bevy_core_pipeline.json" \
	--json "../target/doc/bevy_gltf.json" \
	--json "../target/doc/bevy_hierarchy.json" \
	--json "../target/doc/bevy_text.json" \
	--json "../target/doc/bevy_time.json" \
	--json "../target/doc/bevy_utils.json" \
	--json "../target/doc/bevy_reflect.json" \
	--json "../target/doc/bevy.json" \
	--json "../target/doc/glam.json" \
	--json "${HOME}/.rustup/toolchains/nightly-${NIGHTLY_VERSION}-x86_64-unknown-linux-gnu/share/doc/rust/json/core.json" \
	--json "${HOME}/.rustup/toolchains/nightly-${NIGHTLY_VERSION}-x86_64-unknown-linux-gnu/share/doc/rust/json/std.json" \
	--json "${HOME}/.rustup/toolchains/nightly-${NIGHTLY_VERSION}-x86_64-unknown-linux-gnu/share/doc/rust/json/alloc.json" \
	--config "../api_gen_config.toml" ${FLAGS} \
	--templates "../templates" \
	--output ../bevy_script_api/src/generated.rs

make_json_files:
	rustup install nightly-${NIGHTLY_VERSION}
	rustup component add rust-docs-json --toolchain nightly-${NIGHTLY_VERSION}
	git clone https://github.com/bevyengine/bevy --branch v${BEVY_VERSION} --depth 1 ${BEVY_PATH} || true
	git clone https://github.com/lenscas/tealr --branch v0.9.0-alpha4 --depth 1 ${TEALR_PATH} || true
	mkdir -p ./target/doc/
	cd ${BEVY_PATH} && RUSTDOCFLAGS="--document-hidden-items --document-private-items  -Zunstable-options  --output-format json" rustup run nightly-${NIGHTLY_VERSION} cargo doc --workspace
	cp ${BEVY_PATH}/target/doc/* ./target/doc/
	cd ${TEALR_PATH} && RUSTDOCFLAGS="--document-hidden-items --document-private-items  -Zunstable-options  --output-format json" rustup run nightly-${NIGHTLY_VERSION} cargo doc --features=mlua_vendored,mlua_lua54 --workspace
	cp ${TEALR_PATH}/target/doc/* ./target/doc/

install_bevy_api_gen:
	cd bevy_api_gen && cargo install --path .
