use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use midir::{Ignore, MidiInput};

#[tokio::main]
async fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

enum State {
    KeySelection,
    OffsetSelection,
    PlayingScale,
}

#[derive(Clone)]
struct Key {
    _midi: u8,
    name: String,
}

impl Key {
    fn new(_midi: u8, name: String) -> Key {
        Key { _midi, name }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let keys = vec![
        Key::new(0, String::from("A")),
        Key::new(1, String::from("A#")),
        Key::new(2, String::from("B")),
        Key::new(3, String::from("C")),
        Key::new(4, String::from("C#")),
        Key::new(5, String::from("D")),
        Key::new(6, String::from("D#")),
        Key::new(7, String::from("E")),
        Key::new(8, String::from("F")),
        Key::new(9, String::from("F#")),
        Key::new(10, String::from("G")),
        Key::new(11, String::from("G#")),
    ];

    let mut state = State::KeySelection;
    let mut input = String::new();

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
    let in_port_name = midi_in.port_name(in_port)?;

    let mut current_scale: Vec<u8> = vec![];

    let mut offset: u8 = 0;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            // println!("{}: {:?} (len = {})", _stamp, message, message.len());
            match (message[0], &state) {
                //TODO: Add key selection
                (144, State::KeySelection) => {
                    match select_key(message,keys.clone())
                    {
                        Ok(scale) => {
                            current_scale = scale;
                            state = State::OffsetSelection;
                        }
                        Err(_) => (),
                    }
                    
                }
                (144, State::OffsetSelection) => {
                    if message[0] == 144 {
                        if message[1] == 21 {
                            println!("Playing scale with no offset");
                            state = State::PlayingScale;
                        } else {
                            offset = message[1] - 21;
                            println!("Playing scale with offset of {}", offset);
                            state = State::PlayingScale;
                        }
                    }
                }
                (144, State::PlayingScale) => match current_scale.len() {
                    1 => {
                        if message[0] == 144 {
                            if message[1] == current_scale[0] + 21+offset{
                                println!("You played key: {}", get_note(&(message[1]), keys.clone()));
                                current_scale.remove(0);
                                println!("You played the scale correctly!");
                                state = State::KeySelection;
                            } else {
                                println!("Wrong key, try again");
                            }
                        }
                    }
                    _ => {
                        if message[0] == 144 {
                            if message[1] == current_scale[0] + 21 + offset{
                                println!("You played key: {}", get_note(&(message[1]), keys.clone()));
                                current_scale.remove(0);
                            } else {
                                println!("Wrong key, try again");
                            }
                        }
                    }
                },
                _ => (),
            }
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    println!("Press A0 to start playing");

    input.clear();

    stdin().read_line(&mut input)?;

    println!("Closing connection");
    Ok(())
}

fn select_key(message: &[u8], keys: Vec<Key>) -> Result<Vec<u8>, &'static str> {
    let major_scale: Vec<u8> = vec![0, 2, 4, 5, 7, 9, 11, 12];
    let natural_minor_scale: Vec<u8> = vec![0, 2, 3, 5, 7, 8, 10, 12];
    let harmonic_minor_scale: Vec<u8> = vec![0, 2, 3, 5, 7, 8, 11, 12];
    let melodic_minor_scale: Vec<u8> = vec![0, 2, 3, 5, 7, 9, 11, 12];
    match message[1] {
        21 => {
            println!(
                "Playing major scale, press {}!",
                get_note(&(major_scale[0] + 21), keys.clone())
            );
            Ok(major_scale)
        }
        23 => {
            println!(
                "Playing natural minor scale, press {}!",
                get_note(&(natural_minor_scale[0] + 21), keys.clone())
            );
            Ok(natural_minor_scale)
        }
        24 => {
            println!(
                "Playing harmonic minor scale, press {}!",
                get_note(&(&harmonic_minor_scale[0] + 21), keys.clone())
            );
            Ok(harmonic_minor_scale)
        }
        26 => {
            println!(
                "Playing melodic minor scale, press {}!",
                get_note(&(&melodic_minor_scale[0] + 21), keys.clone())
            );
            Ok(melodic_minor_scale)
        }
        _ => {
            println!("Invalid key pressed, try again");
            Err("Invalid key pressed")
        }
    }
}

fn get_note(midikey: &u8, _key: Vec<Key>) -> String {
    println!("Midikey: {}", midikey);
    let int = midikey - 21;
    let note = int % 12;
    let octave = int / 12;
    _key[note as usize].name.clone() + &octave.to_string()
}
