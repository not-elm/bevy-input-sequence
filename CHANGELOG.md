# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2024-04-23

### Features

- Generalize to running one-shot systems instead of firing events.
- Use a plugin, can configure schedule and system set.
- Remove `GamepadEvent`; use system with input `In<Gamepad>`.
- Add IntoCondSystem that adds `only_if()` conditions to `IntoSystems`.
- Add `only_if` example.
- Add prelude module for glob imports.

### Refactor

- Hollow out lib so it's just `mod` and `pub use` statements.
- Extract sets of functionality from `lib.rs` into `chord.rs`, `cache.rs` and `plugin.rs`.
- Extract simulated tests into `tests/` directory.

## [0.3.0] - 2024-03-08

### Features

- Removed `Act::Any`, so we can no longer define an act composed of multiple buttons(or keycodes).
- The keyboard and gamepad now use different sequences.


## [0.2.0] - 2024-02-24

### Features

- Add support for bevy v0.12
- Add key chord support
- Add `key!` and `keyseq!` macro from `keyseq` crate.
- Make controllers' sequences independent of one another.
- Add `add_input_sequence_run_if()` variant.
- Add "keymod" example that uses key chords.
- Add "run_if" example

### Refactor

- Make timeout setting more ergonomic.
- Use a trie for sequence detection.

  Changes the algorithmic complexity each tick from `O(number of key_sequences)`
  to `O(length of longest key sequence)`.

- Make `new()` generic: `new<T:Into<Act>>(inputs: [T])`

### Documentation

- Describe examples in README
- Add installation, usage, and license sections to README

### Test

- The `multiple_inputs` test is disabled.

  This test includes keyboard and controller inputs. It's not clear how to
  manage these with controller sequences being independent from one another.

## [0.1.0] - 2023-08-19

- 一通りの機能の実装
