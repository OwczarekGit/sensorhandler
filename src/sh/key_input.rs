use std::sync::mpsc;

use evdev::{EventType, InputEvent, Key};

use super::senders::Senders;

pub fn create_channels() -> (Senders, mpsc::Receiver<String>, mpsc::Receiver<String>, mpsc::Receiver<String>){
    let (keyboard_sender, keyboard_receiver) = mpsc::channel::<String>();
    let (osu_sender, osu_receiver) = mpsc::channel::<String>();
    let (mouse_sender,mouse_receiver) = mpsc::channel::<String>();

    (Senders{osu_sender, keyboard_sender, mouse_sender }, osu_receiver, keyboard_receiver, mouse_receiver)
}

#[test]
fn should_fire_only_second_time(){
    let data1 = 0b0001;
    let data2 = 0b0011;
    let data3 = 0b0000;
    let data4 = 0b1111;
    let mut key = KeyInputU128::new(0, Key::KEY_T);

    assert!(key.get_event(data1).is_some());
    assert!(key.get_event(data2).is_none());
    assert!(key.get_event(data3).is_some());
    assert!(key.get_event(data4).is_some());
}

#[derive(Debug)]
pub struct KeyInputU128 {
    mask: u128,
    key: Key,
    current_state: u8,
}

impl KeyInputU128 {
    pub fn new(bit: u8, key: Key) -> Self{
        Self{ mask: 0u128 | (1 << bit), key, current_state: 0 }
    }

    pub fn get_event(&mut self, mask: u128) -> Option<InputEvent>{
        let state = if self.mask & mask > 0 {1} else {0};

        if self.current_state == state {
            return None;
        }

        self.current_state = state;
        return Some(InputEvent::new(EventType::KEY, self.key.0, self.current_state as i32));
    }
}

#[derive(Debug)]
pub struct KeyInputU8 {
    mask: u8,
    key: Key,
    current_state: u8,
}

impl KeyInputU8 {
    pub fn new(bit: u8, key: Key) -> Self{
        Self{ mask: 0u8 | (1 << bit), key, current_state: 0 }
    }

    pub fn get_event(&mut self, mask: u8) -> Option<InputEvent>{
        let state = if self.mask & mask > 0 {1} else {0};

        if self.current_state == state {
            return None;
        }

        self.current_state = state;
        return Some(InputEvent::new(EventType::KEY, self.key.0, self.current_state as i32));
    }
}





