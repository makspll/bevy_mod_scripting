#!/usr/bin/env pwsh
Remove-Variable -Name RUSTUP_TOOLCHAIN -ErrorAction SilentlyContinue
$CURRENT_DIR = Split-Path -Leaf -Path (Get-Location)

if ($CURRENT_DIR -eq "bevy_api_gen") {
    cargo +nightly-2024-12-15 clippy --all-targets --message-format=json
} else {
    cargo clippy --workspace --all-targets --message-format=json --features="lua54 rhai rune bevy/file_watcher bevy/multi_threaded"
}