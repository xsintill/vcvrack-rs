use std::{io::{self, Write}, thread, time::Duration};
use vcvrack_rs::audio::{AudioEngine, waveform::Waveform};

fn main() -> Result<(), anyhow::Error> {
    let mut engine = AudioEngine::new();
    engine.start()?;

    println!("Interactive Waveform Demo");
    println!("------------------------");
    println!("Press keys to change waveforms:");
    println!("1 - Sine wave");
    println!("2 - Square wave");
    println!("3 - Sawtooth wave");
    println!("4 - Triangle wave");
    println!("q - Quit");
    println!();

    loop {
        print!("Current waveform: Sine. Enter selection: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                println!("Switching to sine wave...");
                engine.set_waveform(Waveform::Sine);
            }
            "2" => {
                println!("Switching to square wave...");
                engine.set_waveform(Waveform::Square);
            }
            "3" => {
                println!("Switching to sawtooth wave...");
                engine.set_waveform(Waveform::Sawtooth);
            }
            "4" => {
                println!("Switching to triangle wave...");
                engine.set_waveform(Waveform::Triangle);
            }
            "q" => {
                println!("Stopping playback...");
                engine.stop();
                break;
            }
            _ => println!("Invalid selection. Please try again."),
        }
    }

    Ok(())
}
