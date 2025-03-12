# Script ID mapping

Every script is currently identified by a unique ID.

ID's are derived from the script asset path for scripts loaded via the asset system.

By default this is an identity mapping, but you can override this by modifying the `AssetPathToScriptIdMapper` inside the `ScriptAssetSettings` resource before loading the script.

<div class="warning">
Be wary of path separators, by default script ID's are derived from asset paths, which are platform dependent. Make sure to use `std::path::PathBuf` if you are targetting multiple platforms.
</div>