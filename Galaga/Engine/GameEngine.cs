using Galaga.Audio;
using Galaga.Entities;

namespace Galaga.Engine;

public class GameEngine
{
    private readonly GameState _state;
    private readonly Random _random = new();

    public GameEngine(GameState state) => _state = state;

    public void Tick(double elapsed)
    {
        if (_state.Phase != GamePhase.Playing) return;

        var player    = _state.Player;
        var formation = _state.Formation;

        player.Update(elapsed, GameState.GameWidth);
        formation.Update(elapsed, GameState.GameWidth, GameState.GameHeight, _state.EnemySpeedMultiplier);

        UpdateBullets(elapsed);
        UpdateExplosions(elapsed);
        HandlePlayerShoot(player);
        HandleEnemyShooting(formation, elapsed);
        TriggerEnemyDives(formation, player);
        ResolveBulletEnemyCollisions(formation);
        ResolveEnemyPlayerCollision(formation, player);
        ResolveBulletPlayerCollision(player);
        CheckStageClear(formation);
    }

    public void UpdateStageClear(double elapsed)
    {
        if (_state.Phase != GamePhase.StageClear) return;
        _state.StageClearTimer -= elapsed;
        if (_state.StageClearTimer <= 0)
            _state.NextLevel();
    }

    private void UpdateBullets(double elapsed)
    {
        foreach (var b in _state.Bullets)
            b.Update(elapsed);

        _state.Bullets.RemoveAll(b =>
            b.Y < -b.Height || b.Y > GameState.GameHeight + 20);
    }

    private void HandlePlayerShoot(Player player)
    {
        if (!_state.ShootPressed) return;
        _state.ShootPressed = false;

        if (!player.IsAlive || player.IsRespawning) return;

        int maxBullets = player.HasDualFighter ? Player.MaxBullets * 2 : Player.MaxBullets;
        int playerBullets = _state.Bullets.Count(b => b.Owner == BulletOwner.Player);
        if (playerBullets >= maxBullets) return;

        if (player.HasDualFighter)
        {
            _state.Bullets.Add(new Bullet(player.X + 4,                 player.Y, BulletOwner.Player));
            _state.Bullets.Add(new Bullet(player.X + player.Width - 4, player.Y, BulletOwner.Player));
        }
        else
        {
            _state.Bullets.Add(new Bullet(player.X + player.Width / 2, player.Y, BulletOwner.Player));
        }
        _state.PendingSounds.Enqueue(SoundEffect.Shoot);
    }

    private void HandleEnemyShooting(EnemyFormation formation, double elapsed)
    {
        _state.EnemyShootTimer -= elapsed;
        if (_state.EnemyShootTimer > 0) return;

        var shooters = formation.Enemies
            .Where(e => e.IsAlive && e.State is EnemyState.InFormation or EnemyState.Diving)
            .ToList();

        if (shooters.Count > 0)
        {
            var shooter = shooters[_random.Next(shooters.Count)];
            _state.Bullets.Add(new Bullet(
                shooter.X + shooter.Width / 2,
                shooter.Y + shooter.Height,
                BulletOwner.Enemy));
        }

        _state.EnemyShootTimer = _random.NextDouble() * 1.5 + 0.5;
    }

    private void TriggerEnemyDives(EnemyFormation formation, Player player)
    {
        // Limit concurrent divers to 2 (checked independently of formation state,
        // so a new dive can start while other enemies are already Diving/Returning)
        int activeDivers = formation.Enemies.Count(e => e.IsAlive && e.State == EnemyState.Diving);
        if (activeDivers >= 2) return;

        if (_random.NextDouble() > 0.008) return;

        var candidates = formation.Enemies
            .Where(e => e.IsAlive && e.State == EnemyState.InFormation)
            .ToList();

        if (candidates.Count == 0) return;

        var diver = candidates[_random.Next(candidates.Count)];
        diver.StartDive(
            player.X + player.Width  / 2,
            player.Y + player.Height / 2);
    }

    private void ResolveBulletEnemyCollisions(EnemyFormation formation)
    {
        var playerBullets = _state.Bullets
            .Where(b => b.Owner == BulletOwner.Player && b.IsAlive)
            .ToList();

        foreach (var bullet in playerBullets)
        {
            foreach (var enemy in formation.Enemies.Where(e => e.IsAlive))
            {
                if (!bullet.CollidesWith(enemy)) continue;

                bullet.IsAlive = false;
                enemy.IsAlive  = false;

                int points;
                if (enemy.CarriesCapturedShip && _state.Player.Lives > 0)
                {
                    _state.Player.GrantDualFighter();
                    points = (enemy.State == EnemyState.Diving
                        ? enemy.PointsDiving
                        : enemy.PointsInFormation) + 1000;
                    _state.PendingSounds.Enqueue(SoundEffect.Rescue);
                }
                else
                {
                    points = enemy.State == EnemyState.Diving
                        ? enemy.PointsDiving
                        : enemy.PointsInFormation;
                }

                _state.Score     += points;
                _state.HighScore  = Math.Max(_state.HighScore, _state.Score);
                HighScoreStore.Save(_state.HighScore);

                _state.Explosions.Add(new Explosion(
                    enemy.X + enemy.Width  / 2,
                    enemy.Y + enemy.Height / 2,
                    radius: 20));
                _state.PendingSounds.Enqueue(SoundEffect.EnemyExplosion);
                break;
            }
        }

        _state.Bullets.RemoveAll(b => !b.IsAlive);
    }

    private void ResolveEnemyPlayerCollision(EnemyFormation formation, Player player)
    {
        if (!player.IsAlive || player.IsInvulnerable) return;

        foreach (var enemy in formation.Enemies)
        {
            if (!enemy.IsAlive || enemy.State != EnemyState.Diving) continue;
            if (!enemy.CollidesWith(player)) continue;

            // Boss Galaga tractor-beams and captures the player's fighter
            // instead of destroying it outright.
            if (enemy.Type == EnemyType.BossGalaga && !enemy.CarriesCapturedShip)
            {
                enemy.CarriesCapturedShip = true;
                player.Die();

                _state.Explosions.Add(new Explosion(
                    player.X + player.Width  / 2,
                    player.Y + player.Height / 2,
                    radius: 24));
                _state.PendingSounds.Enqueue(SoundEffect.Capture);

                if (player.Lives <= 0)
                    _state.Phase = GamePhase.GameOver;

                break;
            }

            enemy.IsAlive = false;
            player.Die();

            _state.Explosions.Add(new Explosion(
                player.X + player.Width  / 2,
                player.Y + player.Height / 2,
                radius: 28));
            _state.PendingSounds.Enqueue(SoundEffect.PlayerDeath);

            if (player.Lives <= 0)
                _state.Phase = GamePhase.GameOver;

            break;
        }
    }

    private void ResolveBulletPlayerCollision(Player player)
    {
        if (!player.IsAlive || player.IsInvulnerable) return;

        foreach (var bullet in _state.Bullets
            .Where(b => b.Owner == BulletOwner.Enemy && b.IsAlive)
            .ToList())
        {
            if (!bullet.CollidesWith(player)) continue;

            bullet.IsAlive = false;
            player.Die();

            _state.Explosions.Add(new Explosion(
                player.X + player.Width  / 2,
                player.Y + player.Height / 2,
                radius: 28));
            _state.PendingSounds.Enqueue(SoundEffect.PlayerDeath);

            if (player.Lives <= 0)
                _state.Phase = GamePhase.GameOver;

            break;
        }

        _state.Bullets.RemoveAll(b => !b.IsAlive);
    }

    private void CheckStageClear(EnemyFormation formation)
    {
        if (_state.Phase != GamePhase.Playing) return;
        if (formation.AliveCount != 0) return;

        _state.Phase           = GamePhase.StageClear;
        _state.StageClearTimer = 2.5;
        _state.PendingSounds.Enqueue(SoundEffect.StageClear);
    }

    private void UpdateExplosions(double elapsed)
    {
        foreach (var exp in _state.Explosions)
            exp.Timer -= elapsed;
        _state.Explosions.RemoveAll(e => e.IsFinished);
    }
}
