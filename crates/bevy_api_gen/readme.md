# bevy_api_gen

This crate is a part of the ["bevy_mod_scripting" workspace](https://github.com/makspll/bevy_mod_scripting).

bevy_api_gen is a Cargo plugin that generates reflection-powered wrappers for Bevy types. It can list Reflect types in a workspace and perform arbitrary codegen using Tera templates.

# Installation
To install bevy_api_gen, use the following command:

```bash
cargo +nightly-2024-11-05 install bevy_api_gen
```

# Usage

## Generate

To run the main codegen process, use the following command:

```bash
cargo +nightly-2024-11-05 bevy-api-gen generate
```

This will perform all parts of the process and generate meta as well as .rs files for each crate in your workspace in your `/target/plugin-nightly-2024-11-05/bevy_api_gen` directory

## Collect

After generating all the files, you can 'collect' them in a mod.rs file like so:

```bash
cargo +nightly-2024-11-05 bevy-api-gen collect
```

## List Types

To see a list of all `Reflect` implementing types in your workspace run:

```bash
cargo +nightly-2024-11-05 bevy-api-gen list-types > all_types.txt
```

## List Templates

To see the list of all templates which you can override use:

```bash
cargo +nightly-2024-11-05 bevy-api-gen list-templates
```

## Print Template

You can also print any of the templates to stdout:

```bash
cargo +nightly-2024-11-05 bevy-api-gen print item.tera
```