pub struct MouseData{
    pub delta_x: f32,
    pub delta_y: f32,
    pub delta_wheel: f32,
    pub button_state: u8,
}

impl MouseData {
    pub fn new(data_string: String) -> Self {

        let mut split = data_string.split(';');
        let dx = split.next().unwrap_or("0").parse::<f32>().unwrap_or(0f32);
        let dy = split.next().unwrap_or("0").parse::<f32>().unwrap_or(0f32);
        let dw = split.next().unwrap_or("0").parse::<f32>().unwrap_or(0f32);
        let buttons = split.next().unwrap_or("0").parse::<u8>().unwrap_or(0u8);

        Self{ delta_x: dx, delta_y: dy, delta_wheel: dw, button_state: buttons }
    }
}