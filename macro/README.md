# bevy_keyseq_macro
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/bevy_keyseq_macro/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/bevy_keyseq_macro/actions)
  [![crates-io](https://img.shields.io/crates/v/bevy_keyseq_macro.svg)](https://crates.io/crates/bevy_keyseq_macro)
  [![api-docs](https://docs.rs/bevy_keyseq_macro/badge.svg)](https://docs.rs/bevy_keyseq_macro)

Specify key chords using a short-hand, e.g., `ctrl-A`, for the [bevy game engine](https://bevyengine.org).

# Install

``` sh
cargo add bevy_keyseq_macro
```

# Usage

## Add plugin to app
```compile
use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(bevy_keyseq_macro::TerminalShaderPlugin)
        .run()
}
```

## Add settings to camera

```compile
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerminalMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(1300., 800.)).into())
            .into(),
        material: materials.add(TerminalMaterial::green()),
        ..default()
    });
```

# Example

Run the "quad" example like so:

``` sh
cargo run --example quad
```

This will show a large quad like the one shown at the beginning of this README.

``` sh
cargo run --example cube
```

This will show a rotating cube with the shader as its surfaces.

# License

This crate is licensed under the MIT License or the Apache License 2.0 or CC0 License.

# Acknowlegments

* [Terminal Shader](https://www.shadertoy.com/view/DdSGzy) by [mrange](https://www.shadertoy.com/user/mrange) originally released under the CC0 License.

* [The sRGB Learning Curve](https://medium.com/@tomforsyth/the-srgb-learning-curve-773b7f68cf7a) by [Tom Forsyth](https://mastodon.gamedev.place/@TomF).
