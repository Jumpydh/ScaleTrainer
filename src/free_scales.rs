use std::sync::Arc;

use crate::{Ctx, get_midi_button, main_menu};

pub(crate) struct FreeScale;
pub struct FsCtx<'a> {
    ctx: &'a mut Ctx,
    scale: Vec<u8>,
    offset: u8,
}

impl FreeScale {
    pub fn run(ctx: &mut Ctx) {
        let scale = ScaleCreator::create_scale(ctx);
        let mut fs_ctx = FsCtx {
            ctx,
            scale,
            offset: 0,
        };
        Self::playing_scale(&mut fs_ctx);
    }

    fn playing_scale(ctx: &mut FsCtx) {
        let mut input:Vec<u8> = vec![];
        loop {
            if input.len() != ctx.scale.len() {
                let message = get_midi_button(Arc::clone(&ctx.ctx.messages));
                if ctx.scale[input.len()] + 21 + ctx.offset == message[1] {
                    println!("Correct");
                    input.push(message[1]);
                } else {
                    println!("Wrong");
                }
                println!(
                    "You played key: {}",
                    get_note(&message[0], KEYS.clone())
                );
            } else {
                break;
            }
        }
    }
}

struct ScaleCreator;

enum Scale {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
}

struct ScaleCreatorArgs {
    scale: Scale,
    octaves: u8,
    offset: u8,
}

impl ScaleCreator {
    fn create_scale(ctx: &mut Ctx) -> Vec<u8> {
        let mut args = ScaleCreatorArgs {
            scale: Scale::Major,
            octaves: 1,
            offset: 0,
        };
        Self::scale_selection(ctx, &mut args);
        Self::octaves_selection(ctx, &mut args);
        Self::startkey_selection(ctx, &mut args);
        Self::build_notes(ctx, &mut args)
    }
    fn scale_selection(ctx: &mut Ctx, args: &mut ScaleCreatorArgs) {
        println!("Choose your Scale:");
        println!("Major: A");
        println!("Natural Minor: B");
        println!("Harmonic Minor: C");
        println!("Melodic Minor: D");
        println!("Press any other key to go back to the main menu");
        let message = get_midi_button(Arc::clone(&ctx.messages));
        match message[1] {
            21 => {
                println!("Playing major scale, press C!");
                args.scale = Scale::Major;
            }
            23 => {
                println!("Playing natural minor scale, press A!");
                args.scale = Scale::NaturalMinor;
            }
            24 => {
                println!("Playing harmonic minor scale, press A!");
                args.scale = Scale::HarmonicMinor;
            }
            26 => {
                println!("Playing melodic minor scale, press A!");
                args.scale = Scale::MelodicMinor;
            }
            _ => main_menu(ctx),
        }
    }
    fn octaves_selection(ctx: &mut Ctx, args: &mut ScaleCreatorArgs) {
        print!("\x1B[2J\x1B[1;1H");
        println!("How many octaves do you want to play?");
        println!("1: A");
        println!("2: B");
        println!("3: C");
        println!("4: D");
        println!("Press any other key to go back to the main menu");
        let message = get_midi_button(Arc::clone(&ctx.messages));
        match message[1] {
            21 => {
                args.octaves = 1;
                println!("Playing 1 octave");
            }
            23 => {
                args.octaves = 2;
                println!("Playing 2 octaves");
            }
            24 => {
                args.octaves = 3;
                println!("Playing 3 octaves");
            }
            26 => {
                args.octaves = 4;
                println!("Playing 4 octaves");
            }
            _ => main_menu(ctx),
        }
    }

    fn startkey_selection(ctx: &mut Ctx, args: &mut ScaleCreatorArgs) {
        println!("Press the key you want to start on");
        let message = get_midi_button(Arc::clone(&ctx.messages));
        match message[1] {
            21 => {
                args.offset = 0;
                println!("Starting on A0");
            }
            23..=111 => {
                args.offset = message[1] - 21;
                println!(
                    "Starting on {}",
                    get_note(&(args.offset + 21), KEYS.clone())
                );
            }
            _ => main_menu(ctx),
        }
    }

    fn build_notes(_: &mut Ctx, args: &mut ScaleCreatorArgs) -> Vec<u8> {
        let mut notes: Vec<u8> = vec![];
        for i in 0..args.octaves {
            match args.scale {
                Scale::Major => {
                    notes.push((0 + i * 12) + (args.offset + 21));
                    notes.push((2 + i * 12) + (args.offset + 21));
                    notes.push((4 + i * 12) + (args.offset + 21));
                    notes.push((5 + i * 12) + (args.offset + 21));
                    notes.push((7 + i * 12) + (args.offset + 21));
                    notes.push((9 + i * 12) + (args.offset + 21));
                    notes.push((11 + i * 12) + (args.offset + 21));
                    notes.push((12 + i * 12) + (args.offset + 21));
                }
                Scale::NaturalMinor => {
                    notes.push((0 + i * 12) + (args.offset + 21));
                    notes.push((2 + i * 12) + (args.offset + 21));
                    notes.push((3 + i * 12) + (args.offset + 21));
                    notes.push((5 + i * 12) + (args.offset + 21));
                    notes.push((7 + i * 12) + (args.offset + 21));
                    notes.push((8 + i * 12) + (args.offset + 21));
                    notes.push((10 + i * 12) + (args.offset + 21));
                    notes.push((12 + i * 12) + (args.offset + 21));
                }
                Scale::HarmonicMinor => {
                    notes.push((0 + i * 12) + (args.offset + 21));
                    notes.push((2 + i * 12) + (args.offset + 21));
                    notes.push((3 + i * 12) + (args.offset + 21));
                    notes.push((5 + i * 12) + (args.offset + 21));
                    notes.push((7 + i * 12) + (args.offset + 21));
                    notes.push((8 + i * 12) + (args.offset + 21));
                    notes.push((11 + i * 12) + (args.offset + 21));
                    notes.push((12 + i * 12) + (args.offset + 21));
                }
                Scale::MelodicMinor => {
                    notes.push((0 + i * 12) + (args.offset + 21));
                    notes.push((2 + i * 12) + (args.offset + 21));
                    notes.push((3 + i * 12) + (args.offset + 21));
                    notes.push((5 + i * 12) + (args.offset + 21));
                    notes.push((7 + i * 12) + (args.offset + 21));
                    notes.push((9 + i * 12) + (args.offset + 21));
                    notes.push((11 + i * 12) + (args.offset + 21));
                    notes.push((12 + i * 12) + (args.offset + 21));
                }
            }
        }
        notes
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


// fn state_enter_playing_scale() {
//     print!("\x1B[2J\x1B[1;1H");
// }


fn get_note(midikey: &u8, _key: &[&Key]) -> String {
    // println!("Midikey: {}", midikey);
    let int = midikey - 21;
    let note = int % 12;
    let octave = int / 12;
    _key[note as usize].name.clone().to_owned() + &octave.to_string()
}
