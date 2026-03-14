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

Stack: **.NET 8 · Avalonia 11 · xUnit**

---

## Architecture

```
Galaga/
├── Galaga/                   # Avalonia desktop app
│   ├── Engine/
│   │   ├── GameEngine.cs     # Pure game logic: tick, collision, scoring
│   │   └── GameState.cs      # All mutable state (phase, score, lives, lists)
│   ├── Entities/
│   │   ├── Entity.cs         # Abstract base: position, size, AABB collision
│   │   ├── Player.cs         # Movement, shooting limits, respawn timer
│   │   ├── Enemy.cs          # Per-enemy state machine (FormationEntry → InFormation → Diving → Returning)
│   │   ├── Bullet.cs         # Owner-aware, direction determined by BulletOwner
│   │   └── EnemyFormation.cs # Formation grid, oscillation, Initialize()
│   └── Views/
│       ├── GameCanvas.cs     # Avalonia Control: game loop timer, key events, Render()
│       └── MainWindow.axaml  # Hosts GameCanvas, 800×600 fixed size
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
All logic is hardcoded to **800 × 600** (`GameState.GameWidth/Height`). The window is non-resizable.

### Enemy state machine
Enemies always follow: `FormationEntry` → `InFormation` → `Diving` → `Returning` → `InFormation`. In `InFormation` state, `Enemy.Update()` **snaps** X/Y to `FormationX + oscillationOffset` and `FormationY` every tick — setting `X`/`Y` directly has no lasting effect unless you also update `FormationX`/`FormationY`.

### Shooting limits
- Player: max 2 bullets on screen simultaneously (`Player.MaxBullets`).
- Enemy: `EnemyShootTimer` controlled by `GameEngine`; only enemies in `InFormation` or `Diving` state shoot.

### Collision detection
AABB via `Entity.CollidesWith(Entity)`. Dead entities (`IsAlive = false`) never collide.

### Respawn
`Player.Die()` sets `IsAlive = false` and starts a 2-second `RespawnTimer`. After the timer expires inside `Player.Update()`, position resets to `(DefaultX, DefaultY)` and `IsAlive` is restored — no external call needed.

### Phase guard
`GameEngine.Tick()` exits immediately if `Phase != Playing`. All logic that sets a terminal phase (`GameOver`, `StageClear`) inside a tick must be ordered carefully — later checks won't fire because `CheckStageClear` also guards on `Phase == Playing`.

### Rendering
No sprites. All entities are drawn with `DrawingContext.FillRectangle` using colored rectangles. The starfield is generated once in the constructor with a fixed seed (42) and rendered every frame.

### Testing
Tests manipulate `GameState` directly and call `GameEngine.Tick(dt)`. When placing an enemy at a specific position for collision tests, set **both** `FormationX/FormationY` and `X/Y`.
