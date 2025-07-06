#!/bin/bash
WORKSPACE_DIR="$PWD"

cd "$(dirname "$0")"
# if the path is in /bevy_api_gen then we run the codegen check

if [[ "$WORKSPACE_DIR" == *"/bevy_api_gen"* ]]; then
    cd "$WORKSPACE_DIR"
    cargo xtask check --ide-mode --kind codegen
else
    cd "$WORKSPACE_DIR"
    cargo xtask check --ide-mode --kind main
fi
