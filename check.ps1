#!/usr/bin/env pwsh
$WORKSPACE_DIR = Get-Location

Set-Location (Split-Path $MyInvocation.MyCommand.Path)

# dump environment to current ./xtask.log file 

# if the path is in /bevy_api_gen then we run the codegen check

if ($WORKSPACE_DIR -like "*\bevy_api_gen*") {
    cargo xtask check --ide-mode --kind codegen
} else {
    cargo xtask check --ide-mode --kind main
}