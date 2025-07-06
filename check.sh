#!/bin/bash
WORKSPACE_DIR="$PWD"

cd "$(dirname "$0")"
# if the path is in /bevy_api_gen then we run the codegen check

if [[ "$WORKSPACE_DIR" == *"/bevy_api_gen"* ]]; then
    # save output to file as well as stdout and stderr
    cargo xtask check --ide-mode --kind codegen
elif [[ "$WORKSPACE_DIR" == *"/xtask"* ]]; then
    cd "$WORKSPACE_DIR"
    cargo check --quiet --workspace --message-format=json --all-targets 
else 
    cd "$WORKSPACE_DIR"
    cargo xtask check --ide-mode --kind main
fi