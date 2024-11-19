#!/bin/bash
unset RUSTUP_TOOLCHAIN
CURRENT_DIR=$(basename "$PWD")


if [[ "$CURRENT_DIR" == "bevy_api_gen" ]]; then
    cargo +nightly-2024-11-05 clippy --all-targets --message-format=json 
else
    cargo clippy --workspace --all-targets --message-format=json --features="lua54 rhai teal rune bevy/file_watcher bevy/multi_threaded"
fi
