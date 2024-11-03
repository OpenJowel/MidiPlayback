use midir::{MidiOutput, MidiOutputConnection};
use std::{thread, time};
use std::error::Error;


pub mod players
{
  use super::*;

  pub struct MidiPlayer {
    connection: Option<MidiOutputConnection>
  }

  impl MidiPlayer{

    pub fn new() -> Result<Self, Box<dyn Error>> {
      let mut new_midi_player = Self{connection:None};
      
       new_midi_player.connect()?;
       Ok(new_midi_player)
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn Error>>{
      let midi_output = MidiOutput::new("MIDI Output")?;
      let outputs = midi_output.ports();

      if outputs.is_empty() {
        println!("No MIDI outputs available.");
        return Ok(())
      }

      for (i, port) in outputs.iter().enumerate() {
        let name = midi_output.port_name(port)?;
        println!("{}: {}", i, name);
      }

      // Use the first available output
      let port = &outputs[0]; // Get the first MIDI output
      let connection = midi_output.connect(port, "midi-output")?;
      self.connection = Some(connection);

      Ok(())
    }

  
    pub fn play_notes(&mut self) -> Result<(), Box<dyn Error>>{
      if let Some(ref mut connection) = self.connection{

        for n in 0..5 {
          connection.send(&[0x90, 60 + n, 127])?; // Note On
          thread::sleep(time::Duration::from_millis(500)); // Hold the note for 500 ms
        }
        thread::sleep(time::Duration::from_millis(500)); // Hold the note for 500 ms

        for n in 0..5 {
          connection.send(&[0x80, 60 + n, 0])?; // Note Off
        }
      }

        Ok(())
    }
  }
}