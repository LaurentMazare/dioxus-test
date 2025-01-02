use dioxus::prelude::*;
use oboe::{
    AudioOutputCallback, AudioOutputStreamSafe, AudioStream, AudioStreamBuilder,
    DataCallbackResult, Mono, PerformanceMode, SharingMode,
};

const PI: f32 = 3.14159265358979;

// Structure for sound generator
pub struct SineWave {
    frequency: f32,
    gain: f32,
    phase: f32,
    delta: Option<f32>,
}

// Default constructor for sound generator
impl Default for SineWave {
    fn default() -> Self {
        Self {
            frequency: 440.0,
            gain: 0.5,
            phase: 0.0,
            delta: None,
        }
    }
}

// Audio output callback trait implementation
impl AudioOutputCallback for SineWave {
    // Define type for frames which we would like to process
    type FrameType = (f32, Mono);

    // Implement sound data output callback
    fn on_audio_ready(
        &mut self,
        stream: &mut dyn AudioOutputStreamSafe,
        frames: &mut [f32],
    ) -> DataCallbackResult {
        // Configure out wave generator
        if self.delta.is_none() {
            let sample_rate = stream.get_sample_rate() as f32;
            self.delta = (self.frequency * 2.0 * PI / sample_rate).into();
            println!(
                "Prepare sine wave generator: samplerate={}, time delta={}",
                sample_rate,
                self.delta.unwrap()
            );
        }

        let delta = self.delta.unwrap();

        // Generate audio frames to fill the output buffer
        for frame in frames {
            *frame = self.gain * self.phase.sin();
            self.phase += delta;
            while self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }
        }

        // Notify the oboe that stream is continued
        DataCallbackResult::Continue
    }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    let mut sine = AudioStreamBuilder::default()
        // select desired performance mode
        .set_performance_mode(PerformanceMode::LowLatency)
        // select desired sharing mode
        .set_sharing_mode(SharingMode::Shared)
        // select sound sample format
        .set_format::<f32>()
        // select channels configuration
        .set_channel_count::<Mono>()
        // set our generator as callback
        .set_callback(SineWave::default())
        // open the output stream
        .open_stream()
        .unwrap();

    // Start playback
    sine.start().unwrap();

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
