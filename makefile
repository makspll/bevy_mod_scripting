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
BEVY_VERSION=0.11.2
GLAM_VERSION=0.24.1
FILE_NAME=generated.rs


generate_all: make_json_files make_json_files_aarch64
	${MAKE} generate_api_aarch64 FILE_NAME=generated_scalar.rs
	${MAKE} generate_api FILE_NAME=generated_simd.rs


comp_benches:
	RUSTFLAGS="-g" cargo bench --no-run 
	
# # valgrind outputs a callgrind.out.<pid>. We can analyze this with kcachegrind
# kcachegrind
valgrind:
	valgrind --tool=callgrind \
			--dump-instr=yes \
			--collect-jumps=yes \
			--simulate-cache=yes \
			${EXEC} --bench  ${T_ID} 
generate_api:
	cd bevy_api_gen && \
	cargo run \
	-- \
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
	--config "../api_gen_config.toml" ${FLAGS} \
	> ../bevy_script_api/src/${FILE_NAME}
	rustfmt ./bevy_script_api/src/${FILE_NAME}

generate_api_aarch64:
	cd bevy_api_gen && \
	cargo run \
	-- \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_asset.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_ecs.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_pbr.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_render.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_math.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_transform.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_sprite.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_ui.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_animation.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_core.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_core_pipeline.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_gltf.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_hierarchy.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_text.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_time.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_utils.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy_reflect.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/bevy.json" \
	--json "../target/aarch64-unknown-linux-gnu/doc/glam.json" \
	--config "../api_gen_config.toml" ${FLAGS} \
	> ../bevy_script_api/src/${FILE_NAME}
	rustfmt ./bevy_script_api/src/${FILE_NAME}

make_json_files:
	cargo +nightly-2023-07-16 rustdoc -p bevy_asset@${BEVY_VERSION}  --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_ecs@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_pbr@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_render@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_math@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_transform@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_sprite@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_ui@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_animation@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_core@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_core_pipeline@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_gltf@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_hierarchy@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_text@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_time@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_utils@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy_reflect@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p glam@${GLAM_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc -p bevy@${BEVY_VERSION} --  -Zunstable-options --output-format json 

make_json_files_aarch64:
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_asset@${BEVY_VERSION}  --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_ecs@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_pbr@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_render@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_math@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_transform@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_sprite@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_ui@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_animation@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_core@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_core_pipeline@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_gltf@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_hierarchy@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_text@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_time@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_utils@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy_reflect@${BEVY_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p glam@${GLAM_VERSION} --  -Zunstable-options --output-format json && \
	cargo +nightly-2023-07-16 rustdoc --target=aarch64-unknown-linux-gnu -p bevy@${BEVY_VERSION} --  -Zunstable-options --output-format json 