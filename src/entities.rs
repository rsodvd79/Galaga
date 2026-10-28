use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub const WIDTH: f64 = 800.0;
pub const HEIGHT: f64 = 600.0;

#[derive(Clone, Debug)]
pub struct Body {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub alive: bool,
}

impl Body {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
            alive: true,
        }
    }

    pub fn collides(&self, other: &Self) -> bool {
        self.alive
            && other.alive
            && self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

#[derive(Debug)]
pub struct Player {
    pub body: Body,
    pub lives: i32,
    pub left: bool,
    pub right: bool,
    pub respawning: bool,
    pub respawn_timer: f64,
    pub invulnerability: f64,
    pub dual: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            body: Body::new(384.0, 548.0, 32.0, 32.0),
            lives: 3,
            left: false,
            right: false,
            respawning: false,
            respawn_timer: 0.0,
            invulnerability: 0.0,
            dual: false,
        }
    }
}

impl Player {
    pub fn update(&mut self, dt: f64) {
        if self.respawning {
            self.respawn_timer -= dt;
            if self.respawn_timer <= 0.0 {
                self.respawning = false;
                self.body.alive = true;
                self.invulnerability = 2.0;
                self.body.x = 384.0;
                self.body.y = 548.0;
            }
            return;
        }
        if !self.body.alive {
            return;
        }
        self.invulnerability = (self.invulnerability - dt).max(0.0);
        let direction = i32::from(self.right) - i32::from(self.left);
        self.body.x =
            (self.body.x + f64::from(direction) * 280.0 * dt).clamp(0.0, WIDTH - self.body.width);
    }

    pub fn die(&mut self) {
        self.lives -= 1;
        self.body.alive = false;
        self.invulnerability = 0.0;
        self.dual = false;
        if self.lives > 0 {
            self.respawning = true;
            self.respawn_timer = 2.0;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyType {
    Bee,
    Butterfly,
    Boss,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyState {
    Entry,
    Formation,
    Diving,
    Returning,
}

#[derive(Debug)]
pub struct Enemy {
    pub body: Body,
    pub kind: EnemyType,
    pub state: EnemyState,
    pub captured: bool,
    pub formation_x: f64,
    pub formation_y: f64,
    pub entry_delay: f64,
    dive_angle: f64,
    loop_progress: f64,
}

impl Enemy {
    pub fn new(kind: EnemyType, x: f64, y: f64, delay: f64) -> Self {
        Self {
            body: Body::new(x, -50.0, 24.0, 24.0),
            kind,
            state: EnemyState::Entry,
            captured: false,
            formation_x: x,
            formation_y: y,
            entry_delay: delay,
            dive_angle: 0.0,
            loop_progress: 0.0,
        }
    }

    pub fn points(&self) -> i32 {
        match (self.kind, self.state == EnemyState::Diving) {
            (EnemyType::Bee, false) => 50,
            (EnemyType::Bee, true) => 100,
            (EnemyType::Butterfly, false) => 80,
            (EnemyType::Butterfly, true) => 160,
            (EnemyType::Boss, false) => 150,
            (EnemyType::Boss, true) => 400,
        }
    }

    pub fn start_dive(&mut self, x: f64, y: f64) {
        if self.state != EnemyState::Formation {
            return;
        }
        self.state = EnemyState::Diving;
        self.dive_angle = (y - self.body.y).atan2(x - self.body.x);
        self.loop_progress = 0.0;
    }

    pub fn update(&mut self, dt: f64, offset: f64, multiplier: f64) {
        match self.state {
            EnemyState::Formation => {
                self.body.x = self.formation_x + offset;
                self.body.y = self.formation_y;
            }
            EnemyState::Diving => {
                self.loop_progress += dt;
                let speed = if self.kind == EnemyType::Boss {
                    160.0
                } else {
                    200.0
                };
                self.body.x += self.dive_angle.cos() * speed * multiplier * dt;
                self.body.y += self.dive_angle.sin() * speed * multiplier * dt;
                let diff = (FRAC_PI_2 - self.dive_angle + PI).rem_euclid(TAU) - PI;
                self.dive_angle += diff * dt * 2.0 * multiplier;
                if self.body.y > HEIGHT + 50.0 {
                    self.body.y = -50.0;
                    self.body.x = (self.formation_x + self.loop_progress.sin() * 80.0)
                        .clamp(0.0, WIDTH - self.body.width);
                    self.state = EnemyState::Returning;
                }
            }
            EnemyState::Entry | EnemyState::Returning => {
                if self.state == EnemyState::Entry && self.entry_delay > 0.0 {
                    self.entry_delay -= dt;
                    return;
                }
                let dx = self.formation_x + offset - self.body.x;
                let dy = self.formation_y - self.body.y;
                let distance = dx.hypot(dy);
                if distance < 4.0 {
                    self.body.x = self.formation_x + offset;
                    self.body.y = self.formation_y;
                    self.state = EnemyState::Formation;
                } else {
                    let speed = if self.state == EnemyState::Entry {
                        220.0
                    } else {
                        180.0
                    };
                    self.body.x += dx / distance * speed * multiplier * dt;
                    self.body.y += dy / distance * speed * multiplier * dt;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Formation {
    pub enemies: Vec<Enemy>,
    pub offset: f64,
    direction: f64,
}

impl Default for Formation {
    fn default() -> Self {
        Self {
            enemies: Vec::new(),
            offset: 0.0,
            direction: 1.0,
        }
    }
}

impl Formation {
    pub fn initialize(&mut self) {
        self.enemies.clear();
        for row in 0..5 {
            for col in 0..8 {
                let kind = if row == 0 && (col == 3 || col == 4) {
                    EnemyType::Boss
                } else if row <= 1 {
                    EnemyType::Butterfly
                } else {
                    EnemyType::Bee
                };
                self.enemies.push(Enemy::new(
                    kind,
                    180.0 + f64::from(col) * 56.0,
                    100.0 + f64::from(row) * 48.0,
                    f64::from(row * 8 + col) * 0.1,
                ));
            }
        }
    }

    pub fn update(&mut self, dt: f64, multiplier: f64) {
        self.offset += 28.0 * multiplier * self.direction * dt;
        if self.offset.abs() >= 40.0 {
            self.direction *= -1.0;
        }
        for enemy in &mut self.enemies {
            enemy.update(dt, self.offset, multiplier);
        }
    }

    pub fn alive_count(&self) -> usize {
        self.enemies.iter().filter(|e| e.body.alive).count()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BulletOwner {
    Player,
    Enemy,
}

#[derive(Debug)]
pub struct Bullet {
    pub body: Body,
    pub owner: BulletOwner,
}

impl Bullet {
    pub fn new(center_x: f64, y: f64, owner: BulletOwner) -> Self {
        Self {
            body: Body::new(center_x - 2.0, y, 4.0, 12.0),
            owner,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.body.y += if self.owner == BulletOwner::Player {
            -500.0
        } else {
            260.0
        } * dt;
    }
}

#[derive(Debug)]
pub struct Explosion {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub timer: f64,
}

impl Explosion {
    pub fn new(body: &Body, radius: f64) -> Self {
        Self {
            x: body.x + body.width / 2.0,
            y: body.y + body.height / 2.0,
            radius,
            timer: 0.5,
        }
    }

    pub fn progress(&self) -> f64 {
        1.0 - self.timer / 0.5
    }
}
