use vcvrack_rs::audio::{AudioEngine, AudioIO, Oscillator};
use vcvrack_rs::audio::modules::oscillator::OscillatorMessage;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), String> {
    // Create audio engine
    let mut engine = AudioEngine::new(44100.0);
    
    // Create oscillator and get its sender
    let oscillator = Box::new(Oscillator::new());
    let sender = oscillator.get_sender();
    
    // Add oscillator to engine
    let _osc_id = engine.add_processor(oscillator);
    
    // Create audio I/O
    let mut audio_io = AudioIO::new(engine);
    
    // Start audio
    println!("Starting audio");
    audio_io.start()?;
    
    // Play initial A4 in center for 2 seconds
    println!("Playing A4 in center");
    thread::sleep(Duration::from_secs(2));

    // Pan from center to left
    // println!("Panning to left");
    // for i in 0..=50 {
    //     let pan = -i as f32 / 50.0;
    //     sender.send(OscillatorMessage::SetPan(pan))
    //         .map_err(|_| "Failed to send pan change".to_string())?;
    //     thread::sleep(Duration::from_millis(30));
    // }
    // thread::sleep(Duration::from_secs(1));

    // // Pan from left to right
    // println!("Panning to right");
    // for i in 0..=100 {
    //     let pan = -1.0 + (i as f32 / 50.0);
    //     sender.send(OscillatorMessage::SetPan(pan))
    //         .map_err(|_| "Failed to send pan change".to_string())?;
    //     thread::sleep(Duration::from_millis(30));
    // }
    // thread::sleep(Duration::from_secs(1));

    // // Pan back to center
    // println!("Panning to center");
    // for i in 0..=50 {
    //     let pan = 1.0 - (i as f32 / 50.0);
    //     sender.send(OscillatorMessage::SetPan(pan))
    //         .map_err(|_| "Failed to send pan change".to_string())?;
    //     thread::sleep(Duration::from_millis(30));
    // }
    // thread::sleep(Duration::from_secs(1));

    // // Fade out
    // println!("Starting fade out");
    // for i in (0..=50).rev() {
    //     let volume = i as f32 / 50.0 * 0.5;
    //     sender.send(OscillatorMessage::SetVolume(volume))
    //         .map_err(|_| "Failed to send volume change".to_string())?;
    //     thread::sleep(Duration::from_millis(30));
    // }
    
    println!("Stopping audio");
    audio_io.stop();
    
    Ok(())
} 