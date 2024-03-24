#!/bin/bash
unset RUSTUP_TOOLCHAIN
CURRENT_DIR=$(basename "$PWD")

if [[ "$CURRENT_DIR" == "bevy_api_gen" ]]; then
    cargo clippy --message-format=json 
else
    cargo clippy --message-format=json --features="lua54, lua_script_api, rhai, rhai_script_api, teal"
fi