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

fn get_midi_button(messages: Arc<Mutex<Vec<u8>>>) -> Vec<u8> {
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
        let midi_in = MidiInput::new("midir reading input").unwrap();
        let mut ctx = Ctx {
            messages: Arc::new(Mutex::new(vec![0, 0, 0])), 
        };
        let messagei = Arc::clone(&ctx.messages);
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
        main_menu(&mut ctx);
    }
}

fn main_menu(ctx: &mut Ctx) {
    loop {
        let message = get_midi_button(Arc::clone(&ctx.messages));
        match message[1] {
            21 => FreeScale::run(ctx),
            _ => main_menu(ctx),
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


