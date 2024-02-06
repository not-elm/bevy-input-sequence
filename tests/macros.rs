use bevy::prelude::*;
use bevy_input_sequence::prelude::Act::*;
use bevy_input_sequence::prelude::*;

#[test]
fn test_keyseq_doc() {
    assert_eq!(keyseq!(A B), [KeyChord(Modifiers::empty(), KeyCode::A), KeyChord(Modifiers::empty(), KeyCode::B)]);
    assert_eq!(keyseq!(ctrl-A B), [KeyChord(Modifiers::Control, KeyCode::A), KeyChord(Modifiers::empty(), KeyCode::B)]);
    assert_eq!(keyseq!(alt-ctrl-A Escape), [KeyChord(Modifiers::Alt | Modifiers::Control, KeyCode::A), KeyChord(Modifiers::empty(), KeyCode::Escape)]);
    assert_eq!(keyseq!(ctrl-;), [KeyChord(Modifiers::Control, KeyCode::Semicolon)]);
    assert_eq!(keyseq!(ctrl-Semicolon), [KeyChord(Modifiers::Control, KeyCode::Semicolon)]);
}
