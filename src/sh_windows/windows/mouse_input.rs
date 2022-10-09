use std::sync::mpsc;
use winapi::{um, shared};

pub fn start_mouse_input(receiver: mpsc::Receiver<String>){

    let speed_multiplier = 8.0;
    let scroll_speed_multiplier = 10.0;

    loop {
        if let Ok(message) = receiver.recv() {
            let mut data = message.split(';');

            let motion_x = data.next();
            let motion_y = data.next();
            let _delta_wheel = data.next();
            let _button_state = data.next();

            let mut delta_x = 0i32;
            let mut delta_y = 0i32;

            if let Some(motion_x) = motion_x{
                if let Ok(dx) = motion_x.to_string().parse::<f32>(){
                    if dx.abs() > 0.0 {
                        delta_x = (dx * speed_multiplier) as i32;
                    }
                }
            }

            if let Some(motion_y) = motion_y{
                if let Ok(dy) = motion_y.to_string().parse::<f32>(){
                    if dy.abs() > 0.0 {
                        delta_y = (dy * speed_multiplier) as i32;
                    }
                }
            }

            unsafe {
                let mut current_position = shared::windef::POINT{x: 0, y: 0};
                let current_position_ptr: *mut shared::windef::POINT = &mut current_position;
                um::winuser::GetCursorPos(current_position_ptr);
                um::winuser::SetCursorPos(current_position.x + delta_x, current_position.y - delta_y);
            }
        }
    }
}