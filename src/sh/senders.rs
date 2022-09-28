use std::sync::mpsc;

pub struct Senders {
    pub osu_sender: mpsc::Sender<String>,
    pub keyboard_sender: mpsc::Sender<String>,
    pub mouse_sender: mpsc::Sender<String>,
}

impl Clone for Senders {
    fn clone(&self) -> Self {
        Self{
            osu_sender: self.osu_sender.clone(),
            keyboard_sender: self.keyboard_sender.clone(),
            mouse_sender: self.mouse_sender.clone(),
        }
    }
}
