use bevy_input_sequence::{key, KeyChord};

#[test]
fn keychord_display() {
    let keychord = KeyChord::from(key!(ctrl - A));
    assert_eq!(format!("{}", keychord), "ctrl-A");
    let keychord = KeyChord::from(key!(ctrl - 1));
    assert_eq!(format!("{}", keychord), "ctrl-1");
    let keychord = KeyChord::from(key!(1));
    assert_eq!(format!("{}", keychord), "1");
}

mod simulate_app {
    use bevy::{
        app::{App, PostUpdate},
        ecs::{
            component::Component,
            event::{Event, EventReader},
            system::{Command, Commands, Query},
            world::World,
        },
        input::{
            gamepad::{
                Gamepad, GamepadAxis, GamepadButton, GamepadButtonType, GamepadConnection,
                GamepadConnectionEvent, GamepadInfo, Gamepads,
            },
            keyboard::KeyCode,
            Axis, ButtonInput as Input,
        },
        MinimalPlugins,
    };
    use bevy_input_sequence::prelude::*;

    #[derive(Event, Clone)]
    struct MyEvent;

    #[derive(Component)]
    struct EventSent(u8);

    trait AddCommand {
        fn add(&mut self, command: impl Command);
    }

    impl AddCommand for World {
        fn add(&mut self, command: impl Command) {
            command.apply(self);
        }
    }

    #[test]
    fn one_key() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [(Modifiers::empty(), KeyCode::KeyA)],
        ));
        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn two_components_one_event() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA],
        ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA],
        ));
        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert_eq!(app.world.query::<&EventSent>().iter(&app.world).count(), 1);
    }

    #[test]
    fn two_presses_two_events() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA],
        ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyB],
        ));
        press_key(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert_eq!(app.world.query::<&EventSent>().iter(&app.world).count(), 2);
    }

    #[test]
    fn two_keycodes_match_first() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB],
        ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyC],
        ));

        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn match_short_seq() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB],
        ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC],
        ));

        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert_eq!(
            app.world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .map(|x| x.0)
                .unwrap(),
            1 // .is_some()
        );

        clear_just_pressed(&mut app, KeyCode::KeyB);
        press_key(&mut app, KeyCode::KeyC);
        app.update();
        assert_eq!(
            app.world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .map(|x| x.0)
                .unwrap(),
            2 // .is_some()
        );

        clear_just_pressed(&mut app, KeyCode::KeyC);
        press_key(&mut app, KeyCode::KeyD);
        app.update();
        assert_eq!(
            app.world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .map(|x| x.0)
                .unwrap(),
            2
        );
    }

    #[test]
    fn two_keycodes_match_second() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB],
        ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyC],
        ));

        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyC);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn two_any_patterns() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB],
        ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyC],
        ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyD],
        ));
        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn two_any_patterns_match_2nd() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB],
        ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyC],
        ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyD],
        ));
        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyD);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn two_keycodes() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB],
        ));

        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn delete_sequences_if_pressed_incorrect_key() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC],
        ));

        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyB);
        press_key(&mut app, KeyCode::KeyD);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());
    }

    #[test]
    fn game_pad_button() {
        let mut app = new_app();

        app.world.send_event(GamepadConnectionEvent::new(
            Gamepad::new(1),
            GamepadConnection::Connected(GamepadInfo {
                name: "".to_string(),
            }),
        ));
        app.world.add(ButtonSequence::new(
            action::send_event_with_input(|_: Gamepad| MyEvent),
            [
                GamepadButtonType::North,
                GamepadButtonType::East,
                GamepadButtonType::South,
            ],
        ));
        app.update();

        press_pad_button(&mut app, GamepadButtonType::North);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed_pad_button(&mut app, GamepadButtonType::North);
        press_pad_button(&mut app, GamepadButtonType::East);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed_pad_button(&mut app, GamepadButtonType::East);
        press_pad_button(&mut app, GamepadButtonType::South);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn multiple_inputs() {
        let mut app = new_app();
        app.world.send_event(GamepadConnectionEvent::new(
            Gamepad::new(1),
            GamepadConnection::Connected(GamepadInfo {
                name: "".to_string(),
            }),
        ));
        // This is no longer possible right now. We could introduce a
        // KeyButtonSequence mixture struct would allow it.
        // app.world.add(KeySequence::new(
        //     action::send_event(MyEvent),
        //     [
        //         (KeyCode::KeyA),
        //         (KeyCode::KeyB),
        //         (KeyCode::KeyC) | Act::PadButton(GamepadButtonType::North.into()),
        //         (GamepadButtonType::C.into()),
        //     ],
        // ));
        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyX],
        ));

        app.world.add(ButtonSequence::new(
            action::send_event_with_input(|_: Gamepad| MyEvent),
            [GamepadButtonType::North, GamepadButtonType::C],
        ));
        app.update();

        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyB);
        press_pad_button(&mut app, GamepadButtonType::North);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed_pad_button(&mut app, GamepadButtonType::North);
        press_pad_button(&mut app, GamepadButtonType::C);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn timeout_1frame() {
        let mut app = new_app();

        app.world.add(
            KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB])
                .time_limit(TimeLimit::Frames(1)),
        );

        // eprintln!("t0");
        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        // eprintln!("t1");
        clear_just_pressed(&mut app, KeyCode::KeyA);
        app.update();

        // eprintln!("t2");
        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());
    }

    #[test]
    fn test_modifier() {
        let mut app = new_app();

        app.world.add(KeySequence::new(
            action::send_event(MyEvent),
            [KeyCode::KeyA, KeyCode::KeyB],
        ));

        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        app.update();

        press_key(&mut app, KeyCode::ControlLeft);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());
        release(&mut app, KeyCode::ControlLeft);
        app.update();

        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn no_timeout_1frame() {
        let mut app = new_app();

        app.world.add(
            KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB])
                .time_limit(TimeLimit::Frames(2)),
        );

        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        app.update();

        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn timeout_3frames() {
        let mut app = new_app();

        app.world.add(
            KeySequence::new(
                action::send_event(MyEvent),
                [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC],
            )
            .time_limit(TimeLimit::Frames(2)),
        );

        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyA);
        app.update();

        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::KeyB);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());
    }

    fn press_key(app: &mut App, key: KeyCode) {
        app.world.resource_mut::<Input<KeyCode>>().press(key);
    }

    fn clear_just_pressed(app: &mut App, key: KeyCode) {
        app.world
            .resource_mut::<Input<KeyCode>>()
            .clear_just_pressed(key);
    }

    fn release(app: &mut App, key: KeyCode) {
        app.world.resource_mut::<Input<KeyCode>>().release(key);
    }

    fn press_pad_button(app: &mut App, game_button: GamepadButtonType) {
        app.world
            .resource_mut::<Input<GamepadButton>>()
            .press(GamepadButton::new(Gamepad::new(1), game_button))
    }

    fn clear_just_pressed_pad_button(app: &mut App, game_button: GamepadButtonType) {
        app.world
            .resource_mut::<Input<GamepadButton>>()
            .clear_just_pressed(GamepadButton::new(Gamepad::new(1), game_button));
    }

    fn read(
        mut commands: Commands,
        mut er: EventReader<MyEvent>,
        mut query: Query<&mut EventSent>,
    ) {
        for _ in er.read() {
            match query.get_single_mut() {
                Ok(ref mut event_sent) => {
                    event_sent.0 += 1;
                }
                _ => {
                    commands.spawn(EventSent(1));
                }
            }
        }
    }

    fn new_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        // app.add_plugins(DefaultPlugins);
        app.add_plugins(
            InputSequencePlugin::default()
                .match_key(true)
                .match_button(true),
        );
        app.add_systems(PostUpdate, read);
        app.add_event::<MyEvent>();
        app.init_resource::<Gamepads>();
        app.init_resource::<Input<GamepadButton>>();
        app.init_resource::<Input<GamepadAxis>>();
        app.init_resource::<Axis<GamepadButton>>();
        app.init_resource::<Axis<GamepadAxis>>();
        app.init_resource::<Input<KeyCode>>();
        app
    }
}
