# Galaga — Agent Guide

Stack: **.NET 8 · Avalonia 11 · xUnit · Silk.NET.OpenAL**

## Commands

```bash
dotnet run --project Galaga/Galaga.csproj   # run game
dotnet test                                  # all tests (13)
dotnet test --filter "FullyQualifiedName~Player_dies_when_hit_by_enemy_bullet"  # single test
dotnet publish Galaga/Galaga.csproj -c Release -r osx-x64 --self-contained true -p:PublishSingleFile=true -o publish/
```

## Architecture

- `GameEngine` has **no Avalonia dependency** — unit-testable directly
- `GameCanvas` drives the loop at 16ms (`DispatcherTimer`), dt capped at 50ms
- Entrypoint: `MainWindow.axaml` hosts `GameCanvas` (fixed 800×600 logical space) inside a `Viewbox` so the window is resizable and the game scales uniformly (letterboxed)
- `--screenshots` CLI flag auto-captures menu + gameplay frames then exits
- Sound: PCM synthesis at 22050Hz via OpenAL; runs silently if unavailable (`SoundPlayer.IsAvailable`)

## Engine Quirks

- `GameEngine.Tick()` exits immediately if `Phase != Playing` — order terminal-phase writes carefully in a single tick
- `Enemy` state machine: `FormationEntry` → `InFormation` → `Diving` → `Returning` → `InFormation`
- **In `InFormation` state**, `Enemy.Update()` **snaps** X/Y to `FormationX + oscillationOffset` / `FormationY` every tick. Setting X/Y directly has no lasting effect unless you also update `FormationX`/`FormationY`
- `Player.Die()` sets `IsAlive = false`, starts 2s `RespawnTimer`; `Player.Update()` restores `IsAlive` automatically when timer expires
- Dead entities (`IsAlive = false`) never collide (`Entity.CollidesWith` checks both sides)
- Starfield generated once with `Random(42)` in `GameCanvas` constructor
- All drawing via `DrawingContext.FillRectangle` — no image files or sprite sheets
- `AllowUnsafeBlocks` enabled (OpenAL interop)

## Testing Quirks

- Tests manipulate `GameState` directly + call `engine.Tick(dt)`
- For collision tests, set **both** `FormationX`/`FormationY` and `X`/`Y` on the enemy (see `Score_increases_when_in_formation_bee_is_killed`)
- Clear `state.Formation.Enemies` when testing player-specific behavior to avoid stage-clear loop exit

## Reference

- `Galaga/.github/copilot-instructions.md` — earlier instructions (archived but compatible)
