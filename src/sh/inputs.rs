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

                    if let Some(event) = z_key.get_event(converted) {
                        events.push(event);
                    }

                    if let Some(event) = x_key.get_event(converted) {
                        events.push(event);
                    }

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
            keys.insert(Key::KEY_EQUAL);
            keys.insert(Key::KEY_MINUS);
            keys.insert(Key::KEY_LEFTBRACE);
            keys.insert(Key::KEY_RIGHTBRACE);
            keys.insert(Key::KEY_SEMICOLON);
            keys.insert(Key::KEY_APOSTROPHE);
            keys.insert(Key::KEY_BACKSLASH);
            keys.insert(Key::KEY_COMMA);
            keys.insert(Key::KEY_DOT);
            keys.insert(Key::KEY_SLASH);

            keys.insert(Key::KEY_ESC);
            keys.insert(Key::KEY_GRAVE);
            keys.insert(Key::KEY_SPACE);
            keys.insert(Key::KEY_ENTER);
            keys.insert(Key::KEY_TITLE);
            keys.insert(Key::KEY_LEFTSHIFT);
            keys.insert(Key::KEY_RIGHTSHIFT);
            keys.insert(Key::KEY_LEFTALT);
            keys.insert(Key::KEY_RIGHTALT);
            keys.insert(Key::KEY_LEFTCTRL);
            keys.insert(Key::KEY_RIGHTCTRL);
            keys.insert(Key::KEY_TAB);
            keys.insert(Key::KEY_BACKSPACE);
            keys.insert(Key::KEY_CAPSLOCK);

            keys.insert(Key::KEY_UP);
            keys.insert(Key::KEY_DOWN);
            keys.insert(Key::KEY_LEFT);
            keys.insert(Key::KEY_RIGHT);

            keys.insert(Key::KEY_INSERT);
            keys.insert(Key::KEY_DELETE);
            keys.insert(Key::KEY_HOME);
            keys.insert(Key::KEY_END);
            keys.insert(Key::KEY_PAGEUP);
            keys.insert(Key::KEY_PAGEDOWN);

            keys.insert(Key::KEY_PRINT);
            keys.insert(Key::KEY_SCROLLLOCK);
            keys.insert(Key::KEY_PAUSE);
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

            KeyInputU128::new(60, Key::KEY_UP),
            KeyInputU128::new(61, Key::KEY_DOWN),
            KeyInputU128::new(62, Key::KEY_LEFT),
            KeyInputU128::new(63, Key::KEY_RIGHT),

            KeyInputU128::new(64, Key::KEY_RIGHTALT),
            KeyInputU128::new(65, Key::KEY_RIGHTCTRL),
            KeyInputU128::new(66, Key::KEY_RIGHTSHIFT),

            KeyInputU128::new(67, Key::KEY_MINUS),
            KeyInputU128::new(68, Key::KEY_EQUAL),
            KeyInputU128::new(69, Key::KEY_LEFTBRACE),
            KeyInputU128::new(70, Key::KEY_RIGHTBRACE),
            KeyInputU128::new(71, Key::KEY_SEMICOLON),
            KeyInputU128::new(72, Key::KEY_APOSTROPHE),
            KeyInputU128::new(73, Key::KEY_BACKSLASH),
            KeyInputU128::new(74, Key::KEY_COMMA),
            KeyInputU128::new(75, Key::KEY_DOT),
            KeyInputU128::new(76, Key::KEY_SLASH),

            KeyInputU128::new(77, Key::KEY_INSERT),
            KeyInputU128::new(78, Key::KEY_DELETE),
            KeyInputU128::new(79, Key::KEY_HOME),
            KeyInputU128::new(80, Key::KEY_END),
            KeyInputU128::new(81, Key::KEY_PAGEUP),
            KeyInputU128::new(82, Key::KEY_PAGEDOWN),
            KeyInputU128::new(83, Key::KEY_PRINT),
            KeyInputU128::new(84, Key::KEY_SCROLLLOCK),
            KeyInputU128::new(85, Key::KEY_PAUSE),
        ];

        loop {
            if let Ok(message) = receiver.recv() {
                if let Ok(converted) = message.parse::<u128>() {

                    let mut events: Vec<InputEvent> = vec![];

                    for key in keys.iter_mut() {
                        if let Some(event) = key.get_event(converted){
                            events.push(event);
                        }
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
    });
}
