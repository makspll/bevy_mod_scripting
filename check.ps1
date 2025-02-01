#!/usr/bin/env pwsh
Set-Location -Path (Split-Path -Parent $MyInvocation.MyCommand.Definition)
cargo xtask check --ide-mode