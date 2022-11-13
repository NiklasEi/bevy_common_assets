# Bevy common assets

[![crates.io](https://img.shields.io/crates/v/bevy_common_assets.svg)](https://crates.io/crates/bevy_common_assets)
[![docs](https://docs.rs/bevy_common_assets/badge.svg)](https://docs.rs/bevy_common_assets)
[![license](https://img.shields.io/crates/l/bevy_common_assets)](https://github.com/NiklasEi/bevy_common_assets#license)
[![crates.io](https://img.shields.io/crates/d/bevy_common_assets.svg)](https://crates.io/crates/bevy_common_assets)

Collection of [Bevy][bevy] plugins offering generic asset loaders for common file formats.

Supported formats:

| format    | feature   | example                               |
|:----------|:----------|:--------------------------------------|
| `json`    | `json`    | [`json.rs`](./examples/json.rs)       |
| `msgpack` | `msgpack` | [`msgpack.rs`](./examples/msgpack.rs) |
| `ron`     | `ron`     | [`ron.rs`](./examples/ron.rs)         |
| `toml`    | `toml`    | [`toml.rs`](./examples/toml.rs)       |
| `yaml`    | `yaml`    | [`yaml.rs`](./examples/yaml.rs)       |

## Usage

Enable the feature(s) for the format(s) that you want to use.

Define the types that you would like to load from files and derive `serde::Deserialize` and `bevy::reflect::TypeUuid` for them. The latter requires a unique uuid as an attribute:
```rust
#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"] // <-- keep me unique
struct Level {
    positions: Vec<[f32;3]>,
}
```

With the types ready, you can start adding asset plugins. Every plugin gets the asset type that it is supposed to load
as a generic parameter. You also need to configure custom file endings for each plugin:
```rust no_run
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_common_assets::msgpack::MsgPackAssetPlugin;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_common_assets::yaml::YamlAssetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(JsonAssetPlugin::<Level>::new(&["json.level", "custom"]))
        .add_plugin(RonAssetPlugin::<Level>::new(&["ron.level"]))
        .add_plugin(MsgPackAssetPlugin::<Level>::new(&["msgpack.level"]))
        .add_plugin(TomlAssetPlugin::<Level>::new(&["toml.level"]))
        .add_plugin(YamlAssetPlugin::<Level>::new(&["yaml.level"]))
        // ...
        .run();
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
struct Level {
    positions: Vec<[f32; 3]>,
}
```

The example above will load `Level` structs from json files ending on `.json.level` or `.custom`, from
ron files ending on `.ron.level` and so on...

See the [examples](./examples) for working Bevy apps using the different formats.

## Compatible Bevy versions

The main branch is compatible with the latest Bevy release.

Compatibility of `bevy_common_assets` versions:

| `bevy_common_assets` | `bevy` |
|:---------------------|:-------|
| `0.4`                | `0.9`  |
| `0.3`                | `0.8`  |
| `0.1` - `0.2`        | `0.7`  |
| `main`               | `0.9`  |
| `bevy_main`          | `main` |

## License

Dual-licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[bevy]: https://bevyengine.org/
