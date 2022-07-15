extern crate core;

use std::{io, str};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::channel;

use evdev_rs::{DeviceWrapper, InputEvent, TimeVal, UInputDevice, UninitDevice};
use evdev_rs::enums::{BusType, EV_KEY, EV_REL, EV_SYN, EventCode, EventType};
use evdev_rs::util::event_code_to_int;

use crate::sh::socketed_event_sorter::SocketedEventSorter;
use crate::sh::uinput_virtual_pointer::UinputVirtualPointer;
use crate::sh::virtual_pointer::VirtualPointer;

mod sh;

fn main(){
    let eventHandler = SocketedEventSorter::new("2137");

    match eventHandler {
        None => {
            panic!("Failed to start event handler on port: 2137.\nIs the port in use?");
        }
        Some(handler) => {
            println!("Started event handler on port: 2137.");
            let (rx1, tx1) = channel::<String>();
            let (rx2, tx2) = channel::<String>();
            SocketedEventSorter::create_cursor_input(tx1);
            SocketedEventSorter::create_osu_input(tx2);
            handler.start_server(rx1, rx2);
        }
    }
}