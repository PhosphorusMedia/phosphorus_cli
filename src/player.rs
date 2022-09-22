use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    sync::mpsc::{Receiver, SendError, Sender},
    thread,
};

#[allow(dead_code)]
pub enum Command {
    Play,
    Pause,
    Quit,
}

#[allow(dead_code)]
pub struct Player {
    stream: (OutputStream, OutputStreamHandle),
    sink: Sink,
    tx: Sender<Command>,
    displayer: thread::JoinHandle<()>,
}

impl Player {
    pub fn try_new() -> Result<Player, Box<dyn Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        let (tx, rx) = std::sync::mpsc::channel();
        let displayer = thread::spawn(move || display(rx));
        Ok(Player {
            stream: (stream, stream_handle),
            sink,
            tx,
            displayer,
        })
    }

    pub fn append(&self, source: Decoder<BufReader<File>>) -> Result<(), SendError<Command>> {
        self.sink.append(source);
        self.play()?;
        Ok(())
    }

    pub fn play(&self) -> Result<(), SendError<Command>> {
        self.sink.play();
        self.tx.send(Command::Play)?;
        Ok(())
    }

    pub fn _pause(&self) -> Result<(), Box<dyn Error>> {
        self.sink.pause();
        self.tx.send(Command::Pause)?;
        Ok(())
    }
}

fn display(rx: Receiver<Command>) {
    loop {
        let command = match rx.recv() {
            Ok(command) => command,
            Err(_) => break,
        };

        match command {
            Command::Play => {
                println!("Play")
            }
            Command::Pause => {
                println!("Pause")
            }
            Command::Quit => {
                break;
            }
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        let _result = self.tx.send(Command::Quit);
        self.sink.stop();
    }
}
