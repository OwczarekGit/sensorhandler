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
pub struct KeyInput{
    mask: u128,
    key: Key,
    current_state: u8,
}

impl KeyInput {
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

pub fn start_osu_input(receiver: mpsc::Receiver<String>){
    thread::spawn(move || {

        let mut keys = AttributeSet::<Key>::new();
        keys.insert(Key::KEY_Z);
        keys.insert(Key::KEY_X);

        let mut device = VirtualDeviceBuilder::new().expect("Failed to create virtual device: osu!")
            .name("Virtual osu! input")
            .with_keys(&keys).expect("Failed to init keys for osu! input.")
            .build().unwrap();

        let mut z_key = KeyInput::new(0, Key::KEY_Z);
        let mut x_key = KeyInput::new(1, Key::KEY_X);

        loop{
            if let Ok(message) = receiver.recv(){
                if let Ok(converted) = message.parse::<u128>(){

                    let mut events: Vec<InputEvent> = vec![];

                    z_key.extract_value_from_mask(converted);
                    events.push(z_key.get_event());

                    x_key.extract_value_from_mask(converted);
                    events.push(x_key.get_event());


                    device.emit(&events).unwrap();
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
            keys.insert(Key::KEY_BACKSPACE);
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
            KeyInput::new(1,  Key::KEY_0),
            KeyInput::new(2,  Key::KEY_1),
            KeyInput::new(3,  Key::KEY_2),
            KeyInput::new(4,  Key::KEY_3),
            KeyInput::new(5,  Key::KEY_4),
            KeyInput::new(6,  Key::KEY_5),
            KeyInput::new(7,  Key::KEY_6),
            KeyInput::new(8,  Key::KEY_7),
            KeyInput::new(9,  Key::KEY_8),
            KeyInput::new(10, Key::KEY_9),

            KeyInput::new(11, Key::KEY_A),
            KeyInput::new(12, Key::KEY_B),
            KeyInput::new(13, Key::KEY_C),
            KeyInput::new(14, Key::KEY_D),
            KeyInput::new(15, Key::KEY_E),
            KeyInput::new(16, Key::KEY_F),
            KeyInput::new(17, Key::KEY_G),
            KeyInput::new(18, Key::KEY_H),
            KeyInput::new(19, Key::KEY_I),
            KeyInput::new(20, Key::KEY_J),
            KeyInput::new(21, Key::KEY_K),
            KeyInput::new(22, Key::KEY_L),
            KeyInput::new(23, Key::KEY_M),
            KeyInput::new(24, Key::KEY_N),
            KeyInput::new(25, Key::KEY_O),
            KeyInput::new(26, Key::KEY_P),
            KeyInput::new(27, Key::KEY_Q),
            KeyInput::new(28, Key::KEY_R),
            KeyInput::new(29, Key::KEY_S),
            KeyInput::new(30, Key::KEY_T),
            KeyInput::new(31, Key::KEY_U),
            KeyInput::new(32, Key::KEY_V),
            KeyInput::new(33, Key::KEY_W),
            KeyInput::new(34, Key::KEY_X),
            KeyInput::new(35, Key::KEY_Y),
            KeyInput::new(36, Key::KEY_Z),

            KeyInput::new(37, Key::KEY_ESC),
            KeyInput::new(38, Key::KEY_GRAVE),
            KeyInput::new(39, Key::KEY_SPACE),
            KeyInput::new(40, Key::KEY_ENTER),
            KeyInput::new(41, Key::KEY_TITLE),
            KeyInput::new(42, Key::KEY_LEFTSHIFT),
            KeyInput::new(43, Key::KEY_BACKSPACE),

            KeyInput::new(44, Key::KEY_F1),
            KeyInput::new(45, Key::KEY_F2),
            KeyInput::new(46, Key::KEY_F3),
            KeyInput::new(47, Key::KEY_F4),
            KeyInput::new(48, Key::KEY_F5),
            KeyInput::new(49, Key::KEY_F6),
            KeyInput::new(50, Key::KEY_F7),
            KeyInput::new(51, Key::KEY_F8),
            KeyInput::new(52, Key::KEY_F9),
            KeyInput::new(53, Key::KEY_F10),
            KeyInput::new(54, Key::KEY_F11),
            KeyInput::new(55, Key::KEY_F12),
        ];

        loop {
            if let Ok(message) = receiver.recv() {
                if let Ok(converted) = message.parse::<u128>() {

                    let mut events: Vec<InputEvent> = vec![];

                    for key in keys.iter_mut() {
                        key.extract_value_from_mask(converted);
                        events.push(key.get_event());
                    }

                    device.emit(&events);
                }
            }
        }
    });
}

pub fn start_mouse_input(receiver: mpsc::Receiver<String>){
    thread::spawn(move || {

        let speed_multiplier = 8.0;

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

        let mut button_left = KeyInput::new(0, Key::BTN_LEFT);
        let mut button_right = KeyInput::new(1, Key::BTN_RIGHT);

        loop{
            if let Ok(message) = receiver.recv(){
                let mut data = message.split(';');
                let motion_x = data.next();
                let motion_y = data.next();
                // let delta_wheel = message.split(";").next();
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

                if let Some(button_state) = button_state{
                    if let Ok(button_state) = button_state.parse::<u128>(){

                        button_left.extract_value_from_mask(button_state);
                        events.push(button_left.get_event());

                        button_right.extract_value_from_mask(button_state);
                        events.push(button_right.get_event());
                    }
                }

                device.emit(&events);
            }
        }
    });
}
