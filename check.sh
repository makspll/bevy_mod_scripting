#!/bin/bash
unset RUSTUP_TOOLCHAIN
CURRENT_DIR=$(basename "$PWD")

if [[ "$CURRENT_DIR" == "bevy_api_gen" ]]; then
    cargo clippy --all-targets --message-format=json 
elif [[ "$CURRENT_DIR" == "macro_tests" ]]; then
    cargo clippy --all-targets --message-format=json  
else
    cargo clippy --all-targets  --workspace --message-format=json --features="lua54 rhai teal rune bevy/file_watcher bevy/multi-threaded" 
fi