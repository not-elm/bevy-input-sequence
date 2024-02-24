# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

## [0.2.0] - 2024-02-20

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
