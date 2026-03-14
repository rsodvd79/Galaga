using Galaga.Audio;
using Galaga.Entities;

namespace Galaga.Engine;

public enum GamePhase { Menu, Playing, Paused, StageClear, GameOver }

public class GameState
{
    public const double GameWidth  = 800.0;
    public const double GameHeight = 600.0;

    public GamePhase Phase    { get; set; } = GamePhase.Menu;
    public int       Score    { get; set; }
    public int       HighScore { get; set; }
    public int       Level    { get; set; } = 1;

    public Player         Player     { get; } = new(Player.DefaultX, Player.DefaultY);
    public EnemyFormation Formation  { get; } = new();
    public List<Bullet>   Bullets    { get; } = new();
    public List<Explosion> Explosions { get; } = new();

    public double EnemyShootTimer { get; set; }
    public double StageClearTimer { get; set; }
    public bool   ShootPressed    { get; set; }

    // GameCanvas dequeues these after each tick to trigger audio
    public Queue<SoundEffect> PendingSounds { get; } = new();

    public void Reset()
    {
        Score  = 0;
        Level  = 1;
        Phase  = GamePhase.Playing;
        Player.Reset();
        Bullets.Clear();
        Explosions.Clear();
        Formation.Initialize();
        EnemyShootTimer = 3.0;
        PendingSounds.Clear();
    }

    public void NextLevel()
    {
        Level++;
        Phase = GamePhase.Playing;
        Bullets.Clear();
        Explosions.Clear();
        Player.IsAlive = true;
        Player.X = Player.DefaultX;
        Player.Y = Player.DefaultY;
        Formation.Initialize();
        EnemyShootTimer = Math.Max(1.0, 3.0 - Level * 0.2);
    }
}
