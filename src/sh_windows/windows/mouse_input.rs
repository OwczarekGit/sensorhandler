use std::sync::mpsc;
use winsafe;
use winsafe::co::VK;
use crate::sh::key_input::KeyInputU8;
use crate::sh::mouse_data::MouseData;

pub fn start_mouse_input(receiver: mpsc::Receiver<String>){

    let speed_multiplier = 8.0;
    let scroll_speed_multiplier = 10.0;

    let mut delta_x = 0i32;
    let mut delta_y = 0i32;

    let mut button_left = KeyInputU8::new(0, VK::LBUTTON);
    let mut button_right = KeyInputU8::new(1, VK::RBUTTON);
    let mut button_middle = KeyInputU8::new(2, VK::MBUTTON);

    loop {
        if let Ok(message) = receiver.recv() {
            let mouse_state = MouseData::new(message);

            if mouse_state.delta_x.abs() > 0.0 {
                delta_x = (mouse_state.delta_x * speed_multiplier) as i32;
            }

            if mouse_state.delta_y.abs() > 0.0 {
                delta_y = (mouse_state.delta_y * speed_multiplier) as i32;
            }

            if let Some((key, state)) = button_left.get_event(mouse_state.button_state){
                //TODO: Press/Release left mouse.
            }

            if let Some((key, state)) = button_right.get_event(mouse_state.button_state){
                //TODO: Press/Release right mouse.
            }

            if let Some((key, state)) = button_middle.get_event(mouse_state.button_state){
                //TODO: Press/Release middle mouse.
            }

            if let Ok(current_position) = winsafe::GetCursorPos(){
                let _ = winsafe::SetCursorPos(current_position.x + delta_x, current_position.y - delta_y);
            }
        }
    }
}