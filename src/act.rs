use bevy::prelude::{GamepadButtonType, KeyCode};
use std::cmp::Ordering;
use std::ops::BitOr;
use keyseq::Modifiers;

/// An act represents a key press, button press, key chord, or some combination.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Act {
    // A simple key input, e.g. `Act::Key(KeyCode::A)` for the `A` key.
    // Key(KeyCode),
    /// A key chord, e.g. `A`, `ctrl-B`, `ctrl-alt-C`
    KeyChord(Modifiers, KeyCode),
    /// A controller input
    PadButton(GamepadButton),
    /// Any collection of Acts
    Any(Vec<Act>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GamepadButton(GamepadButtonType);

impl From<GamepadButtonType> for GamepadButton {
    fn from(a: GamepadButtonType) -> Self {
        Self(a)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for GamepadButton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use bevy::reflect::Enum;
        Some(self.0.variant_index().cmp(&other.0.variant_index()))
    }
}

impl Ord for GamepadButton {
    fn cmp(&self, other: &Self) -> Ordering {
        use bevy::reflect::Enum;
        self.0.variant_index().cmp(&other.0.variant_index())
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone)]
    pub(crate) struct InputKind: u8 {
        const KEYBOARD = 0b0000_0001;
        const GAMEPAD  = 0b0000_0010;
    }
}

impl Act {
    #[allow(dead_code)]
    pub(crate) fn key(key: KeyCode) -> Act {
        key.into()
    }

    pub(crate) fn input_kind(&self) -> InputKind {
        match self {
            &Act::KeyChord(_, _) => InputKind::KEYBOARD,
            &Act::PadButton(_) => InputKind::GAMEPAD,
            &Act::Any(ref a) => a.iter().fold(InputKind::empty(), |a, b| a | b.input_kind()),
        }
    }
}

impl From<(Modifiers, KeyCode)> for Act {
    #[inline(always)]
    fn from((mods, key): (Modifiers, KeyCode)) -> Self {
        Self::KeyChord(mods, key)
    }
}

impl From<KeyCode> for Act {
    #[inline(always)]
    fn from(value: KeyCode) -> Self {
        Self::KeyChord(Modifiers::empty(), value)
    }
}

impl From<GamepadButtonType> for Act {
    #[inline(always)]
    fn from(value: GamepadButtonType) -> Self {
        Self::PadButton(value.into())
    }
}

impl BitOr for Act {
    type Output = Act;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Any(mut v), Self::Any(w)) => {
                v.extend(w);
                Self::Any(v)
            },
            (Self::Any(mut v), y) => {
                v.push(y);
                Self::Any(v)
            },
            (x, Self::Any(mut v)) => {
                v.push(x);
                Self::Any(v)
            },
            (x, y) => Self::Any(vec![x, y])
        }
    }
}
