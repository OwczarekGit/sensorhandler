use std::sync::mpsc;

use evdev::{AttributeSet, EventType, InputEvent, Key, RelativeAxisType};
use evdev::uinput::VirtualDeviceBuilder;

use crate::sh_linux::linux::key_input::KeyInputU8;

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
            let mut data = message.split(';');

            let motion_x = data.next();
            let motion_y = data.next();
            let delta_wheel = data.next();
            let button_state = data.next();

            let mut events: Vec<InputEvent> = vec![];

            if let Some(motion_x) = motion_x {
                if let Ok(dx) = motion_x.to_string().parse::<f32>(){
                    if dx.abs() > 0.0 {
                        events.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_X.0, (dx * speed_multiplier) as i32));
                    }
                }
            }

            if let Some(motion_y) = motion_y {
                if let Ok(dy) = motion_y.to_string().parse::<f32>(){
                    if dy.abs() > 0.0 {
                        events.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_Y.0, -(dy * speed_multiplier) as i32));
                    }
                }
            }

            if let Some(delta_wheel) = delta_wheel {
                if let Ok(d) = delta_wheel.to_string().parse::<f32>(){
                    if d.abs() > 0f32 {
                        events.push(InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_WHEEL.0, (d * scroll_speed_multiplier) as i32));
                    }
                }
            }

            if let Some(button_state) = button_state{
                if let Ok(button_state) = button_state.parse::<u8>(){

                    if let Some(event) = button_left.get_event(button_state){
                        events.push(event);
                    }

                    if let Some(event) = button_right.get_event(button_state){
                        events.push(event);
                    }

                    if let Some(event) = button_middle.get_event(button_state){
                        events.push(event);
                    }
                }
            }

            let _ = device.emit(&events);
        }
    }
}
