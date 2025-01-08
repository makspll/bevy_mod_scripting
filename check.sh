#!/bin/bash
cd "$(dirname "$0")"
cargo xtask check --ide-mode
