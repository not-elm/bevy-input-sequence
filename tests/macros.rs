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
        keyseq! { ctrl-A B },
        [
            (Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::empty(), KeyCode::KeyB)
        ]
    );
    assert_eq!(
        keyseq! { ctrl-alt-A Escape },
        [
            (Modifiers::ALT | Modifiers::CONTROL, KeyCode::KeyA),
            (Modifiers::empty(), KeyCode::Escape)
        ]
    );
    assert_eq!(
        keyseq! { ctrl-; },
        [(Modifiers::CONTROL, KeyCode::Semicolon)]
    );
    assert_eq!(
        keyseq! { ctrl-Semicolon },
        [(Modifiers::CONTROL, KeyCode::Semicolon)]
    );
}
