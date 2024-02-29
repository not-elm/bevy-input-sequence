use bevy::prelude::*;
use bevy_input_sequence::*;

#[allow(unused_must_use)]
#[test]
fn test_key_eq() {
    let a: KeyChord = KeyCode::KeyA.into();
    let b: KeyChord = KeyCode::KeyA.into();
    assert_eq!(a, b);
    assert!(a == b);
}

#[allow(unused_must_use)]
#[test]
fn test_keyseq_macro() {
    assert_eq!(vec![(Modifiers::empty(), KeyCode::KeyA)], keyseq! { A });
    assert_eq!(
        vec![
            (Modifiers::empty(), KeyCode::KeyA),
            (Modifiers::empty(), KeyCode::KeyB),
        ],
        keyseq! { A B }
    );
}

#[test]
fn eq_if_contains_key_in_lhs() {
    let lhs = KeyChord(Modifiers::CONTROL, KeyCode::KeyA);
    let rhs = KeyChord(Modifiers::CONTROL, KeyCode::KeyA);
    assert!(lhs == rhs);
    assert!(rhs == lhs);
}

// #[test]
// fn test_shifted_key_macro() {
//     assert_eq!((Modifiers::CONTROL, KeyCode::KeyB), key! { ctrl-* });
// }

/// XXX: This doc test isn't working.
///
/// ```compile_fail
/// assert_eq!((Modifiers::CONTROL, KeyCode::F2), key!{ ctrl-f2 });
/// ```
///
/// ```
/// let _ = key! { ctrl-* });
/// ```
#[allow(unused_must_use)]
#[test]
fn test_key_macro() {
    assert_eq!((Modifiers::CONTROL, KeyCode::KeyB), key! { ctrl-B });
    assert_eq!((Modifiers::CONTROL, KeyCode::Digit1), key! { ctrl-1 });
    assert_eq!((Modifiers::CONTROL, KeyCode::Digit2), key! { ctrl-2 });
    assert_eq!((Modifiers::CONTROL, KeyCode::F2), key! { ctrl-F2 });
    // assert_eq!((Modifiers::CONTROL, KeyCode::F2), key!{ ctrl-f2 });
    assert_eq!((Modifiers::CONTROL, KeyCode::Semicolon), key! { ctrl-; });
    // assert_eq!((Modifiers::CONTROL, KeyCode::Caret), key! { ctrl-^ });
    // assert_eq!((Modifiers::CONTROL, KeyCode::Colon), key! { ctrl-: });
    assert_eq!((Modifiers::CONTROL, KeyCode::Equal), key! { ctrl-= });
    assert_eq!((Modifiers::CONTROL, KeyCode::Comma), key! { ctrl-, });
    assert_eq!((Modifiers::CONTROL, KeyCode::Period), key! { ctrl-. });
    assert_eq!((Modifiers::CONTROL, KeyCode::Slash), key! { ctrl-/ });
    assert_eq!((Modifiers::CONTROL, KeyCode::Enter), key! { ctrl-Enter });
    assert_eq!((Modifiers::CONTROL, KeyCode::Space), key! { ctrl-Space });
    assert_eq!((Modifiers::CONTROL, KeyCode::Tab), key! { ctrl-Tab });
    assert_eq!((Modifiers::CONTROL, KeyCode::Delete), key! { ctrl-Delete });
    assert_eq!((Modifiers::CONTROL, KeyCode::Minus), key! { ctrl-- });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Minus),
        key! { ctrl-shift-- }
    );
    // assert_eq!((Modifiers::CONTROL, KeyCode::Underline), key! { ctrl-_ });
    // No colon key.
    // assert_eq!((Modifiers::CONTROL, KeyCode::Colon), key! { ctrl-: });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Semicolon),
        key! { ctrl-shift-; }
    );
    assert_eq!((Modifiers::CONTROL, KeyCode::Quote), key! { ctrl-'\'' });

    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::KeyA),
        key! { ctrl-shift-A }
    );
    // assert_eq!((Modifiers::CONTROL, KeyCode::KeyA), key!{ ctrl-A });
    assert_eq!((Modifiers::SUPER, KeyCode::KeyA), key! { super-A });
    assert_eq!((Modifiers::CONTROL, KeyCode::KeyA), key! { ctrl-A }); // Allow lowercase or demand lowercase?
    assert_eq!((Modifiers::empty(), KeyCode::KeyA), key! { A });
    let k = (Modifiers::empty(), KeyCode::KeyA);
    assert_eq!(k, key! { A });
    // assert_eq!(
    //     (Modifiers::CONTROL, KeyCode::Asterisk),
    //     key! { ctrl-Asterisk }
    // );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Digit8),
        key! { ctrl-shift-8 }
    );

    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Digit8),
        key! { ctrl-shift-Digit8 }
    );
    // All bevy KeyCode names work.
    // assert_eq!((Modifiers::CONTROL, KeyCode::Asterisk), key! { ctrl-* }); // with some short hand.

    // assert_eq!((Modifiers::CONTROL, KeyCode::Plus), key! { ctrl-+ });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Equal),
        key! { ctrl-shift-= }
    );
    // assert_eq!((Modifiers::CONTROL, KeyCode::At), key! { ctrl-@ });
    assert_eq!(
        (Modifiers::CONTROL, KeyCode::BracketLeft),
        key! { ctrl-'[' }
    );
    assert_eq!(
        (Modifiers::CONTROL, KeyCode::BracketRight),
        key! { ctrl-']' }
    );
    assert_eq!(
        (Modifiers::CONTROL, KeyCode::BracketRight),
        key! { ctrl-']' }
    );
    assert_eq!((Modifiers::CONTROL, KeyCode::Backquote), key! { ctrl-'`' });
    assert_eq!((Modifiers::CONTROL, KeyCode::Backslash), key! { ctrl-'\\' });
    assert_eq!((Modifiers::CONTROL, KeyCode::Escape), key! { ctrl-Escape });
    // assert_eq!((Modifiers::CONTROL, KeyCode::Escape), key!{ ctrl-Esc });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::KeyA),
        key! { ctrl-alt-A }
    );

    assert_eq!((Modifiers::empty(), KeyCode::KeyA), key! { A });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::KeyA),
        key! { ctrl-alt-A }
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::KeyA),
        key! { ctrl-alt-A }
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Semicolon),
        key! { ctrl-alt-Semicolon }
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Semicolon),
        key! { ctrl-alt-; }
    );
    assert_eq!(
        (
            Modifiers::CONTROL | Modifiers::ALT | Modifiers::SHIFT,
            KeyCode::Semicolon
        ),
        key! { ctrl-alt-shift-; } // ctrl-alt-:
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Slash),
        key! { ctrl-alt-/ }
    );
}

#[allow(unused_must_use)]
#[test]
fn test_keyseq() {
    assert_eq!(
        vec![(Modifiers::CONTROL, KeyCode::KeyA)],
        keyseq! { ctrl-A }
    );
    assert_eq!(
        vec![(Modifiers::CONTROL, KeyCode::KeyA)],
        keyseq! { ctrl-ctrl-A }
    );
    assert_eq!(
        vec![
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::ALT, KeyCode::KeyB)
        ],
        keyseq! { ctrl-A alt-B }
    );

    assert_eq!(
        vec![
            (Modifiers::empty(), KeyCode::KeyA),
            (Modifiers::empty(), KeyCode::KeyB)
        ],
        keyseq! { A B }
    );
}

#[test]
fn test_key_eq_not() {
    let a: KeyChord = KeyCode::KeyA.into();
    let b: KeyChord = KeyCode::KeyB.into();
    assert!(a != b);
}

#[test]
fn test_key_eq_vec() {
    let a: Vec<KeyChord> = vec![KeyCode::KeyA.into()];
    let b: Vec<KeyChord> = vec![KeyCode::KeyB.into()];
    let c: Vec<KeyChord> = vec![KeyCode::KeyA.into()];
    let e: Vec<KeyChord> = vec![];
    assert!(a != b);
    assert!(a == c);
    assert_eq!(a, c);
    assert!(e != a);
    assert!(e != b);
    assert!(e != c);
}
