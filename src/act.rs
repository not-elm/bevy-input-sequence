use bevy::prelude::{GamepadButtonType, Input, KeyCode, Res};
use bitflags::bitflags;
use std::ops::BitOr;
use std::cmp::Ordering;

bitflags! {
    /// A bit flag that stores the modifier keys--alt, control, shift, and super--in a byte.
    #[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Hash, Ord)]
    pub struct Modifiers: u8 {
        /// Represents the alt key, left or right.
        const Alt     = 0b00000001;
        /// Represents the control key, left or right.
        const Control = 0b00000010;
        /// Represents the shift key, left or right.
        const Shift   = 0b00000100;
        /// Represents the macOS command or Windows key, left or right.
        const Super   = 0b00001000;
    }
}

impl Modifiers {
    /// Check modifier keys for `any_pressed()` to populate bit flags.
    pub fn from_input(input: &Res<Input<KeyCode>>) -> Modifiers {
        let mut mods = Modifiers::empty();
        if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            mods |= Modifiers::Shift;
        }
        if input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            mods |= Modifiers::Control;
        }
        if input.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
            mods |= Modifiers::Alt;
        }
        if input.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]) {
            mods |= Modifiers::Super;
        }
        mods
    }
}

impl From<KeyCode> for Modifiers {
    #[inline(always)]
    fn from(key: KeyCode) -> Self {
        match key {
            KeyCode::ShiftLeft | KeyCode::ShiftRight => Modifiers::Shift,
            KeyCode::ControlLeft | KeyCode::ControlRight => Modifiers::Control,
            KeyCode::AltLeft | KeyCode::AltRight => Modifiers::Alt,
            KeyCode::SuperLeft | KeyCode::SuperRight => Modifiers::Super,
            _ => Modifiers::empty(),
        }
    }
}

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

// impl PartialEq for Act {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Key(a), Key(b)) => a == b,
//             (KeyChord(_, a), Key(b)) => a == b,
//             (Key(a), KeyChord(_, b)) => a == b,
//         }
//         self.isbn == other.isbn
//     }
// }
// impl Eq for Act { }

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GamepadButton(GamepadButtonType);

impl From<GamepadButtonType> for GamepadButton {
    fn from(a: GamepadButtonType) -> Self {
        Self(a)
    }
}

impl PartialOrd for GamepadButton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use bevy::reflect::Enum;
        self.0.variant_index().partial_cmp(&other.0.variant_index())
    }
}

impl Ord for GamepadButton {
    fn cmp(&self, other: &Self) -> Ordering {
        use bevy::reflect::Enum;
        self.0.variant_index().cmp(&other.0.variant_index())
    }
}

impl Act {
    #[allow(dead_code)]
    pub(crate) fn key(key: KeyCode) -> Act {
        key.into()
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
        // TODO: Consider specializing for Self::Any.
        Self::Any(vec![self, rhs])
    }
}
