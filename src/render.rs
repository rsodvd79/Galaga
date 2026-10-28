use eframe::egui::{Align2, Color32 as C, FontId, Mesh, Painter, Pos2, Rect, Shape, Stroke, Vec2};
use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::{
    entities::*,
    game::{Game, Phase},
};

const BEE: [&[&str]; 2] = [
    &[
        ".a....a.", ".#====#.", "##====##", "#=ebbe=#", "###bb###", ".##==##.", ".#=##=#.",
        "..#..#..",
    ],
    &[
        "a......a", "#=....=#", "##====##", ".=ebbe=.", ".##bb##.", ".#=##=#.", "..####..",
        "........",
    ],
];
const BUTTERFLY: [&[&str]; 2] = [
    &[
        "o.hddh.o", "#hdwwdh#", "###ww###", ".=bwwb=.", ".=bwwb=.", "###ww###", "#hdwwdh#",
        "o.hddh.o",
    ],
    &[
        "...aa...", ".h####h.", "##dwwd##", "..bwwb..", "..bwwb..", "##dwwd##", ".h####h.",
        "........",
    ],
];
const BOSS: [&[&str]; 2] = [
    &[
        "...##...", "..#mm#..", ".##yy##.", "##h##h##", "#m####m#", "##o##o##", ".#mmmm#.",
        "..####..",
    ],
    &[
        "...##...", ".##mm##.", "##hyyh##", "#h####h#", "#m####m#", "##o##o##", ".#mmmm#.",
        "..####..",
    ],
];
const PLAYER: [&[&str]; 2] = [
    &[
        "................",
        ".......cc.......",
        ".......cc.......",
        "......cccc......",
        ".....##cc##.....",
        "....###==###....",
        "...####==####...",
        "...####==####...",
        "..#####==#####..",
        "..##gge==egg##..",
        ".=#o=ge==eg=o=#.",
        ".=#ooe====eoo=#.",
        ".=#ooe====eoo=#.",
        "..=ooe=ff=eoo=..",
        "..=oof=ww=foo=..",
        "...fww=ww=wwf...",
    ],
    &[
        "................",
        ".......cc.......",
        ".......cc.......",
        "......cccc......",
        ".....##cc##.....",
        "....###==###....",
        "...####==####...",
        "...####==####...",
        "..#####==#####..",
        "..##gge==egg##..",
        ".=#o=ge==eg=o=#.",
        ".=#ooe====eoo=#.",
        ".=#ooe====eoo=#.",
        "..=ooe=ff=eoo=..",
        "..=oof=ff=foo=..",
        "...fff=ff=fff...",
    ],
];
const MINI: &[&str] = &[
    "...cc...", "..cccc..", ".##cc##.", "###==###", "##geegg#", ".=oeo=..",
];

fn rgb(r: u8, g: u8, b: u8) -> C {
    C::from_rgb(r, g, b)
}

fn player_color(c: u8, frame: usize) -> C {
    match c {
        b'#' => rgb(50, 210, 90),
        b'=' => rgb(30, 160, 60),
        b'o' => rgb(20, 100, 40),
        b'c' => rgb(80, 210, 255),
        b'd' => rgb(30, 120, 190),
        b'e' => rgb(255, 110, 0),
        b'g' => rgb(255, 200, 80),
        b'f' if frame == 0 => rgb(255, 250, 220),
        b'f' => rgb(255, 200, 80),
        b'w' if frame == 0 => C::WHITE,
        b'w' => rgb(255, 250, 220),
        b'l' => rgb(100, 255, 130),
        b'r' => rgb(255, 60, 60),
        _ => C::TRANSPARENT,
    }
}

fn enemy_color(kind: EnemyType, c: u8) -> C {
    match kind {
        EnemyType::Bee => match c {
            b'#' | b'b' => rgb(255, 215, 0),
            b'=' => rgb(200, 150, 0),
            b'o' => rgb(255, 180, 0),
            b'a' => rgb(255, 255, 160),
            b'e' => rgb(220, 50, 50),
            _ => C::TRANSPARENT,
        },
        EnemyType::Butterfly => match c {
            b'#' => rgb(30, 200, 255),
            b'd' | b'o' => rgb(10, 120, 190),
            b'h' => rgb(140, 235, 255),
            b'w' => C::WHITE,
            b'b' => rgb(220, 245, 255),
            b'a' => rgb(130, 180, 200),
            _ => C::TRANSPARENT,
        },
        EnemyType::Boss => match c {
            b'#' => rgb(220, 40, 40),
            b'd' => rgb(150, 20, 20),
            b'o' => rgb(100, 0, 0),
            b'm' => rgb(255, 130, 0),
            b'h' => C::WHITE,
            b'y' => rgb(255, 255, 200),
            _ => C::TRANSPARENT,
        },
    }
}

pub fn viewport(available: Rect) -> Rect {
    let scale = (available.width() / 800.0).min(available.height() / 600.0);
    Rect::from_center_size(available.center(), Vec2::new(800.0, 600.0) * scale)
}

struct Canvas {
    painter: Painter,
    rect: Rect,
    scale: f32,
}

impl Canvas {
    fn new(painter: &Painter, rect: Rect) -> Self {
        Self {
            painter: painter.with_clip_rect(rect),
            rect,
            scale: rect.width() / 800.0,
        }
    }
    fn pos(&self, x: f64, y: f64) -> Pos2 {
        self.rect.min + Vec2::new(x as f32, y as f32) * self.scale
    }
    fn rect(&self, x: f64, y: f64, w: f64, h: f64, color: C) {
        self.painter.rect_filled(
            Rect::from_min_size(self.pos(x, y), Vec2::new(w as f32, h as f32) * self.scale),
            0.0,
            color,
        );
    }
    fn text(&self, text: &str, x: Option<f64>, y: f64, size: f32, color: C) {
        let anchor = if x.is_some() {
            Align2::LEFT_TOP
        } else {
            Align2::CENTER_TOP
        };
        let pos = self.pos(x.unwrap_or(400.0), y);
        let font = FontId::proportional(size * self.scale);
        self.painter.text(
            pos + Vec2::splat(self.scale),
            anchor,
            text,
            font.clone(),
            C::from_black_alpha(180),
        );
        self.painter.text(pos, anchor, text, font, color);
    }
    fn pixels(&self, map: &[&str], x: f64, y: f64, scale: f64, color: impl Fn(u8) -> C) {
        for (row, line) in map.iter().enumerate() {
            for (col, c) in line.bytes().enumerate() {
                if c != b'.' {
                    let fill = color(c);
                    if fill != C::TRANSPARENT {
                        self.rect(
                            x + col as f64 * scale,
                            y + row as f64 * scale,
                            scale,
                            scale,
                            fill,
                        );
                    }
                }
            }
        }
    }
    fn player(&self, x: f64, y: f64, frame: usize, movement: f64) {
        if movement != 0.0 {
            self.pixels(PLAYER[frame], x - movement * 4.0, y, 2.0, |c| {
                player_color(c, frame).gamma_multiply(0.25)
            });
        }
        self.pixels(PLAYER[frame], x, y, 2.0, |c| player_color(c, frame));
        if frame == 0 {
            self.rect(x + 2.0, y + 20.0, 2.0, 2.0, rgb(100, 255, 130));
            self.rect(x + 28.0, y + 20.0, 2.0, 2.0, rgb(255, 60, 60));
        }
    }
    fn enemy(&self, e: &Enemy, frame: usize) {
        let b = &e.body;
        if matches!(e.state, EnemyState::Diving | EnemyState::Returning) {
            let len = if e.state == EnemyState::Diving {
                10.0
            } else {
                6.0
            };
            self.rect(
                b.x + 8.0,
                b.y - len,
                8.0,
                len,
                rgb(255, 200, 80).gamma_multiply(0.35),
            );
        }
        let map = match e.kind {
            EnemyType::Bee => BEE,
            EnemyType::Butterfly => BUTTERFLY,
            EnemyType::Boss => BOSS,
        };
        self.pixels(map[frame], b.x, b.y, 3.0, |c| enemy_color(e.kind, c));
        if e.kind == EnemyType::Bee && frame == 0 {
            for x in [3.0, 18.0] {
                self.rect(b.x + x, b.y - 3.0, 3.0, 3.0, rgb(255, 255, 160));
            }
        }
        if e.kind == EnemyType::Butterfly && frame == 1 {
            for x in [9.0, 12.0] {
                self.rect(b.x + x, b.y - 3.0, 3.0, 3.0, rgb(140, 235, 255));
            }
        }
        if e.captured {
            let (x, y) = (b.x + 3.0, b.y + 26.0);
            self.rect(x + 6.0, y, 4.0, 11.0, rgb(80, 210, 255));
            self.rect(x, y + 5.0, 16.0, 6.0, rgb(80, 210, 255));
            self.rect(x + 7.0, y + 2.0, 2.0, 3.0, rgb(100, 255, 130));
            for dx in [1.0, 12.0] {
                self.rect(x + dx, y + 9.0, 3.0, 3.0, rgb(255, 200, 80));
            }
        }
    }
    fn explosion(&self, e: &Explosion) {
        let p = e.progress();
        if p < 0.55 {
            self.painter.circle_stroke(
                self.pos(e.x, e.y),
                (e.radius * p * 1.4) as f32 * self.scale,
                Stroke::new(
                    (3.0 * (1.0 - p / 0.55)).max(1.0) as f32 * self.scale,
                    rgb(255, 230, 60).gamma_multiply((0.6 * (1.0 - p / 0.55)) as f32),
                ),
            );
        }
        let color = if p < 0.2 {
            C::WHITE
        } else if p < 0.45 {
            rgb(255, 230, 60)
        } else if p < 0.7 {
            rgb(255, 130, 20)
        } else {
            rgb(200, 40, 10)
        };
        for i in 0..12 {
            let angle = f64::from(i) * std::f64::consts::TAU / 12.0
                + p * if i % 2 == 0 { 0.9 } else { -0.9 };
            let dist = e.radius * p * (0.6 + 0.4 * f64::from((i * 7) % 5) / 4.0);
            let sz = (5.0 * (1.0 - p)).max(1.5);
            self.rect(
                e.x + angle.cos() * dist - sz / 2.0,
                e.y + angle.sin() * dist - sz / 2.0,
                sz,
                sz,
                color,
            );
        }
        if p < 0.25 {
            let flash = 10.0 * (1.0 - p / 0.25);
            self.rect(e.x - flash / 2.0, e.y - flash / 2.0, flash, flash, C::WHITE);
        } else if p > 0.55 {
            let r = e.radius * 0.5 * p;
            self.rect(
                e.x - r / 2.0,
                e.y - r / 2.0,
                r,
                r,
                rgb(80, 80, 80).gamma_multiply((0.5 * (1.0 - (p - 0.55) / 0.45)) as f32),
            );
        }
    }
    fn glow(&self, rx: f64, ry: f64) {
        let mut mesh = Mesh::default();
        mesh.colored_vertex(self.pos(400.0, 214.0), rgb(255, 170, 60));
        for (radius, color) in [(0.4, rgb(255, 80, 160)), (1.0, C::TRANSPARENT)] {
            for i in 0..64 {
                let angle = f64::from(i) * std::f64::consts::TAU / 64.0;
                mesh.colored_vertex(
                    self.pos(
                        400.0 + angle.cos() * rx * radius,
                        214.0 + angle.sin() * ry * radius,
                    ),
                    color,
                );
            }
        }
        for i in 0..64_u32 {
            let n = (i + 1) % 64;
            mesh.add_triangle(0, 1 + i, 1 + n);
            mesh.add_triangle(1 + i, 65 + i, 65 + n);
            mesh.add_triangle(1 + i, 65 + n, 1 + n);
        }
        self.painter.add(Shape::mesh(mesh));
    }
}

pub struct Visuals {
    stars: Vec<(f64, f64, usize)>,
    rng: StdRng,
    pub ticks: u64,
}

impl Visuals {
    pub fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let stars = (0..110)
            .map(|_| {
                (
                    rng.random::<f64>() * WIDTH,
                    rng.random::<f64>() * HEIGHT,
                    rng.random_range(0..4),
                )
            })
            .collect();
        Self {
            stars,
            rng,
            ticks: 0,
        }
    }
    pub fn tick(&mut self, dt: f64) {
        self.ticks += 1;
        for (x, y, level) in &mut self.stars {
            *y += [15.0, 35.0, 60.0, 90.0][*level] * dt;
            if *y >= HEIGHT {
                *y = 0.0;
                *x = self.rng.random::<f64>() * WIDTH;
            }
        }
    }
    pub fn draw(&self, painter: &Painter, rect: Rect, game: &Game) {
        let c = Canvas::new(painter, rect);
        c.rect(0.0, 0.0, WIDTH, HEIGHT, C::BLACK);
        for &(x, y, level) in &self.stars {
            let sz = [1.0, 1.0, 2.0, 3.0][level];
            let color = [
                rgb(90, 90, 90),
                rgb(170, 170, 170),
                C::WHITE,
                rgb(200, 220, 255),
            ][level]
                .gamma_multiply([0.45, 0.65, 0.9, 1.0][level]);
            c.rect(x, y, sz, sz, color);
        }
        let frame = ((self.ticks / 8) % 2) as usize;
        if game.phase == Phase::Menu {
            self.menu(&c, game, frame);
            return;
        }
        c.text(
            &format!("SCORE {}", game.score),
            Some(10.0),
            10.0,
            16.0,
            C::WHITE,
        );
        c.text(
            &format!("HI    {}", game.high_score),
            Some(300.0),
            10.0,
            16.0,
            C::CYAN,
        );
        c.text(
            &format!("LEVEL {}", game.level),
            Some(660.0),
            10.0,
            16.0,
            C::YELLOW,
        );
        for i in 0..game.player.lives {
            c.pixels(MINI, 10.0 + f64::from(i) * 22.0, 578.0, 2.0, |ch| {
                player_color(ch, 0)
            });
        }
        if game.player.dual {
            let x = 20.0 + f64::from(game.player.lives) * 22.0;
            for dx in [0.0, 18.0] {
                c.pixels(MINI, x + dx, 578.0, 2.0, |ch| player_color(ch, 0));
            }
            c.text("DUAL", Some(x + 38.0), 580.0, 12.0, C::CYAN);
        }
        for e in game.formation.enemies.iter().filter(|e| e.body.alive) {
            c.enemy(e, frame);
        }
        let p = &game.player;
        let blink = p.invulnerability > 0.0 && (p.invulnerability * 6.0).floor() as i32 % 2 == 0;
        if p.body.alive && !blink {
            let movement = if p.left {
                -1.0
            } else if p.right {
                1.0
            } else {
                0.0
            };
            c.player(p.body.x, p.body.y, frame, movement);
            if p.dual {
                c.player(p.body.x + p.body.width, p.body.y, frame, 0.0);
            }
        }
        for b in &game.bullets {
            let (core, tail) = if b.owner == BulletOwner::Player {
                (rgb(150, 255, 150), rgb(40, 180, 40))
            } else {
                (rgb(255, 80, 80), rgb(180, 30, 30))
            };
            c.rect(b.body.x, b.body.y + 4.2, 4.0, 7.8, tail);
            c.rect(b.body.x + 1.0, b.body.y + 1.8, 2.0, 7.8, core);
            c.rect(b.body.x + 1.0, b.body.y, 2.0, 4.2, rgb(255, 255, 220));
        }
        for e in &game.explosions {
            c.explosion(e);
        }
        match game.phase {
            Phase::GameOver => {
                c.text("GAME OVER", None, 250.0, 42.0, rgb(255, 69, 0));
                c.text("PRESS SPACE TO RETRY", None, 310.0, 20.0, C::WHITE);
            }
            Phase::StageClear => c.text(
                &format!("STAGE {} CLEAR!", game.level),
                None,
                260.0,
                32.0,
                C::CYAN,
            ),
            Phase::Paused => c.text("PAUSED", None, 270.0, 32.0, C::YELLOW),
            _ => {}
        }
    }
    fn menu(&self, c: &Canvas, game: &Game, frame: usize) {
        let t = self.ticks as f64;
        let radius = 130.0 + (0.5 + 0.5 * (t * 0.05).sin()) * 26.0;
        c.glow(radius, radius * 0.55);
        for (i, (kind, x)) in [
            (EnemyType::Boss, 400.0),
            (EnemyType::Butterfly, 312.0),
            (EnemyType::Butterfly, 488.0),
            (EnemyType::Bee, 224.0),
            (EnemyType::Bee, 576.0),
        ]
        .into_iter()
        .enumerate()
        {
            let mut enemy = Enemy::new(kind, x, 118.0, 0.0);
            enemy.body.x = x + (t * 0.03).sin() * 26.0;
            enemy.body.y = 118.0 + (t * 0.05 + i as f64).sin() * 6.0;
            c.enemy(&enemy, frame);
        }
        let colors = [
            rgb(255, 80, 160),
            rgb(255, 215, 0),
            rgb(50, 205, 50),
            rgb(255, 215, 0),
            rgb(255, 80, 160),
            rgb(255, 215, 0),
        ];
        let font = FontId::proportional(52.0 * c.scale);
        let glyphs: Vec<_> = "GALAGA"
            .chars()
            .map(|ch| {
                c.painter
                    .layout_no_wrap(ch.to_string(), font.clone(), C::WHITE)
            })
            .collect();
        let total: f32 = glyphs.iter().map(|g| g.size().x).sum();
        let mut x = c.rect.center().x - total / 2.0;
        for (glyph, color) in glyphs.into_iter().zip(colors) {
            let pos = Pos2::new(x, c.pos(0.0, 190.0).y);
            c.painter.galley_with_override_text_color(
                pos + Vec2::splat(3.0 * c.scale),
                glyph.clone(),
                rgb(120, 30, 0),
            );
            c.painter
                .galley_with_override_text_color(pos, glyph.clone(), color);
            x += glyph.size().x;
        }
        if (self.ticks / 30).is_multiple_of(2) {
            c.text("PRESS SPACE TO START", None, 290.0, 24.0, C::WHITE);
        }
        c.text(
            &format!("HIGH SCORE  {}", game.high_score),
            None,
            350.0,
            18.0,
            C::CYAN,
        );
        c.text(
            "LEFT RIGHT / A D  MOVE    SPACE  SHOOT    P  PAUSE",
            None,
            410.0,
            13.0,
            rgb(105, 105, 105),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letterbox_keeps_four_by_three_at_any_window_size() {
        for size in [[1200.0, 600.0], [800.0, 1000.0], [400.0, 300.0]] {
            let available = Rect::from_min_size(Pos2::ZERO, Vec2::new(size[0], size[1]));
            let rect = viewport(available);
            assert!((rect.width() / rect.height() - 4.0 / 3.0).abs() < 0.001);
            assert!(available.contains_rect(rect));
            assert_eq!(rect.center(), available.center());
        }
    }
}
