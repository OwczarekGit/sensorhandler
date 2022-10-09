use std::sync::mpsc;
use evdev::{AttributeSet, EventType, InputEvent, Key};
use evdev::uinput::VirtualDeviceBuilder;
use crate::sh::key_input::KeyInputU8;

pub fn start_osu_input(receiver: mpsc::Receiver<String>){

    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_Z);
    keys.insert(Key::KEY_X);

    let mut device = VirtualDeviceBuilder::new().expect("Failed to create virtual device: osu!")
        .name("Virtual osu! input")
        .with_keys(&keys).expect("Failed to init keys for osu! input.")
        .build().unwrap();

    let mut z_key = KeyInputU8::new(0, Key::KEY_Z);
    let mut x_key = KeyInputU8::new(1, Key::KEY_X);

    loop{
        if let Ok(message) = receiver.recv(){
            if let Ok(converted) = message.parse::<u8>(){

                let mut events: Vec<InputEvent> = vec![];

                if let Some((key, state)) = z_key.get_event(converted) {
                    events.push(InputEvent::new(EventType::KEY, key.code(), state));
                }

                if let Some((key, state)) = x_key.get_event(converted) {
                    events.push(InputEvent::new(EventType::KEY, key.code(), state));
                }

                let _ = device.emit(&events).unwrap();
            }
        }
    }
}
