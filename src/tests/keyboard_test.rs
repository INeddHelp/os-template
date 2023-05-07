use crate::drivers::keyboard::{self, Keycode};

fn test_keyboard() {
    let mut input_buffer = [0u8; 8];

    // Test a single key press
    let mut key_event = keyboard::KeyEvent::new(Keycode::Char('a'), false);
    keyboard::handle_key_event(&mut input_buffer, &mut key_event);
    assert_eq!(input_buffer, ['a' as u8, 0, 0, 0, 0, 0, 0, 0]);

    // Test a single key release
    key_event = keyboard::KeyEvent::new(Keycode::Char('a'), true);
    keyboard::handle_key_event(&mut input_buffer, &mut key_event);
    assert_eq!(input_buffer, [0, 0, 0, 0, 0, 0, 0, 0]);

    // Test multiple key presses
    key_event = keyboard::KeyEvent::new(Keycode::Char('a'), false);
    keyboard::handle_key_event(&mut input_buffer, &mut key_event);
    assert_eq!(input_buffer, ['a' as u8, 0, 0, 0, 0, 0, 0, 0]);

    key_event = keyboard::KeyEvent::new(Keycode::Char('b'), false);
    keyboard::handle_key_event(&mut input_buffer, &mut key_event);
    assert_eq!(input_buffer, ['a' as u8, 'b' as u8, 0, 0, 0, 0, 0, 0]);

    // Test multiple key releases
    key_event = keyboard::KeyEvent::new(Keycode::Char('a'), true);
    keyboard::handle_key_event(&mut input_buffer, &mut key_event);
    assert_eq!(input_buffer, ['b' as u8, 0, 0, 0, 0, 0, 0, 0]);

    key_event = keyboard::KeyEvent::new(Keycode::Char('b'), true);
    keyboard::handle_key_event(&mut input_buffer, &mut key_event);
    assert_eq!(input_buffer, [0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_keyboard_driver() {
    test_keyboard();
}
