# AssetPath

Opaque Type\. 🔒

## Description

>  Represents a path to an asset in a "virtual filesystem".
> 
>  Asset paths consist of three main parts:
>  * [`AssetPath::source`]: The name of the [`AssetSource`](crate::io::AssetSource) to load the asset from.
>      This is optional. If one is not set the default source will be used (which is the `assets` folder by default).
>  * [`AssetPath::path`]: The "virtual filesystem path" pointing to an asset source file.
>  * [`AssetPath::label`]: An optional "named sub asset". When assets are loaded, they are
>      allowed to load "sub assets" of any type, which are identified by a named "label".
> 
>  Asset paths are generally constructed (and visualized) as strings:
> 
>  ```no_run
>  # use bevy_asset::{Asset, AssetServer, Handle};
>  # use bevy_reflect::TypePath;
>  #
>  # #[derive(Asset, TypePath, Default)]
>  # struct Mesh;
>  #
>  # #[derive(Asset, TypePath, Default)]
>  # struct Scene;
>  #
>  # let asset_server: AssetServer = panic!();
>  // This loads the `my_scene.scn` base asset from the default asset source.
>  let scene: Handle<Scene> = asset_server.load("my_scene.scn");
> 
>  // This loads the `PlayerMesh` labeled asset from the `my_scene.scn` base asset in the default asset source.
>  let mesh: Handle<Mesh> = asset_server.load("my_scene.scn#PlayerMesh");
> 
>  // This loads the `my_scene.scn` base asset from a custom 'remote' asset source.
>  let scene: Handle<Scene> = asset_server.load("remote://my_scene.scn");
>  ```
> 
>  [`AssetPath`] implements [`From`] for `&'static str`, `&'static Path`, and `&'a String`,
>  which allows us to optimize the static cases.
>  This means that the common case of `asset_server.load("my_scene.scn")` when it creates and
>  clones internal owned [`AssetPaths`](AssetPath).
>  This also means that you should use [`AssetPath::parse`] in cases where `&str` is the explicit type.

## Functions

