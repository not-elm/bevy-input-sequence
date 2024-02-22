use bevy::prelude::*;
use bevy_input_sequence::*;
use std::cmp::Ordering;

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
    assert_eq!(vec![(Modifiers::empty(), KeyCode::A)], keyseq! { A });
    assert_eq!(
        vec![
            (Modifiers::empty(), KeyCode::A),
            (Modifiers::empty(), KeyCode::B),
        ],
        keyseq! { A B }
    );
}

#[test]
fn eq_if_contains_key_in_lhs(){
    let key = Act::KeyChord(Modifiers::CONTROL, KeyCode::A);
    let lhs = Act::Any(vec![key.clone()]);
    let rhs = key;
    assert!(lhs == rhs);
    assert!(rhs == lhs);
}

#[test]
fn ord_eq_if_contains_key_in_lhs(){
    let key = Act::KeyChord(Modifiers::CONTROL, KeyCode::A);
    let lhs = Act::Any(vec![key.clone()]);
    let rhs = key;
    assert_eq!(lhs.cmp(&rhs), Ordering::Equal);
}

#[test]
fn ord_not_eq_if_non_contains_key_in_lhs(){
    let lhs = Act::Any(vec![Act::KeyChord(Modifiers::CONTROL, KeyCode::A)]);
    let rhs = Act::KeyChord(Modifiers::CONTROL, KeyCode::B);
    // If not Equal, `lhs` is Greater or Less? It needs to be implemented with consistency.
    assert_eq!(lhs.cmp(&rhs), Ordering::Less);
}

/// XXX: This doc test isn't working.
///
/// ```compile_fail
/// assert_eq!((Modifiers::CONTROL, KeyCode::F2), key!{ ctrl-f2 });
/// ```
#[allow(unused_must_use)]
#[test]
fn test_key_macro() {
    assert_eq!((Modifiers::CONTROL, KeyCode::B), key! { ctrl-B });
    assert_eq!((Modifiers::CONTROL, KeyCode::Key1), key! { ctrl-1 });
    assert_eq!((Modifiers::CONTROL, KeyCode::Key2), key! { ctrl-2 });
    assert_eq!((Modifiers::CONTROL, KeyCode::F2), key! { ctrl-F2 });
    // assert_eq!((Modifiers::CONTROL, KeyCode::F2), key!{ ctrl-f2 });
    assert_eq!((Modifiers::CONTROL, KeyCode::Semicolon), key! { ctrl-; });
    assert_eq!((Modifiers::CONTROL, KeyCode::Caret), key! { ctrl-^ });
    // assert_eq!((Modifiers::CONTROL, KeyCode::Colon), key! { ctrl-: });
    assert_eq!((Modifiers::CONTROL, KeyCode::Equals), key! { ctrl-= });
    assert_eq!((Modifiers::CONTROL, KeyCode::Comma), key! { ctrl-, });
    assert_eq!((Modifiers::CONTROL, KeyCode::Period), key! { ctrl-. });
    assert_eq!((Modifiers::CONTROL, KeyCode::Slash), key! { ctrl-/ });
    assert_eq!((Modifiers::CONTROL, KeyCode::Minus), key! { ctrl-- });
    assert_eq!((Modifiers::CONTROL, KeyCode::Underline), key! { ctrl-_ });
    assert_eq!((Modifiers::CONTROL, KeyCode::Colon), key! { ctrl-: });
    assert_eq!(
        (Modifiers::CONTROL, KeyCode::Apostrophe),
        key! { ctrl-'\'' }
    );

    assert_eq!(
        (Modifiers::CONTROL | Modifiers::SHIFT, KeyCode::A),
        key! { ctrl-shift-A }
    );
    // assert_eq!((Modifiers::CONTROL, KeyCode::A), key!{ ctrl-A });
    assert_eq!((Modifiers::SUPER, KeyCode::A), key! { super-A });
    assert_eq!((Modifiers::CONTROL, KeyCode::A), key! { ctrl-A }); // Allow lowercase or demand lowercase?
    assert_eq!((Modifiers::empty(), KeyCode::A), key! { A });
    let k = (Modifiers::empty(), KeyCode::A);
    assert_eq!(k, key! { A });
    assert_eq!(
        (Modifiers::CONTROL, KeyCode::Asterisk),
        key! { ctrl-Asterisk }
    ); // All bevy KeyCode names work.
    assert_eq!((Modifiers::CONTROL, KeyCode::Asterisk), key! { ctrl-* }); // with some short hand.

    assert_eq!((Modifiers::CONTROL, KeyCode::Plus), key! { ctrl-+ });
    assert_eq!((Modifiers::CONTROL, KeyCode::At), key! { ctrl-@ });
    assert_eq!((Modifiers::CONTROL, KeyCode::Grave), key! { ctrl-'`' });
    assert_eq!((Modifiers::CONTROL, KeyCode::Backslash), key! { ctrl-'\\' });
    assert_eq!((Modifiers::CONTROL, KeyCode::Escape), key! { ctrl-Escape });
    // assert_eq!((Modifiers::CONTROL, KeyCode::Escape), key!{ ctrl-Esc });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::A),
        key! { ctrl-alt-A }
    );

    assert_eq!((Modifiers::empty(), KeyCode::A), key! { A });
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::A),
        key! { ctrl-alt-A }
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::A),
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
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Colon),
        key! { ctrl-alt-: }
    );
    assert_eq!(
        (Modifiers::CONTROL | Modifiers::ALT, KeyCode::Slash),
        key! { ctrl-alt-/ }
    );
}

#[allow(unused_must_use)]
#[test]
fn test_keyseq() {
    assert_eq!(vec![(Modifiers::CONTROL, KeyCode::A)], keyseq! { ctrl-A });
    assert_eq!(
        vec![(Modifiers::CONTROL, KeyCode::A)],
        keyseq! { ctrl-ctrl-A }
    );
    assert_eq!(
        vec![
            (Modifiers::CONTROL, KeyCode::A),
            (Modifiers::ALT, KeyCode::B)
        ],
        keyseq! { ctrl-A alt-B }
    );

    assert_eq!(
        vec![
            (Modifiers::empty(), KeyCode::A),
            (Modifiers::empty(), KeyCode::B)
        ],
        keyseq! { A B }
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
