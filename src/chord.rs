use bevy::{
    input::keyboard::KeyCode,
    prelude::{Deref, DerefMut, Resource, ReflectResource},
    reflect::{Enum, Reflect},
};

use std::{collections::VecDeque, fmt};

use keyseq::Modifiers;

/// Represents a key chord, i.e., a set of modifiers and a key code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub struct KeyChord(pub Modifiers, pub KeyCode);

impl fmt::Display for KeyChord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use KeyCode::*;
        self.0.fmt(f)?;
        if !self.0.is_empty() {
            f.write_str("-")?;
        }
        let key_repr = match self.1 {
            Semicolon => ";",
            Period => ".",
            Equal => "=",
            Slash => "/",
            Minus => "-",
            BracketLeft => "[",
            BracketRight => "]",
            Quote => "'",
            Backquote => "`",
            key_code => {
                let mut key = key_code.variant_name();
                key = key.strip_prefix("Key").unwrap_or(key);
                key = key.strip_prefix("Digit").unwrap_or(key);
                return f.write_str(key);
            }
        };
        f.write_str(key_repr)
    }
}

impl From<(Modifiers, KeyCode)> for KeyChord {
    #[inline(always)]
    fn from((mods, key): (Modifiers, KeyCode)) -> Self {
        KeyChord(mods, key)
    }
}

impl From<KeyCode> for KeyChord {
    #[inline(always)]
    fn from(key: KeyCode) -> Self {
        KeyChord(Modifiers::empty(), key)
    }
}

pub(crate) fn is_modifier(key: KeyCode) -> bool {
    !Modifiers::from(key).is_empty()
}

/// Manually add key chords to be processed as through they were pressed by the
/// user.
///
/// Normally this does not need to be manipulated. It is a kind of escape hatch.
#[derive(Resource, Debug, Deref, DerefMut, Default, Reflect)]
#[reflect(Resource)]
pub struct KeyChordQueue(pub VecDeque<KeyChord>);
