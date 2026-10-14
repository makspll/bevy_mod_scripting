#!/bin/bash
WORKSPACE_DIR="$PWD"

cd "$(dirname "$0")"

if [[ "$WORKSPACE_DIR" == *"/xtask"* ]]; then
    cd "$WORKSPACE_DIR"
    cargo check --quiet --workspace --message-format=json --all-targets 
else 
    cd "$WORKSPACE_DIR"
    cargo xtask check --ide-mode --kind main
fi