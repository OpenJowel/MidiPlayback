use std::error::Error;

mod midi_player;


fn main() -> Result<(), Box<dyn Error>> {

  let mut mp = midi_player::players::MidiPlayer::new()?;

  mp.play_notes(&[0, 4, 7, 12])?;

  Ok(())
}
