install:
	cargo install --path .

generate_bevy:
	cd bevy && cargo clean
	cd bevy && RUSTFLAGS="-Awarnings" RUST_LOG=debug,bevy_analyzer::passes::codegen=trace,rustc_plugin=debug cargo bevy-analyzer generate

collect_bevy:
	cd bevy && RUSTFLAGS="-Awarnings" RUST_LOG=debug,bevy_analyzer::passes::codegen=trace,rustc_plugin=debug cargo bevy-analyzer collect



install_and_generate_bevy: install generate_bevy collect_bevy



generate_test:
	cd test-lib && cargo clean
	cd test-lib && RUSTFLAGS="-Awarnings" RUST_LOG=debug,bevy_analyzer::passes::codegen=trace,rustc_plugin=debug cargo bevy-analyzer generate

collect_test:
	cd test-lib && RUSTFLAGS="-Awarnings" RUST_LOG=debug,bevy_analyzer::passes::codegen=trace,rustc_plugin=debug cargo bevy-analyzer collect

install_and_generate_test: install generate_test collect_test