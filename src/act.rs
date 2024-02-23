use bevy::prelude::{GamepadButtonType, KeyCode};
use keyseq::Modifiers;
use std::cmp::Ordering;
use std::ops::BitOr;
use bevy::utils::HashSet;

/// An act represents a key press, a key chord, or a button press.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Act {
    /// A simple key press like `A` or key chord, e.g., `ctrl-B`, `ctrl-alt-C`
    KeyChord(Modifiers, KeyCode),
    /// A controller input
    PadButton(GamepadButton),
}

/// An act pattern can match any one act or some set of acts.
#[derive(Debug, Clone, Eq)]
pub enum ActPattern {
    /// Matches one and only one act
    One(Act),
    /// Matches any act from its set.
    Any(HashSet<Act>)
}

impl From<Act> for ActPattern {
    fn from(act: Act) -> Self {
        Self::One(act)
    }
}

/// Some notes on what counts as equality for patterns.
///
/// - [ActPattern::Any] == [ActPattern::Any] is only equal if they have all the
/// same members.
///
/// - [ActPattern::One] == [ActPattern::Any] is equal if one is a member of any.
///
/// - [ActPattern::One] == [ActPattern::One] is equal if they are the same.
impl PartialEq for ActPattern {
    fn eq(&self, other: &Self) -> bool {
        use ActPattern::*;
        match (self, other) {
            (Any(v), Any(w)) => v.symmetric_difference(&w).next().is_none(),
            (Any(v), One(b)) => v.contains(b),
            (One(a), Any(w)) => w.contains(a),
            (One(a), One(b)) => a == b,
        }
    }
}

/// Some notes on what counts as comparisons for patterns.
///
/// - [ActPattern::Any] < [ActPattern::Any] is true if the minimum member of the
///   first pattern is less than the minimum member of the second pattern.
///
/// - [ActPattern::One] < [ActPattern::Any] is true one is less than the minimum
/// member of the pattern.
///
/// - [ActPattern::One] < [ActPattern::One] falls back to `Act`s comparison.
impl PartialOrd for ActPattern {
    fn partial_cmp(&self, other: &ActPattern) -> Option<Ordering> {
        use ActPattern::*;
        match (self, other) {
            (Any(v), Any(w)) => {
                if v.symmetric_difference(&w).next().is_none() {
                    Some(Ordering::Equal)
                } else {
                    let a = v.iter().min().unwrap();
                    let b = w.iter().min().unwrap();
                    a.partial_cmp(&b)
                }
            },
            (Any(v), One(b)) => {
                if v.contains(b) {
                    Some(Ordering::Equal)
                } else {
                    let a = v.iter().min().unwrap();
                    a.partial_cmp(&b)
                }
            },
            (One(a), Any(w)) => {
                if w.contains(a) {
                    Some(Ordering::Equal)
                } else {
                    let b = w.iter().min().unwrap();
                    a.partial_cmp(&b)
                }
            },
            (One(a), One(b)) => a.partial_cmp(&b)
        }
    }
}

impl Ord for ActPattern {
    fn cmp(&self, other: &ActPattern) -> Ordering {
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

impl From<(Modifiers, KeyCode)> for ActPattern {
    #[inline(always)]
    fn from((mods, key): (Modifiers, KeyCode)) -> Self {
        Self::One(Act::KeyChord(mods, key))
    }
}

impl From<KeyCode> for ActPattern {
    #[inline(always)]
    fn from(value: KeyCode) -> Self {
        Self::One(Act::KeyChord(Modifiers::empty(), value))
    }
}

impl From<KeyCode> for Act {
    #[inline(always)]
    fn from(value: KeyCode) -> Self {
        Act::KeyChord(Modifiers::empty(), value)
    }
}

impl From<GamepadButtonType> for ActPattern {
    #[inline(always)]
    fn from(value: GamepadButtonType) -> Self {
        Self::One(Act::PadButton(value.into()))
    }
}

impl From<GamepadButtonType> for Act {
    #[inline(always)]
    fn from(value: GamepadButtonType) -> Self {
        Self::PadButton(value.into())
    }
}
impl BitOr for Act {
    type Output = ActPattern;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut h = HashSet::new();
        h.insert(self);
        h.insert(rhs);
        ActPattern::Any(h)
    }
}

impl BitOr for ActPattern {
    type Output = ActPattern;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        use ActPattern::*;
        match (self, rhs) {
            (Any(mut v), Any(w)) => {
                v.extend(w);
                Any(v)
            }
            (Any(mut v), One(y)) => {
                v.insert(y);
                Any(v)
            }
            (One(x), Any(mut v)) => {
                v.insert(x);
                Any(v)
            }
            (One(x), One(y)) => {
                let mut h = HashSet::new();
                h.insert(x);
                h.insert(y);
                Any(h)
            }
        }
    }
}
