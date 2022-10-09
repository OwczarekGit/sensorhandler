extern crate core;

use std::thread;

use clap::Parser;

use crate::sh::{key_input::create_channels, keyboard_input, mouse_input, osu_input};
use crate::sh::server::Server;

pub mod sh;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config{
    #[clap(short = 'p', long = "port", default_value = "2137")]
    port: String,
}

fn main() {
    let config = Config::parse();
    let (senders, osu_receiver, keyboard_receiver, mouse_receiver) = create_channels();

    thread::spawn(move || osu_input::start_osu_input(osu_receiver));
    thread::spawn(move || keyboard_input::start_keyboard_input(keyboard_receiver));
    thread::spawn(move || mouse_input::start_mouse_input(mouse_receiver));

    let mut server = Server::new(&config.port);
    server.set_senders(senders);
    server.start();
}



