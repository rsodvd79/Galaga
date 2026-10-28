use eframe::egui::{self, Color32, Event, Key, ViewportCommand};

use crate::{
    audio::SoundPlayer,
    game::{Clock, Game, Phase, STEP},
    render::{self, Visuals},
    screenshots::{self, Screenshots},
    storage,
};

pub struct GalagaApp {
    pub game: Game,
    visuals: Visuals,
    sound: SoundPlayer,
    clock: Clock,
    last_time: Option<f64>,
    last_frame: Option<u64>,
    focused: bool,
    screenshots: Screenshots,
    auto_screenshots: bool,
    auto_stage: u8,
    shot_requested_at: Option<f64>,
    saved_score: i32,
    error: Option<String>,
}

impl GalagaApp {
    pub fn new(cc: &eframe::CreationContext<'_>, auto_screenshots: bool, seed: u64) -> Self {
        let mut game = Game::new(seed);
        game.high_score = storage::load(cc.storage);
        Self {
            saved_score: game.high_score,
            game,
            visuals: Visuals::new(seed),
            sound: SoundPlayer::new(),
            clock: Clock::default(),
            last_time: None,
            last_frame: None,
            focused: false,
            screenshots: Screenshots::default(),
            auto_screenshots,
            auto_stage: 0,
            shot_requested_at: None,
            error: None,
        }
    }

    fn input(&mut self, ctx: &egui::Context) -> bool {
        ctx.input(|i| {
            self.game.player.left = i.focused && (i.key_down(Key::ArrowLeft) || i.key_down(Key::A));
            self.game.player.right =
                i.focused && (i.key_down(Key::ArrowRight) || i.key_down(Key::D));
        });
        let mut capture = false;
        for event in ctx.input(|i| i.events.clone()) {
            if let Event::Key {
                key,
                pressed: true,
                repeat,
                ..
            } = event
            {
                match key {
                    Key::Space
                        if matches!(self.game.phase, Phase::Menu | Phase::GameOver) && !repeat =>
                    {
                        self.game.reset()
                    }
                    Key::Space if self.game.phase == Phase::Playing => {
                        self.game.shoot_pressed = true
                    }
                    Key::P if !repeat => {
                        self.game.phase = match self.game.phase {
                            Phase::Playing => Phase::Paused,
                            Phase::Paused => Phase::Playing,
                            phase => phase,
                        };
                        self.game.shoot_pressed = false;
                    }
                    Key::Escape if !repeat => {
                        self.game.phase = Phase::Menu;
                        self.game.shoot_pressed = false;
                    }
                    Key::F12 if !repeat => capture = true,
                    _ => {}
                }
            }
        }
        capture || screenshots::web_requested()
    }

    fn advance(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);
        if let Some(result) = self.screenshots.finish() {
            self.shot_requested_at = None;
            match result {
                Ok(()) if self.auto_screenshots && self.auto_stage == 1 => {
                    self.auto_stage = 2;
                    self.game.reset();
                }
                Ok(()) if self.auto_screenshots && self.auto_stage == 3 => {
                    self.auto_stage = 4;
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
                Err(error) => {
                    eprintln!("Screenshot failed: {error}");
                    self.error = Some(error);
                    if self.auto_screenshots {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                }
                _ => {}
            }
        }
        if self.shot_requested_at.is_some_and(|t| now - t > 10.0) {
            self.error = Some("Screenshot capture timed out".into());
            self.screenshots = Screenshots::default();
            self.shot_requested_at = None;
            eprintln!("Screenshot capture timed out");
            if self.auto_screenshots {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }
        let focused = self.auto_screenshots || ctx.input(|i| i.focused);
        let dt = if self.focused == focused && focused {
            self.last_time.map_or(0.0, |t| now - t)
        } else {
            self.clock.clear();
            self.game.shoot_pressed = false;
            0.0
        };
        self.focused = focused;
        self.last_time = Some(now);
        let capture = !self.screenshots.pending && self.input(ctx);
        if !self.screenshots.pending && focused {
            for _ in 0..self.clock.steps(dt) {
                self.visuals.tick(STEP);
                self.game.tick(STEP);
                self.game.update_stage_clear(STEP);
            }
        } else {
            self.clock.clear();
        }
        for sound in self.game.sounds.drain(..) {
            self.sound.play(sound);
        }
        if self.game.high_score > self.saved_score {
            storage::save(frame.storage_mut(), self.game.high_score);
            self.saved_score = self.game.high_score;
        }
        let auto_capture = self.auto_screenshots
            && match self.auto_stage {
                0 => self.visuals.ticks >= 120,
                2 => self.visuals.ticks >= 300,
                _ => false,
            };
        if !self.screenshots.pending && (capture || auto_capture) {
            if auto_capture {
                self.auto_stage += 1;
            }
            self.screenshots.request(ctx, self.game.phase.name());
            self.shot_requested_at = Some(now);
        }
    }
}

impl eframe::App for GalagaApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let frame_nr = ctx.cumulative_frame_nr();
        if self.last_frame != Some(frame_nr) {
            self.last_frame = Some(frame_nr);
            self.advance(&ctx, frame);
        }
        let available = ui.max_rect();
        ui.painter().rect_filled(available, 0.0, Color32::BLACK);
        self.visuals
            .draw(ui.painter(), render::viewport(available), &self.game);
        if let Some(error) = &self.error {
            ui.painter().text(
                available.left_bottom() + egui::vec2(8.0, -8.0),
                egui::Align2::LEFT_BOTTOM,
                error,
                egui::FontId::proportional(12.0),
                Color32::LIGHT_RED,
            );
        }
        ctx.request_repaint();
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage::save(Some(storage), self.game.high_score);
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }
}
