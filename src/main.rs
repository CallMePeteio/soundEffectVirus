#![allow(non_snake_case)]

use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::seq::SliceRandom;
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::thread;

fn play_audio(file_path: &str, volume: f32, stream_handle: Arc<rodio::OutputStreamHandle>) {
    let file_path = file_path.to_string();
    let stream_handle = Arc::clone(&stream_handle);

    thread::spawn(move || {
        let file = File::open(&file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();

        println!("{volume:?}");
        sink.set_volume(volume);
        sink.append(source);
        sink.sleep_until_end();
    });
}

// WAITS UNTILL THE USER HAS RELEASED THE MOUSE BUTTON
fn waitUntillReleasedBTNMouse(index: i8) {
    let device_state = DeviceState::new();
    loop {
        let mouse_keys = device_state.get_mouse();
        match index {
            0 => {
                if mouse_keys.button_pressed[0] == false {
                    break;
                }
            }
            1 => {
                if mouse_keys.button_pressed[1] == false {
                    break;
                }
            }
            2 => {
                if mouse_keys.button_pressed[2] == false {
                    break;
                }
            }
            3 => {
                if mouse_keys.button_pressed[3] == false {
                    break;
                }
            }
            _ => println!("SERIOUS ERROR IN waitUntillReleasedBTN"),
        }
    }
}

fn waitUntillReleasedBTNKeyBoard(input: &str) {
    let device_state = DeviceState::new();

    loop {
        let keys = device_state.get_keys();
        match input {
            "space" => {
                if keys.contains(&Keycode::Space) == false {
                    break;
                }
            }
            "backspace" => {
                if keys.contains(&Keycode::Backspace) == false {
                    break;
                }
            }
            "a" => {
                if keys.contains(&Keycode::A) == false {
                    break;
                }
            }
            "c" => {
                if keys.contains(&Keycode::C) == false {
                    break;
                }
            }
            "v" => {
                if keys.contains(&Keycode::V) == false {
                    break;
                }
            }
            "e" => {
                if keys.contains(&Keycode::E) == false {
                    break;
                }
            }
            _ => println!("SERIOUS ERROR IN waitUntillReleasedBTNKeyBoard"),
        }
    }
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let stream_handle = Arc::new(stream_handle);
    let device_state = DeviceState::new();

    let mut rng = rand::thread_rng();

    let audioAmplifier = 0.4;
    let audioAmplifier = 0.3;
    let audioClipsGuns = vec![
        ("guns/ak47.mp3", 0.5 * audioAmplifier),
        ("guns/awp.mp3", 0.8 * audioAmplifier),
        ("guns/m4.mp3", 0.8 * audioAmplifier),
        ("guns/dessertEagle.mp3", 9.0 * audioAmplifier),
        ("guns/pistol.mp3", 2.0 * audioAmplifier),
        ("guns/sniper.mp3", 1.3 * audioAmplifier),
    ];
    let aduioClipsMisc = vec![
        ("misc/tackticalNuke.mp3", 1.5 * audioAmplifier),
        ("misc/terroristWin.mp3", 2.0 * audioAmplifier),
        ("misc/defusedBomb.mp3", 2.0 * audioAmplifier),
        ("misc/ctWins.mp3", 2.0 * audioAmplifier),
        ("misc/dangerAlarm.mp3", 1.5 * audioAmplifier),
        ("misc/militaryAlarm.mp3", 2.0 * audioAmplifier),
        ("misc/c4Placed.mp3", 1.5 * audioAmplifier),
        ("misc/missileLaunch.mp3", 2.0 * audioAmplifier),
        ("misc/fighterJet.mp3", 1.8 * audioAmplifier),
        ("misc/tank.mp3", 1.5 * audioAmplifier),
        ("misc/tankShot.mp3", 1.5 * audioAmplifier),
        ("misc/alarm.mp3", 1.5 * audioAmplifier),
        ("misc/airRaidsiren.mp3", 2.5 * audioAmplifier),
        ("misc/fire.mp3", 1.5 * audioAmplifier),
        ("misc/tackticalNuke.mp3", 1.5 * audioAmplifier),
        (("misc/c4Placed.mp3", 1.5 * audioAmplifier)),
    ];
    let (mut gunClip, mut gunVvolume) = audioClipsGuns.choose(&mut rng).unwrap();

    loop {
        let mouse_keys = device_state.get_mouse();
        let keys = device_state.get_keys();

        if mouse_keys.button_pressed[1] == true {
            // IF RIGHT CLICK IS CLICKED
            play_audio(gunClip, gunVvolume, Arc::clone(&stream_handle));
            waitUntillReleasedBTNMouse(1);
        }
        if mouse_keys.button_pressed[2] {
            // IF LEFT CLICK IS CLICKED

            let (newClip, newVolume) = audioClipsGuns.choose(&mut rng).unwrap();
            gunClip = newClip;
            gunVvolume = *newVolume;

            for i in 0..15 {
                play_audio(gunClip, gunVvolume, Arc::clone(&stream_handle));
                thread::sleep(std::time::Duration::from_millis(i * 6));
            }
            thread::sleep(std::time::Duration::from_millis(30));
            //waitUntillReleasedBTNMouse(2);
        }
        if mouse_keys.button_pressed[3] == true
            || keys.contains(&Keycode::Space) == true
            || keys.contains(&Keycode::Backspace) == true
            || keys.contains(&Keycode::A) == true
            || keys.contains(&Keycode::C) == true
            || keys.contains(&Keycode::V) == true
            || keys.contains(&Keycode::E) == true
        {
            // IF MIDDLE MOUSE BUTTONS IS PRESSED OR SPACE
            let (miscClip, miscVolume) = aduioClipsMisc.choose(&mut rng).unwrap();
            play_audio(miscClip, *miscVolume, Arc::clone(&stream_handle));
            waitUntillReleasedBTNMouse(3);
            waitUntillReleasedBTNKeyBoard("space");
            waitUntillReleasedBTNKeyBoard("backspace");
            waitUntillReleasedBTNKeyBoard("a");
            waitUntillReleasedBTNKeyBoard("c");
            waitUntillReleasedBTNKeyBoard("v");
            waitUntillReleasedBTNKeyBoard("e");
        }

        thread::sleep(std::time::Duration::from_millis(10));
    }
}
