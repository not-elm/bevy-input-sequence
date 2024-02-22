use bevy::prelude::{GamepadButtonType, KeyCode};
use keyseq::Modifiers;
use std::cmp::Ordering;
use std::ops::BitOr;
use bevy::utils::HashSet;
// use std::collections::HashSet;

/// An act represents a key press, button press, key chord, or some combination.
#[derive(Debug, Clone, Eq, Hash)]
pub enum Act {
    // A simple key input, e.g. `Act::Key(KeyCode::A)` for the `A` key.
    // Key(KeyCode),
    /// A key chord, e.g. `A`, `ctrl-B`, `ctrl-alt-C`
    KeyChord(Modifiers, KeyCode),
    /// A controller input
    PadButton(GamepadButton),
    /// Any collection of Acts
    Any(Vec<Act>),
    // It'd be nice to use a hashset but these are small numbers.
    // Any(HashSet<Act>),
}

// #[derive(Debug, Clone, Eq)]
// pub enum ActPattern {
//     One(Act),
//     Any(HashSet<Act>)
// }

/// Note: [Act::Any] == [Act::Any] is not supported and will panic. It's
/// expected that there will be one concrete sequence of acts being compared to
/// an "act pattern" which may have [Act::Any].
impl PartialEq for Act {
    fn eq(&self, other: &Self) -> bool {
        use Act::*;
        match (self, other) {
            (Act::Any(v), Act::Any(w)) => {
                let a = HashSet::from_iter(v);
                let b = HashSet::from_iter(w);
                a.intersection(&b).next().is_some()
            },
            (Act::Any(v), b) => v.contains(b),
            (a, Act::Any(w)) => w.contains(a),
            (KeyChord(am, ak), KeyChord(bm, bk)) => am == bm && ak == bk,
            (PadButton(a), PadButton(b)) => a == b,
            (_, _) => false
        }
    }
}

/// Note: [Act::Any] < [Act::Any] is not supported and will panic. It's expected
/// that there will be one concrete sequence of acts being compared to an "act
/// pattern" which may have [Act::Any].
impl PartialOrd for Act {
    fn partial_cmp(&self, other: &Act) -> Option<Ordering> {
        use Act::*;
        match (self, other) {

            (Act::Any(v), Act::Any(w)) => {
                let a = HashSet::from_iter(v);
                let b = HashSet::from_iter(w);
                if a.intersection(&b).next().is_some() {
                    Some(Ordering::Equal)
                } else {
                    v.first().and_then(|x| w.first().and_then(|y| x.partial_cmp(y)))
                }
            },
            (Act::Any(v), b) => {
                if v.contains(b) {
                    Some(Ordering::Equal)
                } else {
                    v.first().and_then(|x| x.partial_cmp(b))
                }
            },
            (a, Act::Any(w)) => {
                if w.contains(a) {
                    Some(Ordering::Equal)
                } else {
                    w.first().and_then(|x| x.partial_cmp(a))
                }
            },
            // compare on key first, modifiers second.
            (KeyChord(am, ak), KeyChord(bm, bk)) =>
                ak.partial_cmp(bk)
                  .and_then(|x|
                            if x == Ordering::Equal {
                                am.partial_cmp(bm)
                            } else {
                                Some(x)
                            }),
            (PadButton(a), PadButton(b)) => a.partial_cmp(b),
            (KeyChord(_, _), PadButton(_)) => Some(Ordering::Less),
            (PadButton(_), KeyChord(_, _)) => Some(Ordering::Greater),
        }
    }
}

impl Ord for Act {
    fn cmp(&self, other: &Act) -> Ordering {
        self.partial_cmp(other).expect("Should have an ordering")
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
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
    pub(crate) fn input_kind(&self) -> InputKind {
        match *self {
            Act::KeyChord(_, _) => InputKind::KEYBOARD,
            Act::PadButton(_) => InputKind::GAMEPAD,
            Act::Any(ref a) => a.iter().fold(InputKind::empty(), |a, b| a | b.input_kind()),
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
            }
            (Self::Any(mut v), y) => {
                v.push(y);
                Self::Any(v)
            }
            (x, Self::Any(mut v)) => {
                v.push(x);
                Self::Any(v)
            }
            (x, y) => Self::Any(vec![x, y]),
        }
    }
}
