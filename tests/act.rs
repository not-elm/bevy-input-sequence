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
//     assert_eq!((Modifiers::CONTROL, KeyCode::KeyB), key! { Ctrl-* });
// }

/// XXX: This doc test isn't working.
///
/// ```compile_fail
/// assert_eq!((Modifiers::CONTROL, KeyCode::F2), key!{ Ctrl-f2 });
/// ```
///
/// ```
/// let _ = key! { Ctrl-* });
/// ```
#[allow(unused_must_use)]
#[test]
fn test_key_macro() {
    assert_eq!((Modifiers::CONTROL, KeyCode::KeyB), key! { Ctrl-B });
    assert_eq!((Modifiers::CONTROL, KeyCode::Digit1), key! { Ctrl-1 });
    assert_eq!((Modifiers::CONTROL, KeyCode::Digit2), key! { Ctrl-2 });
    assert_eq!((Modifiers::CONTROL, KeyCode::F2), key! { Ctrl-F2 });
    // assert_eq!((Modifiers::CONTROL, KeyCode::F2), key!{ Ctrl-f2 });
    assert_eq!((Modifiers::CONTROL, KeyCode::Semicolon), key! { Ctrl-; });
    // assert_eq!((Modifiers::CONTROL, KeyCode::Caret), key! { Ctrl-^ });
    // assert_eq!((Modifiers::CONTROL, KeyCode::Colon), key! { Ctrl-: });
    assert_eq!((Modifiers::CONTROL, KeyCode::Equal), key! { Ctrl-= });
    assert_eq!((Modifiers::CONTROL, KeyCode::Comma), key! { Ctrl-, });
    assert_eq!((Modifiers::CONTROL, KeyCode::Period), key! { Ctrl-. });
    assert_eq!((Modifiers::CONTROL, KeyCode::Slash), key! { Ctrl-/ });
    assert_eq!((Modifiers::CONTROL, KeyCode::Enter), key! { Ctrl-Enter });
    assert_eq!((Modifiers::CONTROL, KeyCode::Space), key! { Ctrl-Space });
    assert_eq!((Modifiers::CONTROL, KeyCode::Tab), key! { Ctrl-Tab });
    assert_eq!((Modifiers::CONTROL, KeyCode::Delete), key! { Ctrl-Delete });
    assert_eq!((Modifiers::CONTROL, KeyCode::Minus), key! { Ctrl-- });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Minus),
        key! { Ctrl-Shift-- }
    );
    // assert_eq!((Modifiers::CONTROL, KeyCode::Underline), key! { Ctrl-_ });
    // No colon key.
    // assert_eq!((Modifiers::CONTROL, KeyCode::Colon), key! { Ctrl-: });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Semicolon),
        key! { Ctrl-Shift-; }
    );
    assert_eq!((Modifiers::CONTROL, KeyCode::Quote), key! { Ctrl-'\'' });

    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::KeyA),
        key! { Ctrl-Shift-A }
    );
    // assert_eq!((Modifiers::CONTROL, KeyCode::KeyA), key!{ Ctrl-A });
    assert_eq!((Modifiers::SUPER, KeyCode::KeyA), key! { Super-A });
    assert_eq!((Modifiers::CONTROL, KeyCode::KeyA), key! { Ctrl-A }); // Allow lowercase or demand lowercase?
    assert_eq!((Modifiers::empty(), KeyCode::KeyA), key! { A });
    let k = (Modifiers::empty(), KeyCode::KeyA);
    assert_eq!(k, key! { A });
    // assert_eq!(
    //     (Modifiers::CONTROL, KeyCode::Asterisk),
    //     key! { Ctrl-Asterisk }
    // );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Digit8),
        key! { Ctrl-Shift-8 }
    );

    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Digit8),
        key! { Ctrl-Shift-Digit8 }
    );
    // All bevy KeyCode names work.
    // assert_eq!((Modifiers::CONTROL, KeyCode::Asterisk), key! { Ctrl-* }); // with some short hand.

    // assert_eq!((Modifiers::CONTROL, KeyCode::Plus), key! { Ctrl-+ });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::Equal),
        key! { Ctrl-Shift-= }
    );
    // assert_eq!((Modifiers::CONTROL, KeyCode::At), key! { Ctrl-@ });
    assert_eq!(
        (Modifiers::CONTROL, KeyCode::BracketLeft),
        key! { Ctrl-'[' }
    );
    assert_eq!(
        (Modifiers::CONTROL, KeyCode::BracketRight),
        key! { Ctrl-']' }
    );
    assert_eq!(
        (Modifiers::CONTROL, KeyCode::BracketRight),
        key! { Ctrl-']' }
    );
    assert_eq!((Modifiers::CONTROL, KeyCode::Backquote), key! { Ctrl-'`' });
    assert_eq!((Modifiers::CONTROL, KeyCode::Backslash), key! { Ctrl-'\\' });
    assert_eq!((Modifiers::CONTROL, KeyCode::Escape), key! { Ctrl-Escape });
    // assert_eq!((Modifiers::CONTROL, KeyCode::Escape), key!{ Ctrl-Esc });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::KeyA),
        key! { Ctrl-Alt-A }
    );

    assert_eq!((Modifiers::empty(), KeyCode::KeyA), key! { A });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::KeyA),
        key! { Ctrl-Alt-A }
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::KeyA),
        key! { Ctrl-Alt-A }
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Semicolon),
        key! { Ctrl-Alt-Semicolon }
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Semicolon),
        key! { Ctrl-Alt-; }
    );
    assert_eq!(
        (
            Modifiers::CONTROL | Modifiers::ALT | Modifiers::SHIFT,
            KeyCode::Semicolon
        ),
        key! { Ctrl-Alt-Shift-; } // Ctrl-Alt-:
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Slash),
        key! { Ctrl-Alt-/ }
    );
}

#[allow(unused_must_use)]
#[test]
fn test_keyseq() {
    assert_eq!(
        vec![(Modifiers::CONTROL, KeyCode::KeyA)],
        keyseq! { Ctrl-A }
    );
    assert_eq!(
        vec![(Modifiers::CONTROL, KeyCode::KeyA)],
        keyseq! { Ctrl-Ctrl-A }
    );
    assert_eq!(
        vec![
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::ALT, KeyCode::KeyB)
        ],
        keyseq! { Ctrl-A Alt-B }
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
