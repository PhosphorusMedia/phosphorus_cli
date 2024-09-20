use rodio::Decoder;
use std::{error::Error, fs::File, io::BufReader};

use crate::player::Player;

/// Downloads the provided url and plays the associated audio
pub fn download_and_play(player: &Player, file_name: &str) -> Result<(), Box<dyn Error>> {
    // Keep trying opening the file until it opens
    let mut file = File::open(file_name);
    while file.is_err() {
        file = File::open(file_name);
    }

    // Keep trying decoding the file until it decodes
    let reader = BufReader::new(file.unwrap());
    let mut source = Decoder::new_mp3(reader);
    while source.is_err() {
        let file = File::open(file_name);
        let reader = BufReader::new(file.unwrap());
        source = Decoder::new(reader);
    }

    player.append(source.unwrap())?;

    Ok(())
}

/// Downloads the provided url and saves it
pub fn download(file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_name);
    while file.is_err() {
        file = File::open(file_name);
    }

    // Keep trying decoding the file until it decodes
    let reader = BufReader::new(file.unwrap());
    let mut source = Decoder::new_mp3(reader);
    while source.is_err() {
        let file = File::open(file_name);
        let reader = BufReader::new(file.unwrap());
        source = Decoder::new(reader);
    }

    Ok(())
}

// Plays the media found at `path`
pub fn play(player: &Player, path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let source = Decoder::new(reader);

    player.append(source.unwrap())?;

    Ok(())
}
