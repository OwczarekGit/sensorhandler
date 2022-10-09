extern crate core;

use std::sync::mpsc;
use std::thread;

use clap::Parser;
pub mod sh;

use crate::sh::server::Server;
use crate::sh::senders::Senders;

#[cfg(target_os = "linux")]
pub mod sh_linux;
#[cfg(target_os = "linux")]
use crate::sh_linux::linux::mouse_input;
#[cfg(target_os = "linux")]
use crate::sh_linux::linux::keyboard_input;
#[cfg(target_os = "linux")]
use crate::sh_linux::linux::osu_input;

#[cfg(target_os = "windows")]
pub mod sh_windows;
#[cfg(target_os = "windows")]
use crate::sh_windows::windows::mouse_input;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config{
    #[clap(short = 'p', long = "port", default_value = "2137")]
    port: String,
}

fn main() {
    let config = Config::parse();
    let (senders, osu_receiver, keyboard_receiver, mouse_receiver) = create_channels();

    #[cfg(target_os = "linux")]
    thread::spawn(move || mouse_input::start_mouse_input(mouse_receiver));
    #[cfg(target_os = "linux")]
    thread::spawn(move || keyboard_input::start_keyboard_input(keyboard_receiver));
    #[cfg(target_os = "linux")]
    thread::spawn(move || osu_input::start_osu_input(osu_receiver));

    #[cfg(target_os = "windows")]
    thread::spawn(move || mouse_input::start_mouse_input(mouse_receiver));

    let mut server = Server::new(&config.port);
    server.set_senders(senders);
    server.start();
}

pub fn create_channels() -> (Senders, mpsc::Receiver<String>, mpsc::Receiver<String>, mpsc::Receiver<String>){
    let (keyboard_sender, keyboard_receiver) = mpsc::channel::<String>();
    let (osu_sender, osu_receiver) = mpsc::channel::<String>();
    let (mouse_sender,mouse_receiver) = mpsc::channel::<String>();

    (Senders{osu_sender, keyboard_sender, mouse_sender }, osu_receiver, keyboard_receiver, mouse_receiver)
}

