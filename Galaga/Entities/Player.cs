namespace Galaga.Entities;

public class Player : Entity
{
    public const double DefaultSpeed = 280.0;
    public const int MaxBullets = 2;
    public const double RespawnDuration = 2.0;
    public const double DefaultX = 385.0;
    public const double DefaultY = 540.0;

    public int Lives { get; set; } = 3;
    public bool MoveLeft { get; set; }
    public bool MoveRight { get; set; }
    public bool IsCaptured { get; set; }
    public bool IsRespawning { get; private set; }
    public double RespawnTimer { get; private set; }

    public Player(double x, double y) : base(x, y, 30, 20) { }

    public void Update(double elapsed, double gameWidth)
    {
        if (IsRespawning)
        {
            RespawnTimer -= elapsed;
            if (RespawnTimer <= 0)
            {
                IsRespawning = false;
                IsAlive = true;
                X = DefaultX;
                Y = DefaultY;
            }
            return;
        }

        if (!IsAlive) return;

        double dx = 0;
        if (MoveLeft)  dx -= DefaultSpeed * elapsed;
        if (MoveRight) dx += DefaultSpeed * elapsed;
        X = Math.Clamp(X + dx, 0, gameWidth - Width);
    }

    public void Die()
    {
        Lives--;
        IsAlive = false;
        if (Lives > 0)
        {
            IsRespawning = true;
            RespawnTimer = RespawnDuration;
        }
    }

    public void Reset()
    {
        Lives = 3;
        IsAlive = true;
        IsRespawning = false;
        RespawnTimer = 0;
        MoveLeft = false;
        MoveRight = false;
        IsCaptured = false;
        X = DefaultX;
        Y = DefaultY;
    }
}
