use std::ops::BitOr;

use bevy::prelude::{GamepadAxisType, GamepadButtonType, KeyCode};

use crate::InputParams;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Entry {
    Key(KeyCode),
    PadButton(GamepadButtonType),
    PadButtonAxis(GamepadButtonType),
    PadAxis(GamepadAxisType),
    Any(Vec<Entry>),
}


impl Entry {
    pub(crate) fn just_inputted(&self, inputs: &InputParams) -> bool {
        match self {
            Self::Key(keycode) => inputs.key.just_pressed(*keycode),

            Self::PadButton(button) =>
                inputs.button_inputs
                    .get_just_pressed()
                    .any(|pressed| pressed.button_type == *button),

            Self::PadButtonAxis(axis) =>
                inputs.button_axes
                    .devices()
                    .filter(|pad| pad.button_type == *axis)
                    .filter_map(|pad| inputs.button_axes.get(*pad))
                    .any(|axis| 0.01 < axis.abs()),

            Self::PadAxis(axis) =>
                inputs.axes
                    .devices()
                    .filter(|pad| pad.axis_type == *axis)
                    .filter_map(|pad| inputs.axes.get(*pad))
                    .any(|axis| 0.01 < axis.abs()),

            Self::Any(any) =>
                any
                    .iter()
                    .any(|input| input.just_inputted(inputs))
        }
    }
}


impl From<KeyCode> for Entry {
    #[inline(always)]
    fn from(value: KeyCode) -> Self {
        Self::Key(value)
    }
}


impl From<GamepadButtonType> for Entry {
    #[inline(always)]
    fn from(value: GamepadButtonType) -> Self {
        Self::PadButton(value)
    }
}


impl BitOr for Entry {
    type Output = Entry;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Any(vec![self, rhs])
    }
}
