use std::{thread, time::Duration};
use vcvrack_rs::audio::{AudioEngine, waveform::Waveform};

fn main() -> Result<(), anyhow::Error> {
    let mut engine = AudioEngine::new();
    
    // Start with sine wave
    println!("Playing sine wave...");
    engine.start()?;
    thread::sleep(Duration::from_secs(2));

    // Switch to square wave
    println!("Switching to square wave...");
    engine.set_waveform(Waveform::Square);
    thread::sleep(Duration::from_secs(2));

    // Switch to sawtooth wave
    println!("Switching to sawtooth wave...");
    engine.set_waveform(Waveform::Sawtooth);
    thread::sleep(Duration::from_secs(2));

    // Switch to triangle wave
    println!("Switching to triangle wave...");
    engine.set_waveform(Waveform::Triangle);
    thread::sleep(Duration::from_secs(2));

    // Stop the engine
    engine.stop();
    println!("Done!");

    Ok(())
}
