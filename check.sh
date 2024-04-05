#!/bin/bash
unset RUSTUP_TOOLCHAIN
CURRENT_DIR=$(basename "$PWD")


if [[ "$CURRENT_DIR" == "bevy_api_gen" ]]; then
    cargo clippy --all-targets --message-format=json 
elif [[ "$CURRENT_DIR" == "macro_tests" ]]; then
    cargo clippy --all-targets --message-format=json 
else
    cargo clippy --workspace --all-targets --message-format=json --features="lua54 lua_script_api rhai rhai_script_api teal rune bevy/file_watcher bevy/multi-threaded"
fi