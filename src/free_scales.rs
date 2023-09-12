use std::{sync::Arc, time};

use crate::{get_midi_down, Ctx};

pub(crate) struct FreeScale;
pub struct FsCtx<'a> {
    ctx: &'a mut Ctx,
    scale: Vec<MidiNote>,
    offset: u8,
}

impl FreeScale {
    pub fn run(ctx: &mut Ctx) {
        let scale = match ScaleCreator::create_scale(ctx) {
            Ok(scale) => scale,
            Err(_) => return,
        };
        let mut fs_ctx = FsCtx {
            ctx,
            scale,
            offset: 0,
        };
        Self::playing_scale_timedmode(&mut fs_ctx);
    }

    fn _playing_scale_waitmode(ctx: &mut FsCtx) {
        let mut input: Vec<u8> = vec![];
        loop {
            if input.len() != ctx.scale.len() {
                let message = get_midi_down(Arc::clone(&ctx.ctx.messages), None);
                if ctx.scale[input.len()].message[1] + 21 + ctx.offset == message[1] + 21 {
                    println!("Correct");
                    input.push(message[1]);
                } else {
                    println!("Wrong");
                }
                println!(
                    "You played key: {}",
                    get_note(&(message[1] + 21), KEYS.clone())
                );
            } else {
                break;
            }
        }
    }
    fn playing_scale_timedmode(ctx: &mut FsCtx) {
        let time = time::SystemTime::now();
        println!("{:?}", time);
        let mut input: Vec<u8> = vec![];
        loop {
            if input.len() != ctx.scale.len() {
                let message = get_midi_down(Arc::clone(&ctx.ctx.messages), None);
                if ctx.scale[input.len()].message[1] + 21 + ctx.offset == message[1] + 21 {
                    println!("Correct");
                    input.push(message[1]);
                } else {
                    println!("Wrong");
                }
                println!(
                    "You played key: {}",
                    get_note(&(message[1] + 21), KEYS.clone())
                );
            } else {
                let time_end = time::SystemTime::now();
                println!("{:?}", time_end.duration_since(time).unwrap().as_secs_f32());
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
    fn create_scale(ctx: &mut Ctx) -> Result<Vec<MidiNote>, ()> {
        let mut args = ScaleCreatorArgs {
            scale: Scale::Major,
            octaves: 1,
            offset: 0,
        };
        match Self::scale_selection(ctx, &mut args) {
            Ok(_) => {}
            Err(_) => return Err(()),
        }
        match Self::octaves_selection(ctx, &mut args) {
            Ok(_) => {}
            Err(_) => return Err(()),
        }
        match Self::startkey_selection(ctx, &mut args) {
            Ok(_) => {}
            Err(_) => return Err(()),
        }
        Ok(Self::build_notes(ctx, &mut args))
    }
    fn scale_selection(ctx: &mut Ctx, args: &mut ScaleCreatorArgs) -> Result<(), ()> {
        println!("Choose your Scale:");
        println!("Major: A");
        println!("Natural Minor: B");
        println!("Harmonic Minor: C");
        println!("Melodic Minor: D");
        println!("Press any other key to go back to the main menu");
        let message = get_midi_down(Arc::clone(&ctx.messages), None);
        match message[1] {
            21 => {
                println!("Playing major scale, press C!");
                args.scale = Scale::Major;
                Ok(())
            }
            23 => {
                println!("Playing natural minor scale, press A!");
                args.scale = Scale::NaturalMinor;
                Ok(())
            }
            24 => {
                println!("Playing harmonic minor scale, press A!");
                args.scale = Scale::HarmonicMinor;
                Ok(())
            }
            26 => {
                println!("Playing melodic minor scale, press A!");
                args.scale = Scale::MelodicMinor;
                Ok(())
            }
            _ => Err(()),
        }
    }
    fn octaves_selection(ctx: &mut Ctx, args: &mut ScaleCreatorArgs) -> Result<(), ()> {
        print!("\x1B[2J\x1B[1;1H");
        println!("How many octaves do you want to play?");
        println!("1: A");
        println!("2: B");
        println!("3: C");
        println!("4: D");
        println!("Press any other key to go back to the main menu");
        let message = get_midi_down(Arc::clone(&ctx.messages), None);
        match message[1] {
            21 => {
                args.octaves = 1;
                println!("Playing 1 octave");
                Ok(())
            }
            23 => {
                args.octaves = 2;
                println!("Playing 2 octaves");
                Ok(())
            }
            24 => {
                args.octaves = 3;
                println!("Playing 3 octaves");
                Ok(())
            }
            26 => {
                args.octaves = 4;
                println!("Playing 4 octaves");
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn startkey_selection(ctx: &mut Ctx, args: &mut ScaleCreatorArgs) -> Result<(), ()> {
        println!("Press the key you want to start on");
        let message = get_midi_down(Arc::clone(&ctx.messages), None);
        match message[1] {
            21 => {
                args.offset = 0;
                println!("Starting on A0");
                Ok(())
            }
            22..=111 => {
                args.offset = message[1] - 21;
                println!("Starting on {}", get_note(&(message[1] + 21), KEYS.clone()));
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn build_scales_up(scale: &[u8], octave: &u8, offset: &u8) -> Vec<MidiNote> {
        let mut notes: Vec<MidiNote> = vec![];
        for i in 0..*octave {
            for note in scale {
                notes.push(MidiNote {
                    _stamp: 0,
                    message: vec![0, (note + i * 12) + (offset + 21), 0],
                });
            }
        }
        notes
    }

    fn build_scales_down(scale: &[u8], octave: &u8, offset: &u8) -> Vec<MidiNote> {
        let mut notes: Vec<MidiNote> = vec![];
        for i in (0..*octave).rev() {
            for note in scale.iter().rev() {
                notes.push(MidiNote {
                    _stamp: 0,
                    message: vec![0, (note + i * 12) + (offset + 21), 0],
                });
            }
        }
        notes
    }

    fn build_notes(_: &mut Ctx, args: &mut ScaleCreatorArgs) -> Vec<MidiNote> {
        let mut notes: Vec<MidiNote> = vec![];
        match args.scale {
            Scale::Major => {
                notes.extend(Self::build_scales_up(
                    MAJOR_SCALE,
                    &args.octaves,
                    &args.offset,
                ));
                notes.push(MidiNote {
                    _stamp: 0,
                    message: vec![0, (12 + (args.octaves - 1) * 12) + (args.offset + 21), 0],
                });
                notes.extend(Self::build_scales_down(
                    MAJOR_SCALE,
                    &args.octaves,
                    &args.offset,
                ));
            }
            Scale::NaturalMinor => {
                notes.extend(Self::build_scales_up(
                    NATURAL_MINOR_SCALE,
                    &args.octaves,
                    &args.offset,
                ));
                notes.push(MidiNote {
                    _stamp: 0,
                    message: vec![0, (12 + (args.octaves - 1) * 12) + (args.offset + 21), 0],
                });
                notes.extend(Self::build_scales_down(
                    NATURAL_MINOR_SCALE,
                    &args.octaves,
                    &args.offset,
                ));
            }
            Scale::HarmonicMinor => {
                notes.extend(Self::build_scales_up(
                    HARMONIC_MINOR_SCALE,
                    &args.octaves,
                    &args.offset,
                ));
                notes.push(MidiNote {
                    _stamp: 0,
                    message: vec![0, (12 + (args.octaves - 1) * 12) + (args.offset + 21), 0],
                });
                notes.extend(Self::build_scales_down(
                    HARMONIC_MINOR_SCALE,
                    &args.octaves,
                    &args.offset,
                ));
            }
            Scale::MelodicMinor => {
                notes.extend(Self::build_scales_up(
                    MELODIC_MINOR_SCALE,
                    &args.octaves,
                    &args.offset,
                ));
                notes.push(MidiNote {
                    _stamp: 0,
                    message: vec![0, (12 + (args.octaves - 1) * 12) + (args.offset + 21), 0],
                });
                notes.extend(Self::build_scales_down(
                    NATURAL_MINOR_SCALE,
                    &args.octaves,
                    &args.offset,
                ));
            }
        }
        notes
    }
}

#[derive()]
struct MidiNote {
    _stamp: u64,
    message: Vec<u8>,
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

const MAJOR_SCALE: &'static [u8] = &[0, 2, 4, 5, 7, 9, 11];
const NATURAL_MINOR_SCALE: &'static [u8] = &[0, 2, 3, 5, 7, 8, 10];
const HARMONIC_MINOR_SCALE: &'static [u8] = &[0, 2, 3, 5, 7, 8, 11];
const MELODIC_MINOR_SCALE: &'static [u8] = &[0, 2, 3, 5, 7, 9, 11];

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
