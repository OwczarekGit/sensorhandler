use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader};
use super::senders::Senders;

pub struct Server {
    listener: Option<TcpListener>,
    senders: Option<Senders>,
}

impl Server {
    pub fn new(port: &str) -> Self {
        let listener = TcpListener::bind(format!("{}:{}", "0.0.0.0", port));

        match listener {
            Ok(listener) => {
                println!("Socket creation successful.");
                return Self{ listener: Some(listener), senders: None }
            }
            Err(error) => {
                println!("Failed to create socket. Is the port in use?");
                println!("{}", error);
                Self{ listener: None, senders: None }
            }
        }
    }

    pub fn set_senders(&mut self, senders: Senders){
        self.senders = Some(senders);
    }

    // #[you_can::turn_off_the_borrow_checker]
    pub fn start(&mut self){
        println!("Starting a server…");
        match &self.listener {
            Some(listener) => {
                println!("The server has started.");
                for stream in listener.incoming() {
                    if let Ok(stream) = stream{
                        if let Some(senders) = &self.senders{
                            self.create_connection(stream, senders.clone());
                        }
                    }
                }
            },
            None => println!("Unable to start the server!")
        }
    }

    fn create_connection(&self, connection: TcpStream, senders: Senders){
        println!("Device connected from: {}", connection.peer_addr().unwrap());

        thread::spawn(move||{
            let mut reader = BufReader::new(&connection);
            'connection: loop {
                let mut msg = String::new();
                if let Ok(bytes) = reader.read_line(&mut msg){
                    if bytes <= 0  { break 'connection; }
                    Server::sort_and_run_message(msg.trim(), &senders);
                }
            }

            println!("Device disconnected.");
        });
    }

    fn sort_and_run_message(message: &str, senders: &Senders){
        let mut split = message.split('|');
        let protocol = split.next();
        let data = split.next();

        if protocol.is_none() || data.is_none() { return; }

        if let Some(protocol) = protocol{
            match Protocol::from_str(protocol) {
                Protocol::Osu => senders.osu_sender.send(data.unwrap().to_string()).unwrap(),
                Protocol::Keyboard => senders.keyboard_sender.send(data.unwrap().to_string()).unwrap(),
                Protocol::Mouse => senders.mouse_sender.send(data.unwrap().to_string()).unwrap(),
                Protocol::Unknown(str) => println!("Unknown protocol: {}", str),
            }
        }
    }
}

enum Protocol{
    Osu,
    Keyboard,
    Mouse,
    Unknown(String),
}

impl Protocol {
    fn from_str(str: &str) -> Protocol{
        match str{
            "OSU" => Protocol::Osu,
            "KEYBOARD" => Protocol::Keyboard,
            "MOUSE" => Protocol::Mouse,
            v => Protocol::Unknown(v.to_string())
        }
    }
}
