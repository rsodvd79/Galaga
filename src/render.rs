use eframe::egui::{Align2, Color32 as C, FontId, Mesh, Painter, Pos2, Rect, Shape, Stroke, Vec2};
use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::{
    entities::*,
    game::{Game, Phase},
};

const BEE: [&[&str]; 2] = [
    &[
        "..a..........a..",
        "...a........a...",
        "...oh......ho...",
        "..oh#o....o#ho..",
        ".oh##=o..o=##ho.",
        "oh###==oo==###ho",
        "oh##=oeeeeo=##ho",
        ".o##obeeeebo##o.",
        "..ooob#hh#booo..",
        "...ohb####bho...",
        "..oh#b====b#ho..",
        ".oh##b#hh#b##ho.",
        ".o##ob====bo##o.",
        "..oo..b##b..oo..",
        "......o##o......",
        ".......oo.......",
    ],
    &[
        ".a............a.",
        "..a..........a..",
        ".oh..........ho.",
        "oh#o........o#ho",
        "o##=o......o=##o",
        "oh#==o....o==#ho",
        ".o##=oeeeeo=##o.",
        "..o#obeeeebo#o..",
        "...oob#hh#boo...",
        "....ob####bo....",
        "...ohb====bho...",
        "..oh#b#hh#b#ho..",
        "..o##b====b##o..",
        "...ooob##booo...",
        "......o##o......",
        ".......oo.......",
    ],
];
const BUTTERFLY: [&[&str]; 2] = [
    &[
        ".o....a..a....o.",
        "oho....aa....oho",
        "ohhdo..bb..odhho",
        "oh##doheehod##ho",
        "oh#whdbwwbdhw#ho",
        ".oh#whbwwbhw#ho.",
        "..oh#dbwwbd#ho..",
        "...oddbwwbddo...",
        "...oddbwwbddo...",
        "..oh#dbwwbd#ho..",
        ".oh#whbwwbhw#ho.",
        "oh#whdbwwbdhw#ho",
        "oh##do####od##ho",
        "ohhdo.o##o.odhho",
        "oho....oo....oho",
        ".o............o.",
    ],
    &[
        "......a..a......",
        ".......aa.......",
        "...oh..bb..ho...",
        "..oh#doeeod#ho..",
        ".oh#wdbwwbdw#ho.",
        "oh##whbwwbhw##ho",
        ".oh#hdbwwbdh#ho.",
        "...oddbwwbddo...",
        "...oddbwwbddo...",
        ".oh#hdbwwbdh#ho.",
        "oh##whbwwbhw##ho",
        ".oh#wdbwwbdw#ho.",
        "..oh#do##od#ho..",
        "...oh.o##o.ho...",
        ".......oo.......",
        "................",
    ],
];
const BOSS: [&[&str]; 2] = [
    &[
        "......oooo......",
        ".....oh##ho.....",
        "....oh#mm#ho....",
        "...oh#mmmm#ho...",
        "..oh#oyyyyo#ho..",
        ".oh#doyyyyod#ho.",
        "oh#ddoeeeeodd#ho",
        "oh#h##mmmm##h#ho",
        "o#mm#hcccch#mm#o",
        "o#m#dhcccchd#m#o",
        "oh##ddhcchdd##ho",
        ".o##ddmmmmdd##o.",
        "..o##dmmmmd##o..",
        "...o##hhhh##o...",
        "....oo####oo....",
        "......oooo......",
    ],
    &[
        "......oooo......",
        ".....oh##ho.....",
        "...ooh#mm#hoo...",
        "..oh##mmmm##ho..",
        ".oh##oyyyyo##ho.",
        "oh#ddoyyyyodd#ho",
        "oh#hdoeeeeodh#ho",
        "o#hh##mmmm##hh#o",
        "o#mm#hcccch#mm#o",
        ".om#dhcccchd#mo.",
        ".oh#ddhcchdd#ho.",
        "..o#ddmmmmdd#o..",
        "...o#dmmmmd#o...",
        "...o##hhhh##o...",
        "....oo####oo....",
        "......oooo......",
    ],
];
const PLAYER: [&[&str]; 2] = [
    &[
        "...........hh...........",
        "..........ohho..........",
        "..........ohho..........",
        ".........ohccho.........",
        ".........ohcwco.........",
        "........ohcccdho........",
        "........ohcccdho........",
        ".......oh#cddc#ho.......",
        "......oh##cddc##ho......",
        ".....oh###=oo=###ho.....",
        "....oh####=hh=####ho....",
        "...oh##h##=##=##h##ho...",
        "..oh###o##=##=##o###ho..",
        ".oh###oo##=##=##oo###ho.",
        ".ol###oo#ge##eg#oo###ro.",
        ".oh##=oo#ge==eg#oo=##ho.",
        ".o##==oo#ge==eg#oo==##o.",
        "..o#==oo#ee==ee#oo==#o..",
        "..o#==oooff==ffooo==#o..",
        "...o==oooff==ffooo==o...",
        "....oooooff==ffooooo....",
        "........gfw..wfg........",
        "........gww..wwg........",
        ".........f....f.........",
    ],
    &[
        "...........hh...........",
        "..........ohho..........",
        "..........ohho..........",
        ".........ohccho.........",
        ".........ohcwco.........",
        "........ohcccdho........",
        "........ohcccdho........",
        ".......oh#cddc#ho.......",
        "......oh##cddc##ho......",
        ".....oh###=oo=###ho.....",
        "....oh####=hh=####ho....",
        "...oh##h##=##=##h##ho...",
        "..oh###o##=##=##o###ho..",
        ".oh###oo##=##=##oo###ho.",
        ".oh###oo#ge##eg#oo###ho.",
        ".oh##=oo#ge==eg#oo=##ho.",
        ".o##==oo#ge==eg#oo==##o.",
        "..o#==oo#ee==ee#oo==#o..",
        "..o#==oooff==ffooo==#o..",
        "...o==ooofw==wfooo==o...",
        "....ooooogw==wgooooo....",
        ".........ff..ff.........",
        "........................",
        "........................",
    ],
];
const MINI: &[&str] = &[
    "...cc...", "..cccc..", ".##cc##.", "###==###", "##geegg#", ".=oeo=..",
];
const PLAYER_SCALE: f64 = 32.0 / 24.0;
const ENEMY_SCALE: f64 = 24.0 / 16.0;
const STAR_SPEEDS: [f64; 4] = [15.0, 35.0, 60.0, 90.0];

fn rgb(r: u8, g: u8, b: u8) -> C {
    C::from_rgb(r, g, b)
}

fn player_color(c: u8, frame: usize) -> C {
    match c {
        b'#' => rgb(50, 210, 90),
        b'h' => rgb(170, 255, 195),
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
            b'#' => rgb(255, 215, 0),
            b'b' => rgb(255, 175, 35),
            b'h' => rgb(255, 248, 175),
            b'=' => rgb(165, 100, 15),
            b'o' => rgb(85, 50, 20),
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
            b'e' => rgb(255, 120, 205),
            _ => C::TRANSPARENT,
        },
        EnemyType::Boss => match c {
            b'#' => rgb(220, 40, 40),
            b'd' => rgb(150, 20, 20),
            b'o' => rgb(100, 0, 0),
            b'm' => rgb(255, 130, 0),
            b'h' => C::WHITE,
            b'y' => rgb(255, 255, 200),
            b'e' => rgb(255, 75, 110),
            b'c' => rgb(255, 220, 65),
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
    fn vertical_fade(&self, rect: [f64; 4], top: C, bottom: C) {
        let [x, y, w, h] = rect;
        let mut mesh = Mesh::default();
        for (x, y, color) in [
            (x, y, top),
            (x + w, y, top),
            (x, y + h, bottom),
            (x + w, y + h, bottom),
        ] {
            mesh.colored_vertex(self.pos(x, y), color);
        }
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(1, 3, 2);
        self.painter.add(Shape::mesh(mesh));
    }
    fn halo(&self, center: [f64; 2], radius: [f64; 2], color: C) {
        let mut mesh = Mesh::default();
        mesh.colored_vertex(self.pos(center[0], center[1]), color);
        for i in 0..24 {
            let angle = f64::from(i) * std::f64::consts::TAU / 24.0;
            mesh.colored_vertex(
                self.pos(
                    center[0] + angle.cos() * radius[0],
                    center[1] + angle.sin() * radius[1],
                ),
                C::TRANSPARENT,
            );
        }
        for i in 0..24 {
            mesh.add_triangle(0, i + 1, (i + 1) % 24 + 1);
        }
        self.painter.add(Shape::mesh(mesh));
    }
    fn star(&self, x: f64, y: f64, level: usize, time: f64) {
        let size = [1.0, 1.0, 2.0, 3.0][level];
        let twinkle = 0.82 + 0.18 * (time * 0.04 + x * 0.07).sin();
        let color = [
            rgb(120, 140, 180),
            rgb(170, 190, 225),
            rgb(235, 240, 255),
            rgb(170, 215, 255),
        ][level]
            .gamma_multiply(([0.5, 0.7, 0.9, 1.0][level] * twinkle * 0.45) as f32);
        let length = STAR_SPEEDS[level] * 0.24;
        self.vertical_fade(
            [x + size * 0.2, y - length, size * 0.6, length + size * 0.5],
            C::TRANSPARENT,
            color.gamma_multiply(0.45),
        );
        if level == 3 {
            self.halo(
                [x + size / 2.0, y + size / 2.0],
                [5.0, 7.0],
                color.gamma_multiply(0.12),
            );
            self.rect(x - 1.0, y + 1.0, 5.0, 1.0, color.gamma_multiply(0.25));
        }
        self.rect(x, y, size, size, color);
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
        for engine_x in [12.0, 20.0] {
            let flame = if frame == 0 { 14.0 } else { 9.0 };
            self.halo(
                [x + engine_x, y + 29.0],
                [7.0, flame],
                rgb(40, 190, 255).gamma_multiply(0.35),
            );
            self.vertical_fade(
                [x + engine_x - 1.5, y + 27.0, 3.0, flame],
                rgb(130, 225, 255).gamma_multiply(0.75),
                C::TRANSPARENT,
            );
        }
        if movement != 0.0 {
            self.pixels(PLAYER[frame], x - movement * 4.0, y, PLAYER_SCALE, |c| {
                player_color(c, frame).gamma_multiply(0.25)
            });
        }
        self.pixels(PLAYER[frame], x, y, PLAYER_SCALE, |c| {
            player_color(c, frame)
        });
    }
    fn enemy(&self, e: &Enemy, frame: usize) {
        let b = &e.body;
        let aura = match e.kind {
            EnemyType::Bee => rgb(255, 180, 35),
            EnemyType::Butterfly => rgb(35, 175, 255),
            EnemyType::Boss => rgb(255, 65, 75),
        };
        self.halo(
            [b.x + 12.0, b.y + 12.0],
            [17.0, 17.0],
            aura.gamma_multiply(0.15),
        );
        if matches!(e.state, EnemyState::Diving | EnemyState::Returning) {
            let len = if e.state == EnemyState::Diving {
                22.0
            } else {
                14.0
            };
            self.vertical_fade(
                [b.x + 8.0, b.y - len, 8.0, len],
                C::TRANSPARENT,
                aura.gamma_multiply(0.5),
            );
        }
        let map = match e.kind {
            EnemyType::Bee => BEE,
            EnemyType::Butterfly => BUTTERFLY,
            EnemyType::Boss => BOSS,
        };
        self.pixels(map[frame], b.x, b.y, ENEMY_SCALE, |c| {
            enemy_color(e.kind, c)
        });
        if e.captured {
            let (x, y) = (b.x + 3.0, b.y + 26.0);
            self.vertical_fade(
                [x, b.y + 20.0, 16.0, 20.0],
                rgb(80, 210, 255).gamma_multiply(0.25),
                C::TRANSPARENT,
            );
            self.pixels(PLAYER[frame], x, y, 2.0 / 3.0, |ch| match ch {
                b'h' | b'w' => rgb(200, 250, 255),
                b'o' | b'=' => rgb(25, 100, 145),
                b'e' | b'f' | b'g' => rgb(255, 195, 80),
                _ => rgb(70, 205, 235),
            });
        }
    }
    fn bullet(&self, bullet: &Bullet, time: f64) {
        let b = &bullet.body;
        let player = bullet.owner == BulletOwner::Player;
        let color = if player {
            rgb(65, 255, 155)
        } else {
            rgb(255, 65, 90)
        };
        let cx = b.x + b.width / 2.0;
        let pulse = (0.85 + 0.15 * (time * 0.5 + b.y * 0.04).sin()) as f32;
        self.halo(
            [cx, b.y + b.height / 2.0],
            [7.0, 15.0],
            color.gamma_multiply(0.3 * pulse),
        );
        if player {
            self.vertical_fade(
                [cx - 1.5, b.y + b.height * 0.5, 3.0, 22.0],
                color.gamma_multiply(0.8),
                C::TRANSPARENT,
            );
        } else {
            self.vertical_fade(
                [cx - 1.5, b.y - 16.0, 3.0, 22.0],
                C::TRANSPARENT,
                color.gamma_multiply(0.8),
            );
        }
        self.painter.rect_filled(
            Rect::from_min_size(
                self.pos(b.x, b.y),
                Vec2::new(b.width as f32, b.height as f32) * self.scale,
            ),
            2.0 * self.scale,
            color,
        );
        let white = rgb(240, 255, 245);
        let (top, bottom) = if player {
            (white, color)
        } else {
            (color, white)
        };
        self.vertical_fade([cx - 0.75, b.y + 1.0, 1.5, b.height - 2.0], top, bottom);
        let head_y = if player {
            b.y + 1.5
        } else {
            b.y + b.height - 1.5
        };
        self.painter
            .circle_filled(self.pos(cx, head_y), 1.4 * self.scale, white);
    }

    fn explosion(&self, e: &Explosion) {
        let p = e.progress();
        if !(0.0..1.0).contains(&p) {
            return;
        }
        let fade = (1.0 - p).powf(1.4) as f32;
        let expansion = 1.0 - (1.0 - p).powi(3);
        let radius = e.radius * (0.15 + expansion * 1.35);

        self.halo(
            [e.x, e.y],
            [radius * 1.25, radius * 1.1],
            rgb(255, 90, 25).gamma_multiply(0.4 * fade),
        );
        for i in 0..6 {
            let angle = f64::from(i) * std::f64::consts::TAU / 6.0 + e.x * 0.013;
            let spread = e.radius * expansion * 0.45;
            let puff = e.radius * (0.15 + 0.35 * p);
            self.halo(
                [
                    e.x + angle.cos() * spread,
                    e.y + angle.sin() * spread - p * 8.0,
                ],
                [puff, puff * 1.15],
                rgb(70, 55, 75).gamma_multiply(0.6 * (p as f32).sqrt() * fade),
            );
            if p < 0.65 {
                self.halo(
                    [e.x + angle.cos() * spread, e.y + angle.sin() * spread],
                    [puff * 1.1, puff * 0.85],
                    rgb(255, 150, 35).gamma_multiply(0.55 * (1.0 - p / 0.65).powf(1.2) as f32),
                );
            }
        }
        for (offset, width, color) in [
            (
                0.0,
                0.6 + (1.0 - p) * 1.8,
                rgb(255, 190, 65).gamma_multiply(0.85 * fade),
            ),
            (2.5, 0.6, rgb(255, 240, 185).gamma_multiply(0.4 * fade)),
        ] {
            self.painter.circle_stroke(
                self.pos(e.x, e.y),
                (radius + offset) as f32 * self.scale,
                Stroke::new(width as f32 * self.scale, color),
            );
        }
        for i in 0..24 {
            let variation = (f64::from(i) * 0.618_034).fract();
            let angle = f64::from(i) * std::f64::consts::TAU / 24.0
                + (f64::from(i) * 2.4).sin() * 0.12
                + e.x * 0.013
                + e.y * 0.019;
            let dist = e.radius * expansion * (0.6 + variation * 0.9);
            let length = (2.0 + (1.0 - p) * 6.0) * (0.5 + variation);
            let tail_dist = (dist - length).max(0.0);
            let gravity = p * p * 10.0;
            let tip = [e.x + angle.cos() * dist, e.y + angle.sin() * dist + gravity];
            let tail = [
                e.x + angle.cos() * tail_dist,
                e.y + angle.sin() * tail_dist + gravity,
            ];
            let color = if i % 3 == 0 {
                rgb(255, 235, 155)
            } else {
                rgb(255, 130, 35)
            };
            self.painter.line_segment(
                [self.pos(tail[0], tail[1]), self.pos(tip[0], tip[1])],
                Stroke::new(
                    (1.0 + variation as f32) * self.scale,
                    color.gamma_multiply(fade),
                ),
            );
            if i % 3 == 0 {
                self.halo(tip, [3.0, 3.0], color.gamma_multiply(0.5 * fade));
            }
            self.painter.circle_filled(
                self.pos(tip[0], tip[1]),
                (0.7 + (1.0 - p) as f32) * self.scale,
                color.gamma_multiply(fade),
            );
        }
        if p < 0.3 {
            let flash = 1.0 - p / 0.3;
            let size = e.radius * 0.6 * flash;
            self.halo(
                [e.x, e.y],
                [size, size],
                rgb(255, 245, 210).gamma_multiply(0.9),
            );
            self.painter.circle_filled(
                self.pos(e.x, e.y),
                (size * 0.35) as f32 * self.scale,
                C::WHITE.gamma_multiply(flash as f32),
            );
            self.rect(
                e.x - size,
                e.y - 0.5,
                size * 2.0,
                1.0,
                C::WHITE.gamma_multiply(flash as f32 * 0.7),
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
            *y += STAR_SPEEDS[*level] * dt;
            if *y >= HEIGHT {
                *y = 0.0;
                *x = self.rng.random::<f64>() * WIDTH;
            }
        }
    }
    pub fn draw(&self, painter: &Painter, rect: Rect, game: &Game) {
        let c = Canvas::new(painter, rect);
        c.vertical_fade([0.0, 0.0, WIDTH, HEIGHT], rgb(4, 8, 20), rgb(0, 1, 5));
        for &(x, y, level) in &self.stars {
            c.star(x, y, level, self.ticks as f64);
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
            c.bullet(b, self.ticks as f64);
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
    use eframe::egui;

    #[test]
    fn star_trails_fade_upward_and_follow_parallax_without_wrapping_across_the_screen() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let draw = |level, y| {
            let ctx = egui::Context::default();
            let mut output = ctx.run_ui(Default::default(), |ui| {
                let painter = ui.ctx().layer_painter(egui::LayerId::background());
                Canvas::new(&painter, rect).star(100.0, y, level, 0.0);
            });
            output.textures_delta.clear();
            let head = output
                .shapes
                .iter()
                .rev()
                .find_map(|shape| {
                    if let Shape::Rect(head) = &shape.shape {
                        Some(head)
                    } else {
                        None
                    }
                })
                .expect("Star must have a visible head");
            assert!(
                head.fill.a() < 128,
                "Stars must stay dimmer than combat effects"
            );
            let trail = output
                .shapes
                .into_iter()
                .find_map(|shape| {
                    if let Shape::Mesh(mesh) = shape.shape {
                        Some((shape.clip_rect, mesh))
                    } else {
                        None
                    }
                })
                .expect("Star must have a gradient trail");
            assert_eq!(trail.0, rect);
            let vertices = &trail.1.vertices;
            assert!(vertices[0].pos.y < y as f32);
            assert_eq!(vertices[0].color.a(), 0);
            assert!(vertices[2].color.a() > 0);
            assert!(vertices[2].color.a() <= 64);
            assert!(vertices[2].pos.y >= y as f32);
            vertices[2].pos.y - vertices[0].pos.y
        };
        assert!(draw(3, 100.0) > draw(0, 100.0));

        let mut visuals = Visuals::new(42);
        visuals.stars = vec![(100.0, HEIGHT - 0.5, 3)];
        visuals.tick(crate::game::STEP);
        let (x, y, level) = visuals.stars[0];
        assert!((0.0..WIDTH).contains(&x));
        assert_eq!(y, 0.0);
        assert!(draw(level, y) < HEIGHT as f32 / 10.0);
    }

    #[test]
    fn projectiles_have_bright_leading_heads_and_fading_wakes_opposite_their_motion() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        for owner in [BulletOwner::Player, BulletOwner::Enemy] {
            let bullet = Bullet::new(100.0, 200.0, owner);
            let ctx = egui::Context::default();
            let mut output = ctx.run_ui(Default::default(), |ui| {
                let painter = ui.ctx().layer_painter(egui::LayerId::background());
                Canvas::new(&painter, rect).bullet(&bullet, 0.0);
            });
            output.textures_delta.clear();
            let trail = output
                .shapes
                .iter()
                .find_map(|shape| {
                    if let Shape::Mesh(mesh) = &shape.shape
                        && mesh.vertices.len() == 4
                    {
                        Some(mesh)
                    } else {
                        None
                    }
                })
                .expect("Projectile must have a gradient wake");
            let head = output
                .shapes
                .iter()
                .find_map(|shape| {
                    if let Shape::Circle(head) = &shape.shape {
                        Some(head)
                    } else {
                        None
                    }
                })
                .expect("Projectile must have a bright leading head");
            if owner == BulletOwner::Player {
                assert!(head.center.y < (bullet.body.y + bullet.body.height / 2.0) as f32);
                assert_eq!(trail.vertices[2].color.a(), 0);
                assert!(trail.vertices[0].color.a() > 0);
                assert!(trail.vertices[2].pos.y > (bullet.body.y + bullet.body.height) as f32);
            } else {
                assert!(head.center.y > (bullet.body.y + bullet.body.height / 2.0) as f32);
                assert_eq!(trail.vertices[0].color.a(), 0);
                assert!(trail.vertices[2].color.a() > 0);
                assert!(trail.vertices[0].pos.y < bullet.body.y as f32);
            }
        }
    }

    #[test]
    fn explosions_expand_and_fade_without_leaving_geometry_after_their_lifetime() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let mut previous_radius = 0.0;
        let mut previous_alpha = 256;
        for progress in [0.0, 0.3, 0.6, 0.9, 1.0, 1.1] {
            let explosion = Explosion {
                x: 400.0,
                y: 300.0,
                radius: 28.0,
                timer: 0.5 * (1.0 - progress),
            };
            let ctx = egui::Context::default();
            let mut output = ctx.run_ui(Default::default(), |ui| {
                let painter = ui.ctx().layer_painter(egui::LayerId::background());
                Canvas::new(&painter, rect).explosion(&explosion);
            });
            output.textures_delta.clear();
            let effects: Vec<_> = output
                .shapes
                .iter()
                .filter(|shape| {
                    matches!(
                        shape.shape,
                        Shape::Mesh(_) | Shape::Circle(_) | Shape::LineSegment { .. }
                    )
                })
                .collect();
            if progress >= 1.0 {
                assert!(
                    effects.is_empty(),
                    "Completed explosions must leave no visible effects"
                );
                continue;
            }
            for shape in &effects {
                let bounds = shape.shape.visual_bounding_rect();
                assert!(
                    [bounds.min.x, bounds.min.y, bounds.max.x, bounds.max.y]
                        .iter()
                        .all(|v| v.is_finite())
                );
                assert!(rect.contains_rect(bounds));
            }
            let shock = effects
                .iter()
                .find_map(|shape| {
                    if let Shape::Circle(circle) = &shape.shape {
                        Some(circle)
                    } else {
                        None
                    }
                })
                .expect("Active explosion must have a shockwave");
            assert!(shock.radius > previous_radius);
            assert!(i32::from(shock.stroke.color.a()) < previous_alpha);
            previous_radius = shock.radius;
            previous_alpha = i32::from(shock.stroke.color.a());
        }
    }

    #[test]
    fn detailed_sprite_frames_fit_the_hitboxes_and_have_complete_palettes() {
        let check = |frames: [&[&str]; 2], scale: f64, body: &Body, color: &dyn Fn(u8) -> C| {
            for frame in frames {
                assert_eq!(frame.len() as f64 * scale, body.height);
                for row in frame {
                    assert_eq!(row.len() as f64 * scale, body.width);
                    for pixel in row.bytes().filter(|&p| p != b'.') {
                        assert_ne!(
                            color(pixel),
                            C::TRANSPARENT,
                            "Unmapped sprite pixel: {pixel}"
                        );
                    }
                }
            }
        };
        for (frames, kind) in [
            (BEE, EnemyType::Bee),
            (BUTTERFLY, EnemyType::Butterfly),
            (BOSS, EnemyType::Boss),
        ] {
            let enemy = Enemy::new(kind, 0.0, 0.0, 0.0);
            check(frames, ENEMY_SCALE, &enemy.body, &|p| enemy_color(kind, p));
        }
        check(PLAYER, PLAYER_SCALE, &Player::default().body, &|p| {
            player_color(p, 0)
        });
    }

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
