# Galaga — Copilot Instructions

## Build & Test

```bash
# Build
dotnet build

# Run game
dotnet run --project Galaga/Galaga.csproj

# Run all tests
dotnet test

# Run a single test
dotnet test --filter "FullyQualifiedName~Player_dies_when_hit_by_enemy_bullet"
```

Stack: **.NET 8 · Avalonia 11 · xUnit · Silk.NET.OpenAL**

---

## Architecture

```
Galaga/
├── Galaga/                   # Avalonia desktop app
│   ├── Engine/
│   │   ├── GameEngine.cs     # Pure game logic: tick, collision, scoring, capture/rescue
│   │   ├── GameState.cs      # All mutable state (phase, score, lives, lists)
│   │   └── HighScoreStore.cs # High-score persistence to a JSON file (best-effort)
│   ├── Entities/
│   │   ├── Entity.cs         # Abstract base: position, size, AABB collision
│   │   ├── Player.cs         # Movement, shooting limits, respawn, invulnerability, dual-fighter
│   │   ├── Enemy.cs          # Per-enemy state machine (FormationEntry → InFormation → Diving → Returning); can carry a captured ship
│   │   ├── Bullet.cs         # Owner-aware, direction determined by BulletOwner
│   │   ├── Explosion.cs      # Visual-only explosion particle data
│   │   └── EnemyFormation.cs # Formation grid, oscillation, Initialize()
│   ├── Views/
│   │   ├── GameCanvas.cs     # Avalonia Control: game loop timer, key events, Render()
│   │   ├── SpriteRenderer.cs # All drawing (pixel-art sprites + StreamGeometry)
│   │   └── MainWindow.axaml  # Resizable window; hosts GameCanvas in a Viewbox (fixed 800×600 logical space, letterboxed)
│   └── Audio/
│       └── SoundPlayer.cs    # OpenAL PCM synthesis (shoot, explosion, death, stage-clear, capture, rescue)
└── Galaga.Tests/             # xUnit — engine & entity tests only (no UI)
    └── UnitTest1.cs
```

**Data flow:** `GameCanvas.OnTick` → `GameEngine.Tick(dt)` → mutates `GameState` → `InvalidateVisual()` → `GameCanvas.Render(DrawingContext)` reads `GameState`.

`GameEngine` has no Avalonia dependency and can be unit-tested directly.

---

## Key Conventions

### Game loop
`DispatcherTimer` at 16 ms (~60 fps) in `GameCanvas`. Elapsed time is capped at 50 ms to prevent the spiral-of-death on slow frames.

### Fixed game area
All logic is hardcoded to **800 × 600** (`GameState.GameWidth/Height`). The window **is resizable**: `MainWindow.axaml` wraps `GameCanvas` in a `Viewbox` (`Stretch="Uniform"`), so the fixed 800×600 logical space scales uniformly and is letterboxed. Coordinates and collisions always stay in the 800×600 space.

### Enemy state machine
Enemies always follow: `FormationEntry` → `InFormation` → `Diving` → `Returning` → `InFormation`. In `InFormation` state, `Enemy.Update()` **snaps** X/Y to `FormationX + oscillationOffset` and `FormationY` every tick — setting `X`/`Y` directly has no lasting effect unless you also update `FormationX`/`FormationY`.

### Shooting limits
- Player: max 2 bullets on screen simultaneously (`Player.MaxBullets`), or **4 with the dual-fighter** (fires two side-by-side shots).
- Enemy: `EnemyShootTimer` controlled by `GameEngine`; only enemies in `InFormation` or `Diving` state shoot.

### Collision detection
AABB via `Entity.CollidesWith(Entity)`. Dead entities (`IsAlive = false`) never collide.

### Respawn & invulnerability
`Player.Die()` sets `IsAlive = false` and starts a 2-second `RespawnTimer`. After the timer expires inside `Player.Update()`, position resets to `(DefaultX, DefaultY)` and `IsAlive` is restored — no external call needed. On respawn the player becomes `IsInvulnerable` for 2 s (`InvulnerabilityDuration`): it blinks and both `ResolveEnemyPlayerCollision`/`ResolveBulletPlayerCollision` skip collision while invulnerable.

### Capture & dual-fighter
A **Boss Galaga** in `Diving` state that touches the player *captures* it instead of killing it outright (`Enemy.CarriesCapturedShip = true`, player loses a life). Destroying that boss while it carries the ship (and the player still has lives) calls `Player.GrantDualFighter()` → two side-by-side ships, double fire, +1000 bonus. Dying clears the dual-fighter. Sounds: `SoundEffect.Capture` and `SoundEffect.Rescue`.

### High-score persistence
`HighScoreStore.Load()`/`Save(int)` read/write a JSON file under `LocalApplicationData/Galaga/highscore.json`. All I/O is best-effort (wrapped in try/catch) so it never crashes the game. `GameCanvas` loads it at startup; `GameEngine` saves on every score increase.

### Phase guard
`GameEngine.Tick()` exits immediately if `Phase != Playing`. All logic that sets a terminal phase (`GameOver`, `StageClear`) inside a tick must be ordered carefully — later checks won't fire because `CheckStageClear` also guards on `Phase == Playing`.

### Rendering
No image files. Entities are pixel-art sprites drawn in code (`SpriteRenderer`) via `DrawingContext.FillRectangle` + `StreamGeometry`. The menu adds a pulsing radial glow, a drifting attract-mode enemy formation, and a blinking prompt. The starfield is generated in the constructor with `Random.Shared` (unseeded) and scrolls every frame.

### Testing
20 xUnit tests manipulate `GameState` directly and call `GameEngine.Tick(dt)`. When placing an enemy at a specific position for collision tests, set **both** `FormationX/FormationY` and `X/Y`. Clear `state.Formation.Enemies` when testing player-specific behavior to avoid the stage-clear loop exit.
