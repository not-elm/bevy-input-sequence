use bevy::prelude::{GamepadButton, GamepadButtonType, Input, KeyCode, Res};
use bitflags::bitflags;
use std::ops::BitOr;

use crate::InputParams;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Hash, Ord)]
    pub struct Modifiers: u8 {
        const Alt     = 0b00000001;
        const Control = 0b00000010;
        const Shift   = 0b00000100;
        const Super   = 0b00001000; // Windows or Command
    }
}

impl Modifiers {
    fn from_input(input: &Res<Input<KeyCode>>) -> Modifiers {
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

#[derive(Debug, Clone, PartialEq)]
pub enum Act {
    Key(KeyCode),
    KeyChord(Modifiers, KeyCode),
    PadButton(GamepadButtonType),
    Any(Vec<Act>),
}

fn is_modifier(key: KeyCode) -> bool {
    let mods = Modifiers::from(key);
    !mods.is_empty()
}

impl Act {
    #[inline]
    pub(crate) fn other_pressed_keycode<'a>(
        &self,
        mut keys: impl Iterator<Item = &'a KeyCode>,
    ) -> bool {
        if let Self::Key(key) = self {
            // keys.any(|k| k != key)
            // Make it insensitive to modifier key presses.
            keys.any(|k| k != key && !is_modifier(*k))
        } else {
            0 < keys.count()
        }
    }

    #[inline]
    pub(crate) fn other_pressed_pad_button<'a>(
        &self,
        buttons: impl Iterator<Item = &'a GamepadButton>,
    ) -> bool {
        let button = self.button_type();
        0 < buttons
            .filter(|input| !button.contains(&&input.button_type))
            .count()
    }

    pub(crate) fn just_inputted(&self, inputs: &InputParams) -> bool {
        match self {
            Self::Key(keycode) => inputs.key.just_pressed(*keycode),
            Self::KeyChord(modifiers, keycode) => {
                let current_modifiers = Modifiers::from_input(&inputs.key);
                inputs.key.just_pressed(*keycode) && &current_modifiers == modifiers
            }

            Self::PadButton(button) => inputs
                .button_inputs
                .get_just_pressed()
                .any(|pressed| pressed.button_type == *button),

            Self::Any(any) => any.iter().any(|input| input.just_inputted(inputs)),
        }
    }

    fn button_type(&self) -> Vec<&GamepadButtonType> {
        match self {
            Act::PadButton(button) => {
                vec![button]
            }
            Act::Any(acts) => acts.iter().flat_map(|act| act.button_type()).collect(),
            _ => {
                vec![]
            }
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
        Self::Key(value)
    }
}

impl From<GamepadButtonType> for Act {
    #[inline(always)]
    fn from(value: GamepadButtonType) -> Self {
        Self::PadButton(value)
    }
}

impl BitOr for Act {
    type Output = Act;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Any(vec![self, rhs])
    }
}

