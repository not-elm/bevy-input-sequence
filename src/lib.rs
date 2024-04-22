#![doc(html_root_url = "https://docs.rs/bevy-input-sequence/0.3.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]

pub mod action;
mod cache;
mod chord;
pub mod cond_system;
mod covec;
mod frame_time;
pub mod input_sequence;
mod plugin;
mod time_limit;

pub use keyseq::{
    bevy::{pkey as key, pkeyseq as keyseq},
    Modifiers,
};

/// Convenient splat import
pub mod prelude {
    pub use super::{action, keyseq, InputSequencePlugin, Modifiers, TimeLimit};
    pub use crate::input_sequence::{ButtonSequence, InputSequence, KeySequence};

    pub use super::cond_system::IntoCondSystem;
    pub use std::time::Duration;
}

pub use time_limit::TimeLimit;

pub use chord::KeyChord;
pub use plugin::InputSequencePlugin;
