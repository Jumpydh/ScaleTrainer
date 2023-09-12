use std::{
    io::{stdin, stdout, Write},
    sync::Arc,
    sync::Mutex,
};

use free_scales::FreeScale;
use midir::{Ignore, MidiInput, MidiInputPort};
mod free_scales;

pub struct ScaleTrainer {}

unsafe impl Send for ScaleTrainer {}

fn get_midi_down(messages: Arc<Mutex<Vec<u8>>>,_key: Option<u8>) -> Vec<u8> {
    loop {
        let mut messages = messages.lock().unwrap();
        if messages[0] == 0 || messages[0] == 128 {
            continue;
        }
        // pop message
        let message = messages.clone();
        *messages = vec![0, 0, 0];
        return message;
    }
}
fn get_midi_button(messages: Arc<Mutex<Vec<u8>>>) -> Vec<u8> {
    loop {
        let mut messages = messages.lock().unwrap();
        if messages[0] == 0 {
            continue;
        }
        // pop message
        let message = messages.clone();
        *messages = vec![0, 0, 0];
        return message;
    }
}

struct Ctx {
    messages: Arc<Mutex<Vec<u8>>>,
}


impl ScaleTrainer {
    pub fn new() -> ScaleTrainer {
        let mut st = ScaleTrainer {};
        st.run();
        st
    }
    fn run(&mut self) {
        let in_port = get_midi_input();
        match in_port {
            Ok(_) => (),
            Err(_) => {
                println!("No midi input found");
                return;
            }
        }
        let midi_in = MidiInput::new("midir reading input").unwrap();
        let ctx = Ctx {
            messages: Arc::new(Mutex::new(vec![0, 0, 0])), 
        };
        let message_arc = Arc::clone(&ctx.messages);
        let _conn_in = midi_in.connect(
            &in_port.unwrap(),
            "midir-read-input",
            move |_stamp, message, _| {
                let mut messages = message_arc.lock().unwrap();
                *messages = message.clone().to_vec();
                // println!("{}: {:?} (len = {})", stamp, message, message.len());
            },
            (),
        );

        main_menu(ctx);
    }
}

fn main_menu(mut ctx: Ctx) {
    loop {
        // print!("\x1B[2J\x1B[1;1H");
        println!("Main Menu");
        println!("1. Free Scales: Press A0 to start");
        let message = get_midi_down(Arc::clone(&ctx.messages),None);
        match message[1] {
            21 => FreeScale::run(&mut ctx),
            _ => continue,
        }
    }
}


fn get_midi_input() -> Result<MidiInputPort, Box<dyn std::error::Error>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => {
            let mut input = String::new();
            println!("Could not find any input port");
            println!("Please connect your midi, you might need to restart the App!");
            println!("Press 'r' to retry or any other key to exit: ");
            stdin().read_line(&mut input)?;
            match input.trim(){
                "r" => return get_midi_input(),
                _ => return Err("no input port found".into()),
            }
        }
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
                if midi_in.port_name(p).unwrap().contains("Nord Stage") {
                    return Ok(p.clone());
                }
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let _in_port_name = midi_in.port_name(in_port)?;
    Ok(in_port.clone())
}


