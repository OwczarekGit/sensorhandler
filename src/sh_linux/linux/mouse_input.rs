use std::sync::mpsc;

use evdev::{AttributeSet, EventType, InputEvent, Key, RelativeAxisType};
use evdev::uinput::VirtualDeviceBuilder;

use crate::sh::key_input::KeyInputU8;
use crate::sh::mouse_data::MouseData;

pub fn start_mouse_input(receiver: mpsc::Receiver<String>){

    let speed_multiplier = 8.0;
    let scroll_speed_multiplier = 10.0;

    let mut buttons = AttributeSet::<Key>::new();
    buttons.insert(Key::BTN_LEFT);
    buttons.insert(Key::BTN_RIGHT);
    buttons.insert(Key::BTN_MIDDLE);

    let mut motion = AttributeSet::<RelativeAxisType>::new();
    motion.insert(RelativeAxisType::REL_X);
    motion.insert(RelativeAxisType::REL_Y);
    motion.insert(RelativeAxisType::REL_WHEEL);

    let mut device = VirtualDeviceBuilder::new().expect("Failed to create virtual device: mouse")
        .name("Virtual mouse input.")
        .with_keys(&buttons).expect("Failed to create buttons for input mouse.")
        .with_relative_axes(&motion).expect("Failed to create relative axes for mouse.")
        .build().unwrap();

    let mut button_left = KeyInputU8::new(0, Key::BTN_LEFT);
    let mut button_right = KeyInputU8::new(1, Key::BTN_RIGHT);
    let mut button_middle = KeyInputU8::new(2, Key::BTN_MIDDLE);

    loop{
        if let Ok(message) = receiver.recv(){
            let mouse_state = MouseData::new(message);

            let mut events: Vec<InputEvent> = vec![];

            if mouse_state.delta_x.abs() > 0.0 {
                events.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_X.0, (mouse_state.delta_x * speed_multiplier) as i32));
            }

            if mouse_state.delta_y.abs() > 0.0 {
                events.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_Y.0, -(mouse_state.delta_y * speed_multiplier) as i32));
            }

            if mouse_state.delta_wheel.abs() > 0f32 {
                events.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_WHEEL.0, (mouse_state.delta_wheel * scroll_speed_multiplier) as i32));
            }

            if let Some((key, state)) = button_left.get_event(mouse_state.button_state){
                events.push(InputEvent::new(EventType::KEY, key.code(), state));
            }

            if let Some((key, state)) = button_right.get_event(mouse_state.button_state){
                events.push(InputEvent::new(EventType::KEY, key.code(), state));
            }

            if let Some((key, state)) = button_middle.get_event(mouse_state.button_state){
                events.push(InputEvent::new(EventType::KEY, key.code(), state));
            }

            let _ = device.emit(&events);
        }
    }
}
