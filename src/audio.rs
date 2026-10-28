//! Procedural, mono 22,050 Hz arcade effects; unavailable audio is silent.

use rand::{RngExt, SeedableRng, rngs::StdRng};

const SAMPLE_RATE: u32 = 22_050;
const GAIN: f32 = 0.38;
const VOICES: usize = 8;
const EFFECTS: [SoundEffect; 6] = [
    SoundEffect::Shoot,
    SoundEffect::EnemyExplosion,
    SoundEffect::PlayerDeath,
    SoundEffect::StageClear,
    SoundEffect::Capture,
    SoundEffect::Rescue,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum SoundEffect {
    Shoot,
    EnemyExplosion,
    PlayerDeath,
    StageClear,
    Capture,
    Rescue,
}

/// Synthesizes an effect without opening an audio device.
pub fn synthesize(effect: SoundEffect) -> Vec<f32> {
    match effect {
        SoundEffect::Shoot => sweep(820.0, 160.0, 0.09, 0.28),
        SoundEffect::PlayerDeath => sweep(580.0, 55.0, 0.6, 0.35),
        SoundEffect::StageClear => arpeggio(&[261.6, 329.6, 392.0, 523.3], 7, 0.3),
        SoundEffect::Rescue => arpeggio(&[261.6, 392.0, 523.3, 659.3], 9, 0.32),
        SoundEffect::EnemyExplosion => {
            let count = (SAMPLE_RATE as f32 * 0.22) as usize;
            let mut rng = StdRng::seed_from_u64(7);
            let mut phase = 0.0_f64;
            (0..count)
                .map(|i| {
                    let t = i as f32 / count as f32;
                    let amplitude = (1.0 - t) * (1.0 - t) * 0.45;
                    let noise = (rng.random::<f64>() * 2.0 - 1.0) as f32;
                    phase += 80.0 / f64::from(SAMPLE_RATE);
                    let rumble = (phase * std::f64::consts::TAU).sin() as f32 * 0.4;
                    pcm_sample((noise * 0.6 + rumble) * amplitude)
                })
                .collect()
        }
        SoundEffect::Capture => {
            let count = (SAMPLE_RATE as f32 * 0.5) as usize;
            let mut phase = 0.0_f64;
            (0..count)
                .map(|i| {
                    let t = i as f32 / count as f32;
                    let frequency = 420.0 - 280.0 * t + (f64::from(t) * 40.0).sin() as f32 * 45.0;
                    let amplitude = 0.3 * (1.0 - t * 0.5);
                    phase += f64::from(frequency / SAMPLE_RATE as f32);
                    pcm_sample(square_wave(phase) * amplitude)
                })
                .collect()
        }
    }
}

fn sweep(start_hz: f32, end_hz: f32, seconds: f32, amplitude: f32) -> Vec<f32> {
    let count = (SAMPLE_RATE as f32 * seconds) as usize;
    let mut phase = 0.0_f64;
    (0..count)
        .map(|i| {
            let t = i as f32 / count as f32;
            let frequency = start_hz + (end_hz - start_hz) * t;
            phase += f64::from(frequency / SAMPLE_RATE as f32);
            pcm_sample(square_wave(phase) * (amplitude * (1.0 - t * 0.75)))
        })
        .collect()
}

fn arpeggio(frequencies: &[f32], divisor: u32, amplitude: f32) -> Vec<f32> {
    let per_note = (SAMPLE_RATE / divisor) as usize;
    let mut samples = Vec::with_capacity(frequencies.len() * per_note);
    for &frequency in frequencies {
        let mut phase = 0.0_f64;
        for i in 0..per_note {
            let t = i as f32 / per_note as f32;
            phase += f64::from(frequency / SAMPLE_RATE as f32);
            samples.push(pcm_sample(
                square_wave(phase) * (amplitude * (1.0 - t * 0.4)),
            ));
        }
    }
    samples
}

fn square_wave(phase: f64) -> f32 {
    if phase % 1.0 < 0.5 { 1.0 } else { -1.0 }
}

fn pcm_sample(value: f32) -> f32 {
    // Preserve the original Mono16 quantization on both playback backends.
    (value * f32::from(i16::MAX)) as i16 as f32 / f32::from(i16::MAX)
}

pub struct SoundPlayer {
    backend: Option<Backend>,
    next_voice: usize,
}

impl SoundPlayer {
    pub fn new() -> Self {
        Self {
            backend: Backend::new(),
            next_voice: 0,
        }
    }

    pub fn play(&mut self, effect: SoundEffect) {
        #[cfg(target_arch = "wasm32")]
        if self.backend.is_none() {
            self.backend = Backend::new();
        }

        if let Some(backend) = &mut self.backend {
            backend.play(effect, self.next_voice);
            self.next_voice = (self.next_voice + 1) % VOICES;
        }
    }

    pub fn is_available(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.backend.is_some()
        }
        #[cfg(target_arch = "wasm32")]
        {
            web_context()
                .is_some_and(|context| context.state() != web_sys::AudioContextState::Closed)
        }
    }
}

impl Default for SoundPlayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct Backend {
    players: [rodio::Player; VOICES],
    buffers: [rodio::buffer::SamplesBuffer; EFFECTS.len()],
    device: rodio::MixerDeviceSink,
}

#[cfg(not(target_arch = "wasm32"))]
impl Backend {
    fn new() -> Option<Self> {
        let sample_rate = rodio::SampleRate::new(SAMPLE_RATE)?;
        let mut device = rodio::DeviceSinkBuilder::open_default_sink().ok()?;
        device.log_on_drop(false);
        let players = std::array::from_fn(|_| rodio::Player::connect_new(device.mixer()));
        let buffers = EFFECTS.map(|effect| {
            rodio::buffer::SamplesBuffer::new(
                rodio::ChannelCount::MIN,
                sample_rate,
                synthesize(effect),
            )
        });
        Some(Self {
            players,
            buffers,
            device,
        })
    }

    fn play(&mut self, effect: SoundEffect, voice: usize) {
        // Appending to a stopped rodio Player waits for the audio thread to flush.
        // Reconnecting this slot instead keeps the game loop nonblocking.
        self.players[voice].stop();
        let player = rodio::Player::connect_new(self.device.mixer());
        player.set_volume(GAIN);
        player.append(self.buffers[effect as usize].clone());
        self.players[voice] = player;
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    // DOM gestures and the app must share one context: browser autoplay permission
    // is granted synchronously at this boundary, not during a later game frame.
    static WEB_CONTEXT: std::cell::RefCell<Option<web_sys::AudioContext>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn web_context() -> Option<web_sys::AudioContext> {
    WEB_CONTEXT.with(|slot| slot.try_borrow().ok()?.clone())
}

/// Call synchronously from a real keydown/pointerdown handler to unlock Web Audio.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn unlock_audio() -> Result<(), wasm_bindgen::JsValue> {
    let context = WEB_CONTEXT.with(|slot| {
        let mut context = slot
            .try_borrow_mut()
            .map_err(|_| wasm_bindgen::JsValue::from_str("Audio context is busy"))?;
        if context
            .as_ref()
            .is_none_or(|context| context.state() == web_sys::AudioContextState::Closed)
        {
            *context = Some(web_sys::AudioContext::new()?);
        }
        context
            .clone()
            .ok_or_else(|| wasm_bindgen::JsValue::from_str("Audio context is unavailable"))
    })?;
    // Invoke resume before yielding, while the browser's user activation is live.
    let resumed = context.resume()?;
    wasm_bindgen_futures::spawn_local(async move {
        let _ = wasm_bindgen_futures::JsFuture::from(resumed).await;
    });
    Ok(())
}

#[cfg(target_arch = "wasm32")]
struct Backend {
    context: web_sys::AudioContext,
    gain: web_sys::GainNode,
    buffers: [web_sys::AudioBuffer; EFFECTS.len()],
    sources: [Option<web_sys::AudioBufferSourceNode>; VOICES],
}

#[cfg(target_arch = "wasm32")]
impl Backend {
    fn new() -> Option<Self> {
        let context = web_context()?;
        if context.state() != web_sys::AudioContextState::Running {
            return None;
        }
        let buffers: Result<Vec<_>, wasm_bindgen::JsValue> = EFFECTS
            .iter()
            .map(|&effect| {
                let samples = synthesize(effect);
                let buffer = context.create_buffer(1, samples.len() as u32, SAMPLE_RATE as f32)?;
                buffer.copy_to_channel(&samples, 0)?;
                Ok(buffer)
            })
            .collect();
        let buffers = buffers.ok()?.try_into().ok()?;
        let gain = context.create_gain().ok()?;
        gain.gain().set_value(GAIN);
        if gain
            .connect_with_audio_node(&context.destination())
            .is_err()
        {
            let _ = gain.disconnect();
            return None;
        }
        Some(Self {
            context,
            gain,
            buffers,
            sources: std::array::from_fn(|_| None),
        })
    }

    fn play(&mut self, effect: SoundEffect, voice: usize) {
        if let Some(previous) = self.sources[voice].take() {
            stop_web_source(&previous);
        }
        if self.context.state() != web_sys::AudioContextState::Running {
            return;
        }
        let Ok(source) = self.context.create_buffer_source() else {
            return;
        };
        source.set_buffer(Some(&self.buffers[effect as usize]));
        if source.connect_with_audio_node(&self.gain).is_err() || source.start().is_err() {
            let _ = source.disconnect();
            return;
        }
        self.sources[voice] = Some(source);
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(
    deprecated,
    reason = "The equivalent inherited stop requires the extra AudioScheduledSourceNode feature"
)]
fn stop_web_source(source: &web_sys::AudioBufferSourceNode) {
    let _ = source.stop();
    let _ = source.disconnect();
}

#[cfg(target_arch = "wasm32")]
impl Drop for Backend {
    fn drop(&mut self) {
        for source in self.sources.iter_mut().filter_map(Option::take) {
            stop_web_source(&source);
        }
        let _ = self.gain.disconnect();
        // The DOM-owned context lives for the page lifetime, so a replacement app
        // does not lose its gesture permission when this player is dropped.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthesis_preserves_original_effect_lengths() {
        assert_eq!(
            EFFECTS.map(|effect| synthesize(effect).len()),
            [1_984, 4_851, 13_230, 12_600, 11_025, 9_800]
        );
    }

    #[test]
    fn synthesis_produces_finite_bounded_non_silent_samples() {
        for effect in EFFECTS {
            let samples = synthesize(effect);
            assert!(
                samples
                    .iter()
                    .all(|sample| sample.is_finite() && sample.abs() <= 0.45)
                    && samples.iter().any(|sample| sample.abs() > 0.1),
                "{effect:?}"
            );
        }
    }

    #[test]
    fn capture_and_rescue_have_distinct_waveforms_not_just_durations() {
        let capture = synthesize(SoundEffect::Capture);
        let rescue = synthesize(SoundEffect::Rescue);
        let disagreements = capture
            .iter()
            .zip(&rescue)
            .filter(|(left, right)| left.signum() != right.signum())
            .count();
        assert!(disagreements > rescue.len() / 4);
    }

    #[test]
    fn explosion_noise_is_repeatable() {
        assert_eq!(
            synthesize(SoundEffect::EnemyExplosion),
            synthesize(SoundEffect::EnemyExplosion)
        );
    }
}
