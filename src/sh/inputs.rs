use std::thread;
use std::sync::mpsc;
use evdev::{AttributeSet, EventType, InputEvent, Key, RelativeAxisType};
use evdev::uinput::VirtualDeviceBuilder;
use super::senders::Senders;

pub fn create_channels() -> (Senders, mpsc::Receiver<String>, mpsc::Receiver<String>, mpsc::Receiver<String>){
    let (keyboard_sender, keyboard_receiver) = mpsc::channel::<String>();
    let (osu_sender, osu_receiver) = mpsc::channel::<String>();
    let (mouse_sender,mouse_receiver) = mpsc::channel::<String>();

    (Senders{osu_sender, keyboard_sender, mouse_sender }, osu_receiver, keyboard_receiver, mouse_receiver)
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

    pub fn extract_value_from_mask(&mut self, mask: u128) {
        if self.mask & mask > 0{
            self.current_state = 1;
        }else {
            self.current_state = 0;
        }

    }

    pub fn get_event(&mut self) -> InputEvent{
        InputEvent::new(EventType::KEY, self.key.0, self.current_state as i32)
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

    pub fn extract_value_from_mask(&mut self, mask: u8) {
        if self.mask & mask > 0{
            self.current_state = 1;
        }else {
            self.current_state = 0;
        }

    }

    pub fn get_event(&mut self) -> InputEvent{
        InputEvent::new(EventType::KEY, self.key.0, self.current_state as i32)
    }
}

pub fn start_osu_input(receiver: mpsc::Receiver<String>){
    thread::spawn(move || {

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

                    z_key.extract_value_from_mask(converted);
                    events.push(z_key.get_event());

                    x_key.extract_value_from_mask(converted);
                    events.push(x_key.get_event());


                    let _ = device.emit(&events).unwrap();
                }
            }
        }
    });
}

pub fn start_keyboard_input(receiver: mpsc::Receiver<String>){
    thread::spawn(move || {
       let mut keys = AttributeSet::<Key>::new();
        // A - Z
        {
            keys.insert(Key::KEY_A);
            keys.insert(Key::KEY_B);
            keys.insert(Key::KEY_C);
            keys.insert(Key::KEY_D);
            keys.insert(Key::KEY_E);
            keys.insert(Key::KEY_F);
            keys.insert(Key::KEY_G);
            keys.insert(Key::KEY_H);
            keys.insert(Key::KEY_I);
            keys.insert(Key::KEY_J);
            keys.insert(Key::KEY_K);
            keys.insert(Key::KEY_L);
            keys.insert(Key::KEY_M);
            keys.insert(Key::KEY_N);
            keys.insert(Key::KEY_O);
            keys.insert(Key::KEY_P);
            keys.insert(Key::KEY_Q);
            keys.insert(Key::KEY_R);
            keys.insert(Key::KEY_S);
            keys.insert(Key::KEY_T);
            keys.insert(Key::KEY_U);
            keys.insert(Key::KEY_V);
            keys.insert(Key::KEY_W);
            keys.insert(Key::KEY_X);
            keys.insert(Key::KEY_Y);
            keys.insert(Key::KEY_Z);
        }

        // Numbers
        {
            keys.insert(Key::KEY_0);
            keys.insert(Key::KEY_1);
            keys.insert(Key::KEY_2);
            keys.insert(Key::KEY_3);
            keys.insert(Key::KEY_4);
            keys.insert(Key::KEY_5);
            keys.insert(Key::KEY_6);
            keys.insert(Key::KEY_7);
            keys.insert(Key::KEY_8);
            keys.insert(Key::KEY_9);
        }

        // Misc
        {
            keys.insert(Key::KEY_ESC);
            keys.insert(Key::KEY_GRAVE);
            keys.insert(Key::KEY_SPACE);
            keys.insert(Key::KEY_ENTER);
            keys.insert(Key::KEY_TITLE);
            keys.insert(Key::KEY_LEFTSHIFT);
            keys.insert(Key::KEY_LEFTALT);
            keys.insert(Key::KEY_LEFTCTRL);
            keys.insert(Key::KEY_TAB);
            keys.insert(Key::KEY_BACKSPACE);
            keys.insert(Key::KEY_CAPSLOCK);
        }

        // Function keys
        {
            keys.insert(Key::KEY_F1);
            keys.insert(Key::KEY_F2);
            keys.insert(Key::KEY_F3);
            keys.insert(Key::KEY_F4);
            keys.insert(Key::KEY_F5);
            keys.insert(Key::KEY_F6);
            keys.insert(Key::KEY_F7);
            keys.insert(Key::KEY_F8);
            keys.insert(Key::KEY_F9);
            keys.insert(Key::KEY_F10);
            keys.insert(Key::KEY_F11);
            keys.insert(Key::KEY_F12);
        }

        let mut device = VirtualDeviceBuilder::new().expect("Failed to create keyboard device")
            .name("Virtual keyboard input")
            .with_keys(&keys).expect("Failed to create key for keyboard input.")
            .build().unwrap();

        let mut keys = [
            KeyInputU128::new(0, Key::KEY_0),
            KeyInputU128::new(1, Key::KEY_1),
            KeyInputU128::new(2, Key::KEY_2),
            KeyInputU128::new(3, Key::KEY_3),
            KeyInputU128::new(4, Key::KEY_4),
            KeyInputU128::new(5, Key::KEY_5),
            KeyInputU128::new(6, Key::KEY_6),
            KeyInputU128::new(7, Key::KEY_7),
            KeyInputU128::new(8, Key::KEY_8),
            KeyInputU128::new(9, Key::KEY_9),

            KeyInputU128::new(10, Key::KEY_A),
            KeyInputU128::new(11, Key::KEY_B),
            KeyInputU128::new(12, Key::KEY_C),
            KeyInputU128::new(13, Key::KEY_D),
            KeyInputU128::new(14, Key::KEY_E),
            KeyInputU128::new(15, Key::KEY_F),
            KeyInputU128::new(16, Key::KEY_G),
            KeyInputU128::new(17, Key::KEY_H),
            KeyInputU128::new(18, Key::KEY_I),
            KeyInputU128::new(19, Key::KEY_J),
            KeyInputU128::new(20, Key::KEY_K),
            KeyInputU128::new(21, Key::KEY_L),
            KeyInputU128::new(22, Key::KEY_M),
            KeyInputU128::new(23, Key::KEY_N),
            KeyInputU128::new(24, Key::KEY_O),
            KeyInputU128::new(25, Key::KEY_P),
            KeyInputU128::new(26, Key::KEY_Q),
            KeyInputU128::new(27, Key::KEY_R),
            KeyInputU128::new(28, Key::KEY_S),
            KeyInputU128::new(29, Key::KEY_T),
            KeyInputU128::new(30, Key::KEY_U),
            KeyInputU128::new(31, Key::KEY_V),
            KeyInputU128::new(32, Key::KEY_W),
            KeyInputU128::new(33, Key::KEY_X),
            KeyInputU128::new(34, Key::KEY_Y),
            KeyInputU128::new(35, Key::KEY_Z),

            KeyInputU128::new(36, Key::KEY_F1),
            KeyInputU128::new(37, Key::KEY_F2),
            KeyInputU128::new(38, Key::KEY_F3),
            KeyInputU128::new(39, Key::KEY_F4),
            KeyInputU128::new(40, Key::KEY_F5),
            KeyInputU128::new(41, Key::KEY_F6),
            KeyInputU128::new(42, Key::KEY_F7),
            KeyInputU128::new(43, Key::KEY_F8),
            KeyInputU128::new(44, Key::KEY_F9),
            KeyInputU128::new(45, Key::KEY_F10),
            KeyInputU128::new(46, Key::KEY_F11),
            KeyInputU128::new(47, Key::KEY_F12),

            KeyInputU128::new(48, Key::KEY_ESC),
            KeyInputU128::new(49, Key::KEY_GRAVE),
            KeyInputU128::new(50, Key::KEY_SPACE),
            KeyInputU128::new(51, Key::KEY_ENTER),
            KeyInputU128::new(52, Key::KEY_TITLE),

            KeyInputU128::new(53, Key::KEY_LEFTALT),
            KeyInputU128::new(54, Key::KEY_LEFTMETA),
            KeyInputU128::new(55, Key::KEY_LEFTCTRL),
            KeyInputU128::new(56, Key::KEY_LEFTSHIFT),
            KeyInputU128::new(57, Key::KEY_BACKSPACE),
            KeyInputU128::new(58, Key::KEY_TAB),
            KeyInputU128::new(59, Key::KEY_CAPSLOCK),


        ];

        loop {
            if let Ok(message) = receiver.recv() {
                if let Ok(converted) = message.parse::<u128>() {

                    let mut events: Vec<InputEvent> = vec![];

                    for key in keys.iter_mut() {
                        key.extract_value_from_mask(converted);
                        events.push(key.get_event());
                    }

                    let _ = device.emit(&events);
                }
            }
        }
    });
}

pub fn start_mouse_input(receiver: mpsc::Receiver<String>){
    thread::spawn(move || {

        let speed_multiplier = 8.0;
        let scroll_speed_multiplier = 10.0;

        let mut buttons = AttributeSet::<Key>::new();
        buttons.insert(Key::BTN_LEFT);
        buttons.insert(Key::BTN_RIGHT);
        // buttons.insert(Key::BTN_MIDDLE);

        let mut motion = AttributeSet::<RelativeAxisType>::new();
        motion.insert(RelativeAxisType::REL_X);
        motion.insert(RelativeAxisType::REL_Y);
        motion.insert(RelativeAxisType::REL_WHEEL);

        let mut device = VirtualDeviceBuilder::new().expect("Failed to create virtual device: mouse")
            .name("Virtual mouse input.")
            .with_keys(&buttons).expect("Failed to create buttons for input mouse.")
            .with_relative_axes(&motion).expect("Failed to create relative axes for mouse.")
            .build().unwrap();

        let mut button_left = KeyInputU128::new(0, Key::BTN_LEFT);
        let mut button_right = KeyInputU128::new(1, Key::BTN_RIGHT);

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
                    if let Ok(button_state) = button_state.parse::<u128>(){

                        button_left.extract_value_from_mask(button_state);
                        events.push(button_left.get_event());

                        button_right.extract_value_from_mask(button_state);
                        events.push(button_right.get_event());
                    }
                }

                let _ = device.emit(&events);
            }
        }
    });
}
