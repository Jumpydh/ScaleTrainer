struct ScaleSelectionState;
struct OffsetSelectionState;
struct PlayingScaleState;
struct PostGameState;

trait State {
    fn exit(&self) -> bool { false}
    fn enter(&self) {}
    fn handle(self: Box<Self>, message: &[u8], context: &mut Context) -> Box<dyn State>;
}

impl State for ScaleSelectionState {
    fn handle(self: Box<Self>, message: &[u8], context: &mut Context) -> Box<dyn State> {
        match message[0] {
            144 => {
                println!("Scale selected");
                match select_scale(message, KEYS.clone()) {
                    Ok(scale) => {
                        context.current_scale = scale;

                        state_enter_offset_selection();
                        return Box::new(OffsetSelectionState);
                    }
                    Err(_) => Box::new(ScaleSelectionState),
                }
            }
            _ => Box::new(ScaleSelectionState),
        }
    }
}

impl State for OffsetSelectionState {
    fn handle(self: Box<Self>, message: &[u8], context: &mut Context) -> Box<dyn State> {
        match message[0] {
            144 => {
                if message[1] == 21 {
                    println!("Playing scale with no offset");
                    Box::new(PlayingScaleState)
                } else {
                    println!("Offset selected");
                    context.offset = message[1] - 21;
                    println!(
                        "Playing scale with offset of {}",
                        get_note(&(context.offset + 21), KEYS.clone())
                    );
                    Box::new(PlayingScaleState)
                }
            }
            _ => Box::new(OffsetSelectionState),
        }
    }
}

impl State for PlayingScaleState {
    fn handle(self: Box<Self>, message: &[u8], context: &mut Context) -> Box<dyn State> {
        match message[0] {
            144 => match context.current_scale.len() {
                1 => match message[1] == context.current_scale[0] + 21 + context.offset {
                    true => {
                        println!("You played key: {}", get_note(&(message[1]), KEYS.clone()));
                        context.current_scale.remove(0);
                        print!("\x1B[2J\x1B[1;1H");
                        println!("You played the scale correctly!");
                        Box::new(PostGameState)
                    }
                    false => {
                        println!("Wrong key, try again");
                        self
                    }
                },
                _ => match message[1] == context.current_scale[0] + 21 + context.offset {
                    true => {
                        println!("You played key: {}", get_note(&(message[1]), KEYS.clone()));
                        context.current_scale.remove(0);
                        self
                    }
                    false => {
                        println!("Wrong key, try again");
                        self
                    }
                },
            },
            _ => Box::new(PlayingScaleState),
        }
    }
}

impl State for PostGameState {
    fn handle(self: Box<Self>, _message: &[u8], _context: &mut Context) -> Box<dyn State> {
        match _message[0] {
            144 => match _message[1] {
                21 => {
                    state_enter_key_selection();
                    Box::new(ScaleSelectionState)
                }
                _ => Box::new(PostGameState),
            },
            _ => Box::new(PostGameState),
        }
    }
}

impl dyn State {}

struct Context {
    current_scale: Vec<u8>,
    offset: u8,
}

impl Context {}

pub struct FreeScales {
    state: Box<dyn State>,
    context: Context,
}

unsafe impl Send for FreeScales {}

unsafe impl Sync for FreeScales {}

impl FreeScales {
    pub fn new() -> FreeScales {
        state_enter_key_selection();
        FreeScales {
            // input,
            state: Box::new(ScaleSelectionState),
            context: Context {
                current_scale: vec![],
                offset: 0,
            },
        }
    }
    pub fn run(&mut self, message: &[u8]) -> bool {
        self.state = Box::new(std::mem::replace(&mut self.state, Box::new(PostGameState))).handle(message, &mut self.context);

        self.state.exit()
    }
}

struct Key {
    _midi: u8,
    name: &'static str,
}

impl Key {
    const fn new(midi: u8, name: &'static str) -> Key {
        Key { _midi: midi, name }
    }
}

const KEYS: &'static [&'static Key] = &[
    &Key::new(0, "C"),
    &Key::new(1, "C#"),
    &Key::new(2, "D"),
    &Key::new(3, "D#"),
    &Key::new(4, "E"),
    &Key::new(5, "F"),
    &Key::new(6, "F#"),
    &Key::new(7, "G"),
    &Key::new(8, "G#"),
    &Key::new(9, "A"),
    &Key::new(10, "A#"),
    &Key::new(11, "B"),
];

fn state_enter_key_selection() {
    print!("\x1B[2J\x1B[1;1H");

    println!("Choose your Scale:");
    println!("Major: A");
    println!("Natural Minor: B");
    println!("Harmonic Minor: C");
    println!("Melodic Minor: D");
}

fn state_enter_offset_selection() {
    print!("\x1B[2J\x1B[1;1H");

    println!("Press the key you want to start on");
}

// fn state_enter_playing_scale() {
//     print!("\x1B[2J\x1B[1;1H");
// }

fn select_scale(message: &[u8], _keys: &[&Key]) -> Result<Vec<u8>, &'static str> {
    let major_scale: Vec<u8> = vec![0, 2, 4, 5, 7, 9, 11, 12];
    let natural_minor_scale: Vec<u8> = vec![0, 2, 3, 5, 7, 8, 10, 12];
    let harmonic_minor_scale: Vec<u8> = vec![0, 2, 3, 5, 7, 8, 11, 12];
    let melodic_minor_scale: Vec<u8> = vec![0, 2, 3, 5, 7, 9, 11, 12];
    match message[1] {
        21 => {
            println!(
                "Playing major scale, press {}!",
                get_note(&(major_scale[0] + 21), KEYS.clone())
            );
            Ok(major_scale)
        }
        23 => {
            println!(
                "Playing natural minor scale, press {}!",
                get_note(&(natural_minor_scale[0] + 21), KEYS.clone())
            );
            Ok(natural_minor_scale)
        }
        24 => {
            println!(
                "Playing harmonic minor scale, press {}!",
                get_note(&(&harmonic_minor_scale[0] + 21), KEYS.clone())
            );
            Ok(harmonic_minor_scale)
        }
        26 => {
            println!(
                "Playing melodic minor scale, press {}!",
                get_note(&(&melodic_minor_scale[0] + 21), KEYS.clone())
            );
            Ok(melodic_minor_scale)
        }
        _ => {
            println!("Invalid key pressed, try again");
            Err("Invalid key pressed")
        }
    }
}

fn get_note(midikey: &u8, _key: &[&Key]) -> String {
    // println!("Midikey: {}", midikey);
    let int = midikey - 21;
    let note = int % 12;
    let octave = int / 12;
    _key[note as usize].name.clone().to_owned() + &octave.to_string()
}
