#![doc(html_root_url = "https://docs.rs/bevy-input-sequence/0.3.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]

mod cache;
mod plugin;
mod covec;
mod frame_time;
pub mod input_sequence;
mod time_limit;
mod chord;
pub mod action;
pub mod cond_system;

pub use keyseq::{
    bevy::{pkey as key, pkeyseq as keyseq},
    Modifiers,
};

/// Convenient splat import
pub mod prelude {
    pub use super::{keyseq,
                    Modifiers,
                    TimeLimit,
                    InputSequencePlugin,
                    action};
    pub use crate::input_sequence::{ButtonSequence, InputSequence, KeySequence};

    pub use std::time::Duration;
    pub use super::cond_system::IntoCondSystem;
}

pub use time_limit::TimeLimit;

pub use plugin::InputSequencePlugin;
pub use chord::KeyChord;
