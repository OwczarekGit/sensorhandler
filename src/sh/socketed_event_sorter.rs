use std::net::{TcpListener, TcpStream};
use std::{io, thread};
use std::borrow::Borrow;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::{UinputVirtualPointer, VirtualPointer};
use crate::sh::uinput_virtual_osu_keyboard::UinputVirtualOsuKeyboard;
use crate::sh::virtual_osu_keyboard::VirtualOsuKeyboard;

pub struct SocketedEventSorter{
    listener: TcpListener,
}

impl SocketedEventSorter {

    pub fn new(port: &str) -> Option<SocketedEventSorter>{

        let tmpListener = TcpListener::bind(String::from(format!("0.0.0.0:{}", port)));

        match tmpListener {
            Ok(res) => {
                Some(Self{ listener: res })
            }
            Err(_) => {
                None
            }
        }
    }

    /// Starts the server, the server is listening for incoming connections on specified port.
    /// It can accept multiple connections at once.
    pub fn start_server(self, inputChannel1: Sender<String>, inputChannel2: Sender<String>){
        for incomingConnection in self.listener.incoming(){
            let tmpSender1 = inputChannel1.clone();
            let tmpSender2 = inputChannel2.clone();
            match incomingConnection {
                Ok(connection) => {
                    thread::spawn(move ||{
                        println!("New device connected.");
                        SocketedEventSorter::handle_device(tmpSender1, tmpSender2, connection);
                    });
                }
                Err(_) => {
                    println!("Connection failed.");
                }
            }
        }
    }

    pub fn create_osu_input(receiver: Receiver<String>){
        thread::spawn(move || {
            let mut vOsuInput = UinputVirtualOsuKeyboard::new("SensorHandler Virtual osu! input", 0x2345, 0x5432);
            let mut zState: i32 = 0;
            let mut xState: i32 = 0;
            let mut escState: i32 = 0;
            let mut spaceState: i32 = 0;
            let mut f2State: i32 = 0;
            let mut buttonState: i32 = 0;

            loop {
                let message = receiver.recv().unwrap();

                buttonState = match message.to_string().trim().parse::<i32>(){
                    Err(e) => {0},
                    Ok(val) => {val}
                };

                if (buttonState & 0x01) == 0x01{
                    zState = 1;
                }else {
                    zState = 0;
                }

                if (buttonState & 0x02) == 0x02{
                    xState = 1;
                }else {
                    xState = 0;
                }

                if (buttonState & 0x04) == 0x04{
                    spaceState = 1;
                }else {
                    spaceState = 0;
                }

                if (buttonState & 0x08) == 0x08{
                    f2State = 1;
                }else {
                    f2State = 0;
                }

                if (buttonState & 0x10) == 0x10{
                    escState = 1;
                }else {
                    escState = 0;
                }

                vOsuInput.set_k1_state(xState);
                vOsuInput.set_k2_state(zState);
                vOsuInput.exec_event();
                vOsuInput.set_back_state(escState);
                vOsuInput.set_skip_state(spaceState);
                vOsuInput.set_random_map_state(f2State);
                vOsuInput.exec_event();
            }
        });
    }

    pub fn create_cursor_input(receiver: Receiver<String>){
        thread::spawn(move ||{
            let mut vPointer = UinputVirtualPointer::new("SensorHandler Virtual Pointing Device", 0x1234, 0x4321);
            let mut LMBState: i32 = 0;
            let mut RMBState: i32 = 0;
            let mut deltaX: f32 = 0.0;
            let mut deltaY: f32 = 0.0;
            let mut buttonState: i32 = 0;

            loop {

                let message = receiver.recv().unwrap();
                let split: Vec<&str> = message.split(";").collect();

                deltaX = split[0].parse::<f32>().unwrap();
                deltaY = split[1].parse::<f32>().unwrap();

                buttonState = match split[2].to_string().trim().parse::<i32>(){
                    Err(e) => {0},
                    Ok(val) => {val}
                };

                if (buttonState & 0x01) == 0x01{
                    LMBState = 1;
                }else {
                    LMBState = 0;
                }

                if (buttonState & 0x02) == 0x02{
                    RMBState = 1;
                }else {
                    RMBState = 0;
                }

                vPointer.pressLMB(LMBState);
                vPointer.execEvent();
                vPointer.pressRMB(RMBState);
                vPointer.execEvent();
                vPointer.moveX(deltaX);
                vPointer.execEvent();
                vPointer.moveY(deltaY);
                vPointer.execEvent();

            }
        });
    }

    pub fn sort_input(pointerSender: &Sender<String>, osuSender: &Sender<String>, message: String){
        let mut sorted: Vec<&str> = message.split('|').collect();

        if sorted.len() < 2 {
            return;
        }

        if sorted[0] == "MOUSE"{
            pointerSender.send(sorted[1].to_string());
        } else {
            osuSender.send(sorted[1].to_string());
        }
    }

    /// Handles the TcpStream for device.
    /// For now it just does what main did but in the future it will redirect JSON formatted events.
    pub fn handle_device(sender1: Sender<String>, sender2: Sender<String>, stream: TcpStream){
        let mut reader = BufReader::new(&stream);

        loop {
            let mut payload = String::new();

            match reader.read_line(&mut payload){
                Ok(_) => {
                    if payload == String::new() {
                        println!("Device has been disconnected.");
                        break;
                    }

                    SocketedEventSorter::sort_input(&sender1, &sender2, payload);
                    continue;
                }
                Err(_) => {
                    eprintln!("Failed to read data from stream.");
                }
            }

        };

    }
}
