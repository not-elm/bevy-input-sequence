use bevy::prelude::*;
use bevy_input_sequence::prelude::Act::*;
use bevy_input_sequence::prelude::*;

#[allow(unused_must_use)]
#[test]
fn test_key_eq() {
    let a: Act = KeyCode::A.into();
    let b: Act = KeyCode::A.into();
    assert_eq!(a, b);
    assert!(a == b);
}

#[allow(unused_must_use)]
#[test]
fn test_keyseq_macro() {
    assert_eq!(
        vec![KeyChord(Modifiers::empty(), KeyCode::A)],
        keyseq! { A }
    );
    assert_eq!(
        vec![
            KeyChord(Modifiers::empty(), KeyCode::A),
            KeyChord(Modifiers::empty(), KeyCode::B),
        ],
        keyseq! { A B }
    );
}

/// XXX: This doc test isn't working.
///
/// ```compile_fail
/// assert_eq!(KeyChord(Modifiers::Control, KeyCode::F2), key!{ ctrl-f2 });
/// ```
#[allow(unused_must_use)]
#[test]
fn test_key_macro() {
    assert_eq!(KeyChord(Modifiers::Control, KeyCode::B), key! { ctrl-B });
    assert_eq!(KeyChord(Modifiers::Control, KeyCode::Key1), key! { ctrl-1 });
    assert_eq!(KeyChord(Modifiers::Control, KeyCode::Key2), key! { ctrl-2 });
    assert_eq!(KeyChord(Modifiers::Control, KeyCode::F2), key! { ctrl-F2 });
    // assert_eq!(KeyChord(Modifiers::Control, KeyCode::F2), key!{ ctrl-f2 });
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Semicolon),
        key! { ctrl-; }
    );
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Caret),
        key! { ctrl-^ }
    );
    // assert_eq!(KeyChord(Modifiers::Control, KeyCode::Colon), key! { ctrl-: });
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Equals),
        key! { ctrl-= }
    );
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Comma),
        key! { ctrl-, }
    );
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Period),
        key! { ctrl-. }
    );
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Slash),
        key! { ctrl-/ }
    );
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Minus),
        key! { ctrl-- }
    );
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Underline),
        key! { ctrl-_ }
    );
    // assert_eq!(KeyChord(Modifiers::Control, KeyCode::Colon), key! { ctrl-: });
    assert_eq!(
        KeyChord(Modifiers::Control | Modifiers::Shift, KeyCode::Semicolon),
        key! { ctrl-: }
    );
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Apostrophe),
        key! { ctrl-'\'' }
    );

    assert_eq!(
        KeyChord(Modifiers::Control | Modifiers::Shift, KeyCode::A),
        key! { ctrl-shift-A }
    );
    // assert_eq!(KeyChord(Modifiers::Control, KeyCode::A), key!{ ctrl-A });
    assert_eq!(KeyChord(Modifiers::Super, KeyCode::A), key! { super-A });
    assert_eq!(KeyChord(Modifiers::Control, KeyCode::A), key! { ctrl-A }); // Allow lowercase or demand lowercase?
    assert_eq!(KeyChord(Modifiers::empty(), KeyCode::A), key! { A });
    let k: Act = (Modifiers::empty(), KeyCode::A).into();
    assert_eq!(k, key! { A });
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Asterisk),
        key! { ctrl-Asterisk }
    ); // All bevy KeyCode names work.
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Asterisk),
        key! { ctrl-* }
    ); // with some short hand.

    assert_eq!(KeyChord(Modifiers::Control, KeyCode::Plus), key! { ctrl-+ });
    assert_eq!(KeyChord(Modifiers::Control, KeyCode::At), key! { ctrl-@ });
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Grave),
        key! { ctrl-'`' }
    );
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Backslash),
        key! { ctrl-'\\' }
    );
    assert_eq!(
        KeyChord(Modifiers::Control, KeyCode::Escape),
        key! { ctrl-Escape }
    );
    // assert_eq!(KeyChord(Modifiers::Control, KeyCode::Escape), key!{ ctrl-Esc });
    assert_eq!(
        KeyChord(Modifiers::Control | Modifiers::Alt, KeyCode::A),
        key! { ctrl-alt-A }
    );

    assert_eq!(KeyChord(Modifiers::empty(), KeyCode::A), key! { A });
    assert_eq!(
        KeyChord(Modifiers::Control | Modifiers::Alt, KeyCode::A),
        key! { ctrl-alt-A }
    );
    assert_eq!(
        KeyChord(Modifiers::Control | Modifiers::Alt, KeyCode::A),
        key! { ctrl-alt-A }
    );
    assert_eq!(
        KeyChord(Modifiers::Control | Modifiers::Alt, KeyCode::Semicolon),
        key! { ctrl-alt-Semicolon }
    );
    assert_eq!(
        KeyChord(Modifiers::Control | Modifiers::Alt, KeyCode::Semicolon),
        key! { ctrl-alt-; }
    );
    assert_eq!(
        KeyChord(
            Modifiers::Control | Modifiers::Alt | Modifiers::Shift,
            KeyCode::Semicolon
        ),
        key! { ctrl-alt-: }
    );
    assert_eq!(
        KeyChord(Modifiers::Control | Modifiers::Alt, KeyCode::Slash),
        key! { ctrl-alt-/ }
    );
}

#[allow(unused_must_use)]
#[test]
fn test_keyseq() {
    assert_eq!(
        vec![KeyChord(Modifiers::Control, KeyCode::A)],
        keyseq! { ctrl-A }
    );
    assert_eq!(
        vec![KeyChord(Modifiers::Control, KeyCode::A)],
        keyseq! { ctrl-ctrl-A }
    );
    assert_eq!(
        vec![
            KeyChord(Modifiers::Control, KeyCode::A),
            KeyChord(Modifiers::Alt, KeyCode::B)
        ],
        keyseq! { ctrl-A alt-B}
    );

    assert_eq!(
        vec![
            KeyChord(Modifiers::empty(), KeyCode::A),
            KeyChord(Modifiers::empty(), KeyCode::B)
        ],
        keyseq! { A B}
    );
}

#[test]
fn test_key_eq_not() {
    let a: Act = KeyCode::A.into();
    let b: Act = KeyCode::B.into();
    assert!(a != b);
}

#[test]
fn test_key_eq_vec() {
    let a: Vec<Act> = vec![KeyCode::A.into()];
    let b: Vec<Act> = vec![KeyCode::B.into()];
    let c: Vec<Act> = vec![KeyCode::A.into()];
    let e: Vec<Act> = vec![];
    assert!(a != b);
    assert!(a == c);
    assert_eq!(a, c);
    assert!(e != a);
    assert!(e != b);
    assert!(e != c);
}
