use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::{audio::SoundEffect, entities::*};

pub const STEP: f64 = 0.016;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Menu,
    Playing,
    Paused,
    StageClear,
    GameOver,
}

impl Phase {
    pub fn name(self) -> &'static str {
        match self {
            Self::Menu => "menu",
            Self::Playing => "playing",
            Self::Paused => "paused",
            Self::StageClear => "stageclear",
            Self::GameOver => "gameover",
        }
    }
}

pub struct Game {
    pub phase: Phase,
    pub score: i32,
    pub high_score: i32,
    pub level: i32,
    pub player: Player,
    pub formation: Formation,
    pub bullets: Vec<Bullet>,
    pub explosions: Vec<Explosion>,
    pub sounds: Vec<SoundEffect>,
    pub shoot_pressed: bool,
    pub enemy_shoot_timer: f64,
    pub stage_clear_timer: f64,
    rng: StdRng,
}

impl Game {
    pub fn new(seed: u64) -> Self {
        Self {
            phase: Phase::Menu,
            score: 0,
            high_score: 0,
            level: 1,
            player: Player::default(),
            formation: Formation::default(),
            bullets: Vec::new(),
            explosions: Vec::new(),
            sounds: Vec::new(),
            shoot_pressed: false,
            enemy_shoot_timer: 0.0,
            stage_clear_timer: 0.0,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn reset(&mut self) {
        self.phase = Phase::Playing;
        self.score = 0;
        self.level = 1;
        self.player = Player::default();
        self.bullets.clear();
        self.explosions.clear();
        self.sounds.clear();
        self.shoot_pressed = false;
        self.formation.initialize();
        self.enemy_shoot_timer = 3.0;
    }

    pub fn speed_multiplier(&self) -> f64 {
        (1.0 + f64::from(self.level - 1) * 0.08).min(2.0)
    }

    pub fn next_level(&mut self) {
        self.level += 1;
        self.phase = Phase::Playing;
        self.bullets.clear();
        self.explosions.clear();
        self.player.body.alive = true;
        self.player.respawning = false;
        self.player.respawn_timer = 0.0;
        self.player.body.x = 384.0;
        self.player.body.y = 548.0;
        self.formation.initialize();
        self.enemy_shoot_timer = (3.0 - f64::from(self.level) * 0.2).max(1.0);
    }

    pub fn update_stage_clear(&mut self, dt: f64) {
        if self.phase != Phase::StageClear {
            return;
        }
        self.stage_clear_timer -= dt;
        if self.stage_clear_timer <= 0.0 {
            self.next_level();
        }
    }

    pub fn tick(&mut self, dt: f64) {
        if self.phase != Phase::Playing {
            return;
        }
        self.player.update(dt);
        self.formation.update(dt, self.speed_multiplier());
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
        self.bullets
            .retain(|b| b.body.y >= -b.body.height && b.body.y <= HEIGHT + 20.0);
        for explosion in &mut self.explosions {
            explosion.timer -= dt;
        }
        self.explosions.retain(|e| e.timer > 0.0);
        self.player_shoot();
        self.enemy_shoot(dt);
        self.trigger_dive();
        self.bullet_enemy_collisions();
        self.enemy_player_collisions();
        self.bullet_player_collisions();
        if self.phase == Phase::Playing && self.formation.alive_count() == 0 {
            self.phase = Phase::StageClear;
            self.stage_clear_timer = 2.5;
            self.sounds.push(SoundEffect::StageClear);
        }
    }

    fn player_shoot(&mut self) {
        if !std::mem::take(&mut self.shoot_pressed) {
            return;
        }
        if !self.player.body.alive || self.player.respawning {
            return;
        }
        let count = self
            .bullets
            .iter()
            .filter(|b| b.owner == BulletOwner::Player)
            .count();
        let shots = if self.player.dual { 2 } else { 1 };
        if count + shots > shots * 2 {
            return;
        }
        let p = &self.player.body;
        if self.player.dual {
            self.bullets
                .push(Bullet::new(p.x + 4.0, p.y, BulletOwner::Player));
            self.bullets
                .push(Bullet::new(p.x + p.width - 4.0, p.y, BulletOwner::Player));
        } else {
            self.bullets
                .push(Bullet::new(p.x + p.width / 2.0, p.y, BulletOwner::Player));
        }
        self.sounds.push(SoundEffect::Shoot);
    }

    fn enemy_shoot(&mut self, dt: f64) {
        self.enemy_shoot_timer -= dt;
        if self.enemy_shoot_timer > 0.0 {
            return;
        }
        let candidates: Vec<_> = self
            .formation
            .enemies
            .iter()
            .filter(|e| {
                e.body.alive && matches!(e.state, EnemyState::Formation | EnemyState::Diving)
            })
            .collect();
        if !candidates.is_empty() {
            let e = candidates[self.rng.random_range(0..candidates.len())];
            self.bullets.push(Bullet::new(
                e.body.x + e.body.width / 2.0,
                e.body.y + e.body.height,
                BulletOwner::Enemy,
            ));
        }
        self.enemy_shoot_timer = self.rng.random::<f64>() * 1.5 + 0.5;
    }

    fn trigger_dive(&mut self) {
        let divers = self
            .formation
            .enemies
            .iter()
            .filter(|e| e.body.alive && e.state == EnemyState::Diving)
            .count();
        if divers >= 2 || self.rng.random::<f64>() > 0.008 {
            return;
        }
        let candidates: Vec<_> = self
            .formation
            .enemies
            .iter()
            .enumerate()
            .filter(|(_, e)| e.body.alive && e.state == EnemyState::Formation)
            .map(|(i, _)| i)
            .collect();
        if candidates.is_empty() {
            return;
        }
        let index = candidates[self.rng.random_range(0..candidates.len())];
        let p = &self.player.body;
        self.formation.enemies[index].start_dive(p.x + p.width / 2.0, p.y + p.height / 2.0);
    }

    fn bullet_enemy_collisions(&mut self) {
        for bullet in &mut self.bullets {
            if bullet.owner != BulletOwner::Player || !bullet.body.alive {
                continue;
            }
            for enemy in &mut self.formation.enemies {
                if !bullet.body.collides(&enemy.body) {
                    continue;
                }
                bullet.body.alive = false;
                enemy.body.alive = false;
                let mut points = enemy.points();
                if enemy.captured && self.player.lives > 0 {
                    self.player.dual = true;
                    points += 1000;
                    self.sounds.push(SoundEffect::Rescue);
                }
                self.score += points;
                self.high_score = self.high_score.max(self.score);
                self.explosions.push(Explosion::new(&enemy.body, 20.0));
                self.sounds.push(SoundEffect::EnemyExplosion);
                break;
            }
        }
        self.bullets.retain(|b| b.body.alive);
    }

    fn enemy_player_collisions(&mut self) {
        if !self.player.body.alive || self.player.invulnerability > 0.0 {
            return;
        }
        for enemy in &mut self.formation.enemies {
            if enemy.state != EnemyState::Diving || !enemy.body.collides(&self.player.body) {
                continue;
            }
            let capture = enemy.kind == EnemyType::Boss && !enemy.captured;
            if capture {
                enemy.captured = true;
            } else {
                enemy.body.alive = false;
            }
            self.player.die();
            self.explosions.push(Explosion::new(
                &self.player.body,
                if capture { 24.0 } else { 28.0 },
            ));
            self.sounds.push(if capture {
                SoundEffect::Capture
            } else {
                SoundEffect::PlayerDeath
            });
            if self.player.lives <= 0 {
                self.phase = Phase::GameOver;
            }
            break;
        }
    }

    fn bullet_player_collisions(&mut self) {
        if !self.player.body.alive || self.player.invulnerability > 0.0 {
            return;
        }
        for bullet in &mut self.bullets {
            if bullet.owner != BulletOwner::Enemy || !bullet.body.collides(&self.player.body) {
                continue;
            }
            bullet.body.alive = false;
            self.player.die();
            self.explosions
                .push(Explosion::new(&self.player.body, 28.0));
            self.sounds.push(SoundEffect::PlayerDeath);
            if self.player.lives <= 0 {
                self.phase = Phase::GameOver;
            }
            break;
        }
        self.bullets.retain(|b| b.body.alive);
    }
}

#[derive(Default)]
pub struct Clock {
    remainder: f64,
}

impl Clock {
    pub fn steps(&mut self, elapsed: f64) -> usize {
        if !elapsed.is_finite() || elapsed < 0.0 {
            return 0;
        }
        self.remainder += elapsed.min(0.05);
        let count = ((self.remainder + 1e-10) / STEP).floor() as usize;
        self.remainder = (self.remainder - count as f64 * STEP).max(0.0);
        count
    }

    pub fn clear(&mut self) {
        self.remainder = 0.0;
    }
}
