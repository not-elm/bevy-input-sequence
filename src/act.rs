use std::ops::BitOr;

use bevy::prelude::{GamepadButton, GamepadButtonType, KeyCode};

use crate::InputParams;


#[derive(Debug, Clone, PartialEq)]
pub enum Act {
    Key(KeyCode),
    PadButton(GamepadButtonType),
    Any(Vec<Act>),
}


impl Act {
    #[inline]
    pub(crate) fn other_pressed_keycode<'a>(&self, mut keys: impl Iterator<Item=&'a KeyCode>) -> bool {
        if let Self::Key(key) = self {
            keys.any(|k| k != key)
        } else {
            0 < keys.count()
        }
    }


    #[inline]
    pub(crate) fn other_pressed_pad_button<'a>(&self, buttons: impl Iterator<Item=&'a GamepadButton>) -> bool {
        let button = self.button_type();
        0 < buttons
            .filter(|input| !button.contains(&&input.button_type))
            .count()
    }


    pub(crate) fn just_inputted(&self, inputs: &InputParams) -> bool {
        match self {
            Self::Key(keycode) => inputs.key.just_pressed(*keycode),

            Self::PadButton(button) => {
                inputs.button_inputs
                    .get_just_pressed()
                    .any(|pressed| pressed.button_type == *button)
            }

            Self::Any(any) =>
                any
                    .iter()
                    .any(|input| input.just_inputted(inputs))
        }
    }


    fn button_type(&self) -> Vec<&GamepadButtonType> {
        match self {
            Act::PadButton(button) => {
                vec![button]
            }
            Act::Any(acts) => {
                acts
                    .iter()
                    .flat_map(|act| {
                        act.button_type()
                    })
                    .collect()
            }
            _ => {
                vec![]
            }
        }
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
