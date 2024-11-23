use bevy::prelude::*;
use bevy_input_sequence::*;

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
