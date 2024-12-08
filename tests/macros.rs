use bevy::prelude::*;
use bevy_input_sequence::*;

#[rustfmt::skip]
#[test]
fn before_cargo_format() {
    assert_eq!(
        [key![Ctrl-A],
         key! [Ctrl-A],
         key! [ Ctrl-A ],
         key!{Ctrl-A},
         key! {Ctrl-A},
         key! { Ctrl-A },
         key!(Ctrl-A),
         key! (Ctrl-A),
         key! ( Ctrl-A ),
        ],
        [
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
        ]
    );
}

#[test]
fn after_cargo_format() {
    assert_eq!(
        [
            key![Ctrl - A],
            key![Ctrl - A],
            key![Ctrl - A],
            key! {Ctrl-A},
            key! {Ctrl-A},
            key! { Ctrl-A },
            key!(Ctrl - A),
            key!(Ctrl - A),
            key!(Ctrl - A),
        ],
        [
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::CONTROL, KeyCode::KeyA),
        ]
    );
}

#[test]
fn test_keyseq_doc() {
    assert_eq!(
        keyseq! { A B },
        [
            (Modifiers::empty(), KeyCode::KeyA),
            (Modifiers::empty(), KeyCode::KeyB)
        ]
    );
    assert_eq!(
        keyseq! { Ctrl-A B },
        [
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::empty(), KeyCode::KeyB)
        ]
    );
    assert_eq!(
        keyseq! { Ctrl-Alt-A Escape },
        [
            (Modifiers::ALT | Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::empty(), KeyCode::Escape)
        ]
    );
    assert_eq!(
        keyseq! { Ctrl-; },
        [(Modifiers::CONTROL, KeyCode::Semicolon)]
    );
    assert_eq!(
        keyseq! { Ctrl-Semicolon },
        [(Modifiers::CONTROL, KeyCode::Semicolon)]
    );
}
