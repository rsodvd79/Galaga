# Galaga

Follow `AGENTS.md` for current architecture, invariants and validation.
This project is Rust + eframe for native desktop and WebAssembly.

Build: `cargo build --release --locked`.
Test: `cargo test --locked`.
Format: `cargo fmt --all -- --check`.
Lint: `cargo clippy --all-targets --all-features --locked -- -D warnings`.
Web: `trunk build --release --locked`.

Keep the engine independent of UI/audio/storage, preserve the fixed 800x600
game space and use the smallest change that preserves native/web behavior.
Do not restore the retired .NET/Avalonia projects or generated binaries.
