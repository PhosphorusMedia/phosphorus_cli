use phosphorus_core::song::Song;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{error::Error, fs::File, io::BufReader, sync::mpsc::Sender};

use crate::ui::UserEvent;

enum Command {
    Append(Song),
    Play,
    Pause,
    Clear,
    Stop
}

pub struct Player {
    _stream: (OutputStream, OutputStreamHandle),
    internal_tx: Sender<Command>,
}

impl Player {
    pub fn try_new(user_event_tx: Sender<UserEvent>) -> Result<Player, Box<dyn Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let (internal_tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            loop {
                let msg = rx.recv();
                if let Err(_) = msg {
                    break;
                }
                match msg.unwrap() {
                    Command::Append(song) => {
                        let file = File::open(song.path()).expect("Error opening song file");
                        let reader = BufReader::new(file);
                        let source = Decoder::new(reader).expect("Error creating the decoder");
                        sink.append(source);
                    },
                    Command::Play => {
                        sink.play();
                        sink.sleep_until_end();
                        let _ = user_event_tx.send(UserEvent::PlayNext);
                    },
                    Command::Pause => sink.pause(),
                    Command::Clear => sink.clear(),
                    Command::Stop => break
                }
            }
            sink.stop();
        });
        Ok(Player {
            _stream: (stream, stream_handle),
            internal_tx,
        })
    }

    /// Prepares the player to start playing `song`. All other media in the queue
    /// are cleared.
    pub fn initiate(&self, song: Song) {
        let _ = self.internal_tx.send(Command::Clear);
        let _ = self.internal_tx.send(Command::Append(song));
        let _ = self.internal_tx.send(Command::Play);
    }

    pub fn clear(&self) {
        let _ = self.internal_tx.send(Command::Clear);
    }

    pub fn append(&self, song: Song) {
        let _ = self.internal_tx.send(Command::Append(song));
    }

    pub fn play(&self) {
        let _ = self.internal_tx.send(Command::Play);
    }

    pub fn pause(&self) {
        let _ = self.internal_tx.send(Command::Pause);
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        let _ = self.internal_tx.send(Command::Stop);
    }
}
