# Galaga — Agent Guide

Rust 2024 (MSRV 1.95), eframe/egui 0.37 with glow, rodio on desktop and
Web Audio on wasm. One Cargo package, library `galaga_core`, binary `galaga`.

## Commands

```sh
cargo run --release --locked
cargo test --locked
cargo test --locked player_dies_when_hit_by_enemy_bullet
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo clippy --target wasm32-unknown-unknown --lib --locked -- -D warnings
trunk build --release --locked
cargo run --release --locked -- --screenshots
```

Install the wasm target and Trunk 0.21.14 for web. See README for native
system dependencies and browser smoke tests. CI produces native archives
and a static web archive; no Pages deployment.

## Boundaries and invariants

- `game.rs` and `entities.rs` are pure domain logic, using f64 and no egui
  geometry, persistence or device APIs. Use owned structs/enums and Vec.
- `Game::new(seed)` makes randomness reproducible. Tests do not write files.
- `app.rs` advances at 16 ms with an accumulator, caps elapsed time at 50 ms
  and updates once per egui frame, not once per layout pass.
- The game area is always 800x600; only `render.rs` transforms/clips it for
  letterboxing and DPI. Keep current hitboxes, including the dual-fighter.
- In `EnemyState::Formation`, update snaps x/y to formation_x + offset and
  formation_y. Collision tests must set both position and formation slot.
- `Game::tick` returns unless Playing. GameOver must not be overwritten by
  StageClear when the last enemy and last player life disappear together.
- Respawn and invulnerability last 2 s each; the player updates its own
  respawn state. Dead bodies never collide. StageClear lasts 2.5 s.
- Boss capture loses a life. Killing a carrying boss with lives remaining
  grants dual-fighter and +1000 points; dying clears dual.
- The app drains sound events and flushes increased records after ticks.
  Audio/storage/screenshot failure must not stop ordinary gameplay.
- The record starts fresh in Rust. Never read or remove legacy C# saves.
- Browser AudioContext must be created/resumed inside an actual DOM gesture,
  not just the later egui repaint. Native dependencies must not enter wasm.
- Screenshot callbacks are asynchronous. Preserve the requested phase and
  wait for completion before advancing auto-capture or closing the window.
- Reuse procedural ASCII sprites/palettes and existing icons. No sprite
  sheets, ECS, async runtime or speculative abstraction layers.

Generated `target/`, `dist/`, `publish/` and app bundles are ignored.
Keep Cargo.lock committed. Release builds for other platforms must not be
claimed runtime-tested merely because compilation succeeded.
