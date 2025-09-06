---
applyTo: docs/, **/*.md
---

# Documentation Instructions

The `/docs` folder contain documentation for the project as well as release notes.

Newer release notes should be added to `/docs/src/ReleaseNotes/`.

When making changes to the documentation, please ensure that you follow these guidelines:
- Use clear and concise language, focusing on EXAMPLES and practical usage.
- Use these naming conventions:
  - Use lowercase letters and hyphens for file names (e.g., `my-documentation-file.md`).
  - for migration guides use the format `0.0.1-to-0.0.2.md`.
  - for release notes use the format `0.0.1.md`.


# Release Notes Instructions

Release notes should highlight the major highlights of each release, while leaving the breaking changes for migration guides.

When creating release notes, please follow these guidelines:
- Include links to relevant issues or pull requests when applicable.
- YOU MUST use the following heading structure:
  - `# Version - Short Description`
  - `## Summary`
  - `### <Theme>` for each major theme or change
  - `### Other Changes` (if applicable) for smaller changes worth mentioning
  - `## Migration Guide` (if applicable) providing a link to the detailed migration guide.


# Migration Guide Instructions

When creating migration guides, please follow these guidelines:
- Provide step-by-step instructions for updating projects to the new version.
- Each breaking change should come under a heading related to the change i.e. `### ScriptAsset removed`
- Include code snippets to illustrate changes.
- Use diff format when talking about concrete changes to specific structures or traits.
- Include links to relevant issues or pull requests when applicable.
- Generally each pull request has a migration guide section, which must be taken into account and expanded on when writing relevant migration guides.


# Examples

## Release Notes Example

```markdown
# 0.15.0 - Asset Handles and Context Policies

This release focuses on aligning `bevy_mod_scripting` with modern Bevy practices, most notably by switching to `Handle<ScriptAsset>` for script management. This change simplifies the API, removes boilerplate, and makes script handling more idiomatic.

## Summary

### Asset-First Workflow
Scripts are now treated as first-class Bevy assets. The old `ScriptId` (which was a string) has been replaced by `AssetId<ScriptAsset>`, and you'll primarily interact with scripts via `Handle<ScriptAsset>`.

```rust,ignore
// New way
let handle: Handle<ScriptAsset> = asset_server.load("my_script.lua");
commands.spawn(ScriptComponent(vec![handle]));
```

Scripts are now only evaluated when they are attached to a `ScriptComponent` or added to `StaticScripts`, which means you have more control over when and how scripts are executed.

### Other Changes
-   **`Recipients` Enum:** The `Recipients` enum for events has been redesigned to align with the new context policies, offering `AllScripts` and `AllContexts` variants, and removing some variants which don't fit the new model. If you need the old behaviour, you can simply query the ECS first before sending events.
-   **API Cleanup:** Several types and traits were removed or simplified, including `ScriptAssetSettings`, `AssetPathToScriptIdMapper`, and `ScriptMetadataStore`, as they are no longer needed with the new asset-based approach.

## Migration Guide
This release contains significant breaking changes. Please refer to the migration guide for detailed instructions on updating your project.

- [Migration Guide: 0.14 to 0.15](https://makspll.github.io/bevy_mod_scripting/Migration/0.14-to-0.15.html)

```

## Migration Guide Example

```markdown
# Migration Guide: <from> to <to>

## Changes to pre handling callbacks

This change affects the parameters for the `context_pre_handling_initializers`
```diff
- context_pre_handling_initializers: vec![|script_id, entity, context| {
+ context_pre_handling_initializers: vec![|context_key, context| {
```
and `context_initializers`:
```diff
- context_initializers: vec![|script_id, context| {
+ context_initializers: vec![|context_key, context| {
```

```
