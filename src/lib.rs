#![doc(html_root_url = "https://docs.rs/bevy-input-sequence/0.7.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]

pub mod action;
pub mod cache;
mod chord;
pub mod cond_system;
mod frame_time;
pub mod input_sequence;
mod plugin;
mod time_limit;

pub use chord::{KeyChord, KeyChordQueue};
pub use plugin::InputSequencePlugin;
pub use time_limit::TimeLimit;

pub use keyseq::{
    bevy::{pkey as key, pkeyseq as keyseq},
    Modifiers,
};

/// Convenient glob import
pub mod prelude {
    pub use super::cond_system::IntoCondSystem;
    pub use super::input_sequence::{ButtonSequence, InputSequence, KeySequence};
    pub use super::{action, keyseq, InputSequencePlugin, Modifiers, TimeLimit};
    pub use super::{KeyChord, KeyChordQueue};
    pub use std::time::Duration;
}
