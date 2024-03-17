install:
	cargo install --path .

run_bevy:
	cd bevy && cargo clean
	cd bevy && RUSTFLAGS="-Awarnings" RUST_LOG=debug,rustc_plugin=error cargo bevy-analyzer --all-features -- -vv


install_and_run_bevy: install run_bevy