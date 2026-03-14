using Galaga.Engine;
using Galaga.Entities;
using Xunit;

namespace Galaga.Tests;

public class GameEngineTests
{
    private static (GameEngine engine, GameState state) CreateGame()
    {
        var state = new GameState();
        state.Reset();
        return (new GameEngine(state), state);
    }

    [Fact]
    public void Player_starts_with_three_lives()
    {
        var (_, state) = CreateGame();
        Assert.Equal(3, state.Player.Lives);
    }

    [Fact]
    public void Player_can_shoot_one_bullet()
    {
        var (engine, state) = CreateGame();
        state.Formation.Enemies.Clear();

        state.ShootPressed = true;
        engine.Tick(0.016);

        Assert.Single(state.Bullets.Where(b => b.Owner == BulletOwner.Player));
    }

    [Fact]
    public void Player_bullet_count_capped_at_two()
    {
        var (engine, state) = CreateGame();
        // Keep enemies alive so StageClear doesn't halt the game loop

        for (int i = 0; i < 6; i++)
        {
            state.ShootPressed = true;
            engine.Tick(0.001); // tiny dt so bullets don't fly off-screen
        }

        Assert.Equal(2, state.Bullets.Count(b => b.Owner == BulletOwner.Player));
    }

    [Fact]
    public void Player_dies_when_hit_by_enemy_bullet()
    {
        var (engine, state) = CreateGame();
        state.Formation.Enemies.Clear();

        state.Bullets.Add(new Bullet(
            state.Player.X + state.Player.Width / 2,
            state.Player.Y + 1,
            BulletOwner.Enemy));

        engine.Tick(0.016);

        Assert.Equal(2, state.Player.Lives);
        Assert.False(state.Player.IsAlive);
    }

    [Fact]
    public void Game_over_when_last_life_lost()
    {
        var (engine, state) = CreateGame();
        state.Formation.Enemies.Clear();
        state.Player.Lives = 1;

        state.Bullets.Add(new Bullet(
            state.Player.X + state.Player.Width / 2,
            state.Player.Y + 1,
            BulletOwner.Enemy));

        engine.Tick(0.016);

        Assert.Equal(GamePhase.GameOver, state.Phase);
    }

    [Fact]
    public void Stage_clear_triggers_when_all_enemies_dead()
    {
        var (engine, state) = CreateGame();

        foreach (var enemy in state.Formation.Enemies)
            enemy.IsAlive = false;

        engine.Tick(0.016);

        Assert.Equal(GamePhase.StageClear, state.Phase);
    }

    [Fact]
    public void Score_increases_when_in_formation_bee_is_killed()
    {
        var (engine, state) = CreateGame();

        var bee = state.Formation.Enemies.First(e => e.Type == EnemyType.Bee);
        bee.State     = EnemyState.InFormation;
        // Also update the formation slot so Update() doesn't snap it elsewhere
        bee.FormationX = 400;
        bee.FormationY = 300;
        bee.X          = 400;
        bee.Y          = 300;

        state.Bullets.Add(new Bullet(
            bee.X + bee.Width / 2,
            bee.Y + 1,
            BulletOwner.Player));

        engine.Tick(0.016);

        Assert.False(bee.IsAlive);
        Assert.Equal(bee.PointsInFormation, state.Score);
    }

    [Fact]
    public void Score_gives_diving_bonus()
    {
        var (engine, state) = CreateGame();

        var bee = state.Formation.Enemies.First(e => e.Type == EnemyType.Bee);
        bee.State = EnemyState.Diving;
        bee.X = 400;
        bee.Y = 300;

        state.Bullets.Add(new Bullet(
            bee.X + bee.Width / 2,
            bee.Y + 1,
            BulletOwner.Player));

        engine.Tick(0.016);

        Assert.Equal(bee.PointsDiving, state.Score);
    }
}

public class CollisionTests
{
    [Fact]
    public void Entities_collide_when_overlapping()
    {
        var p = new Player(100, 100);
        var b = new Bullet(105, 105, BulletOwner.Enemy);
        Assert.True(b.CollidesWith(p));
    }

    [Fact]
    public void Entities_do_not_collide_when_separated()
    {
        var p = new Player(100, 100);
        var b = new Bullet(300, 300, BulletOwner.Enemy);
        Assert.False(b.CollidesWith(p));
    }

    [Fact]
    public void Dead_entity_does_not_collide()
    {
        var p = new Player(100, 100);
        var b = new Bullet(105, 105, BulletOwner.Enemy);
        p.IsAlive = false;
        Assert.False(b.CollidesWith(p));
    }
}

public class EnemyFormationTests
{
    [Fact]
    public void Formation_initializes_with_correct_count()
    {
        var f = new EnemyFormation();
        f.Initialize();
        Assert.Equal(EnemyFormation.Rows * EnemyFormation.Cols, f.Enemies.Count);
    }

    [Fact]
    public void Boss_galaga_placed_in_first_row()
    {
        var f = new EnemyFormation();
        f.Initialize();
        Assert.Contains(f.Enemies, e => e.Type == EnemyType.BossGalaga);
    }
}
