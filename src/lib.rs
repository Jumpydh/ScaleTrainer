use std::{
    io::{stdin, stdout, Write},
    sync::Arc,
    sync::Mutex,
};

use free_scales::FreeScales;
use midir::{Ignore, MidiInput, MidiInputPort};
mod free_scales;

pub struct ScaleTrainer {}

unsafe impl Send for ScaleTrainer {}

impl ScaleTrainer {
    pub fn new() -> ScaleTrainer {
        let mut st = ScaleTrainer {};
        st.run();
        st
    }
    fn run(&mut self) {
        let in_port = get_midi_input();
        let midi_in = MidiInput::new("midir reading input").unwrap();
        let messages = Arc::new(Mutex::new(vec![0, 0, 0]));
        let mut state: Box<dyn AppState + Send> = Box::new(MainMenuState);
        state.enter();
        let messagei = Arc::clone(&messages);
        let _conn_in = midi_in.connect(
            &in_port.unwrap(),
            "midir-read-input",
            move |_stamp, message, _| {
                let mut messages = messagei.lock().unwrap();
                *messages = message.clone().to_vec();
                // println!("{}: {:?} (len = {})", stamp, message, message.len());
            },
            (),
        );

        // let state = Box::new(MainMenuState);
        //Keep scope alive
        loop {
            // get message
            let mut messages = messages.lock().unwrap();
            if messages[0] == 0 {
                continue;
            }
            // pop message
            let message = messages.clone();
            *messages = vec![0, 0, 0];
            // handle message

            state = state.handle(&message);
        }
    }
}
fn get_midi_input() -> Result<MidiInputPort, Box<dyn std::error::Error>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
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

trait AppState {
    fn enter(&self);
    fn handle(self: Box<Self>, message: &[u8]) -> Box<dyn AppState + Send>;
}

#[derive(Clone, Copy)]
struct MainMenuState;

unsafe impl Send for MainMenuState {}

impl AppState for MainMenuState {
    fn enter(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("Main menu");
        println!("1. Free scales: Press A0");
    }
    fn handle(self: Box<Self>, message: &[u8]) -> Box<dyn AppState + Send> {
        match message[1] {
            21 => Box::new(FreeScalesState {
                free_scales: FreeScales::new(),
            }),
            _ => Box::new(MainMenuState),
        }
    }
}

struct FreeScalesState {
    free_scales: FreeScales,
}

unsafe impl Send for FreeScalesState {}

impl AppState for FreeScalesState {
    fn enter(&self) {
        free_scales::FreeScales::new();
    }
    fn handle(mut self: Box<Self>, message: &[u8]) -> Box<dyn AppState + Send> {
        let free_scales = &mut self.free_scales;
        match free_scales.run(message){
            true => Box::new(MainMenuState),
            false => self
        }
    }
}

// unsafe impl Send for dyn AppState {}

impl dyn AppState {}
