$WORKSPACE_DIR = Get-Location

Set-Location $PSScriptRoot

if ($WORKSPACE_DIR -like "*\codegen*") {
    cargo xtask check --ide-mode --kind codegen
}
elseif ($WORKSPACE_DIR -like "*\xtask*") {
    Set-Location $WORKSPACE_DIR
    cargo check --quiet --workspace --message-format=json --all-targets
}
else {
    Set-Location $WORKSPACE_DIR
    cargo xtask check --ide-mode --kind main
}