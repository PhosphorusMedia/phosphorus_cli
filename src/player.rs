use phosphorus_core::song::Song;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    sync::mpsc::{Receiver, SendError, Sender},
};

pub enum Command {
    Play,
    Pause,
    Quit,
}

pub struct Player {
    _stream: (OutputStream, OutputStreamHandle),
    sink: Sink,
    commands_sender: Sender<Command>,
}

impl Player {
    pub fn try_new() -> Result<Player, Box<dyn Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        let (tx, rx) = std::sync::mpsc::channel();
        Ok(Player {
            _stream: (stream, stream_handle),
            sink,
            commands_sender: tx,
        })
    }

    pub fn initiate(&self, song: &Song) -> Result<(), SendError<Command>> {
        Ok(())
    }

    pub fn append(&self, source: Decoder<BufReader<File>>) -> Result<(), SendError<Command>> {
        self.sink.append(source);
        //self.play()?;
        Ok(())
    }

    pub fn play(&self) -> Result<(), SendError<Command>> {
        self.sink.play();
        //self.commands_sender.send(Command::Play)?;
        Ok(())
    }

    pub fn _pause(&self) -> Result<(), Box<dyn Error>> {
        self.sink.pause();
        //self.commands_sender.send(Command::Pause)?;
        Ok(())
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        let _result = self.commands_sender.send(Command::Quit);
        self.sink.stop();
    }
}
