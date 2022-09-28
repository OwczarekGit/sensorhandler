extern crate core;

use clap::Parser;

pub mod sh;
use crate::sh::{server::Server, inputs::*};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config{
    #[clap(short = 'p', long = "port", default_value = "2137")]
    port: String,
}

fn main() {
    let config = Config::parse();
    let (senders, osu_receiver, keyboard_receiver, mouse_receiver) = create_channels();

    start_osu_input(osu_receiver);
    start_keyboard_input(keyboard_receiver);
    start_mouse_input(mouse_receiver);

    let mut server = Server::new(&config.port);
    server.set_senders(senders);
    server.start();
}



