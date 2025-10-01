# Bevy common assets

[![crates.io](https://img.shields.io/crates/v/bevy_common_assets.svg)](https://crates.io/crates/bevy_common_assets)
[![docs](https://docs.rs/bevy_common_assets/badge.svg)](https://docs.rs/bevy_common_assets)
[![license](https://img.shields.io/crates/l/bevy_common_assets)](https://github.com/NiklasEi/bevy_common_assets#license)
[![crates.io](https://img.shields.io/crates/d/bevy_common_assets.svg)](https://crates.io/crates/bevy_common_assets)

Collection of [Bevy][bevy] plugins offering generic asset loaders and writers for common file formats.

Supported formats:

| format     | feature    | example                                 |
| :--------- | :--------- | :-------------------------------------- |
| `json`     | `json`     | [`json.rs`](./examples/json.rs)         |
| `msgpack`  | `msgpack`  | [`msgpack.rs`](./examples/msgpack.rs)   |
| `postcard` | `postcard` | [`postcard.rs`](./examples/postcard.rs) |
| `ron`      | `ron`      | [`ron.rs`](./examples/ron.rs)           |
| `toml`     | `toml`     | [`toml.rs`](./examples/toml.rs)         |
| `xml`      | `xml`      | [`xml.rs`](./examples/xml.rs)           |
| `yaml`     | `yaml`     | [`yaml.rs`](./examples/yaml.rs)         |
| `csv`      | `csv`      | [`csv.rs`](./examples/csv.rs)           |
| `cbor`     | `cbor`     | [`cbor.rs`](./examples/cbor.rs)         |

## Usage

Enable the feature(s) for the format(s) that you want to use.

Define the types that you would like to load from files and derive `serde::Deserialize`, `bevy::reflect::TypePath`, and `bevy::asset::Asset` for them.

```rust
#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
struct Level {
    positions: Vec<[f32;3]>,
}
```

With the types ready, you can start adding asset plugins. Every plugin gets the asset type that it is supposed to load
as a generic parameter. You can also configure custom file endings for each plugin:

```rust no_run
use bevy::prelude::*;
use bevy_common_assets::cbor::CborAssetPlugin;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_common_assets::msgpack::MsgPackAssetPlugin;
use bevy_common_assets::postcard::PostcardAssetPlugin;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_common_assets::xml::XmlAssetPlugin;
use bevy_common_assets::yaml::YamlAssetPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            JsonAssetPlugin::<Level>::new(&["level.json", "custom.json"]),
            RonAssetPlugin::<Level>::new(&["level.ron"]),
            MsgPackAssetPlugin::<Level>::new(&["level.msgpack"]),
            PostcardAssetPlugin::<Level>::new(&["level.postcard"]),
            TomlAssetPlugin::<Level>::new(&["level.toml"]),
            XmlAssetPlugin::<Level>::new(&["level.xml"]),
            YamlAssetPlugin::<Level>::new(&["level.yaml"]),
            CborAssetPlugin::<Level>::new(&["level.cbor"])
        ))
        // ...
        .run();
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
struct Level {
    positions: Vec<[f32; 3]>,
}
```

The example above will load `Level` structs from json files ending on `.level.json` or `.custom.json`, from
ron files ending on `.level.ron` and so on...

See the [examples](./examples) for working Bevy apps using the different formats.

## Asset savers / using the loaders in .meta files

The more involved [example `asset_savers`](./examples/asset_savers) demonstrates how you can convert a json
asset into a processed postcard asset using the `JsonAssetLoader` and `PostcardAssetSaver`.

## Compatible Bevy versions

The main branch is compatible with the latest Bevy release.

Compatibility of `bevy_common_assets` versions:

| `bevy_common_assets` | `bevy` |
| :------------------- | :----- |
| `0.14`               | `0.17` |
| `0.13`               | `0.16` |
| `0.12`               | `0.15` |
| `0.11`               | `0.14` |
| `0.10`               | `0.13` |
| `0.8` - `0.9`        | `0.12` |
| `0.7`                | `0.11` |
| `0.5` - `0.6`        | `0.10` |
| `0.4`                | `0.9`  |
| `0.3`                | `0.8`  |
| `0.1` - `0.2`        | `0.7`  |

## License

Dual-licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[bevy]: https://bevyengine.org/
