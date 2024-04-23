#![doc(html_root_url = "https://docs.rs/bevy-input-sequence/0.4.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]

pub use keyseq::{
    bevy::{pkey as key, pkeyseq as keyseq},
    Modifiers,
};

pub use chord::KeyChord;
pub use plugin::InputSequencePlugin;
pub use time_limit::TimeLimit;

pub mod action;
pub mod cache;
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

/// Convenient glob import
pub mod prelude {
    pub use std::time::Duration;

    pub use crate::input_sequence::{ButtonSequence, InputSequence, KeySequence};

    pub use super::{action, InputSequencePlugin, keyseq, Modifiers, TimeLimit};
    pub use super::cond_system::IntoCondSystem;
    pub use super::chord::KeyChord;
}
