namespace Galaga.Entities;

public class Player : Entity
{
    public const double DefaultSpeed = 280.0;
    public const int MaxBullets = 2;
    public const double RespawnDuration = 2.0;
    public const double InvulnerabilityDuration = 2.0;
    public const double DefaultX = 385.0;
    public const double DefaultY = 540.0;

    public int Lives { get; set; } = 3;
    public bool MoveLeft { get; set; }
    public bool MoveRight { get; set; }
    public bool IsRespawning { get; private set; }
    public double RespawnTimer { get; set; }

    public bool IsInvulnerable { get; private set; }
    private double _invulnTimer;
    public double InvulnerabilityRemaining => _invulnTimer;

    public bool HasDualFighter { get; private set; }

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
                IsInvulnerable = true;
                _invulnTimer = InvulnerabilityDuration;
                X = DefaultX;
                Y = DefaultY;
            }
            return;
        }

        if (!IsAlive) return;

        if (IsInvulnerable)
        {
            _invulnTimer -= elapsed;
            if (_invulnTimer <= 0)
                IsInvulnerable = false;
        }

        double dx = 0;
        if (MoveLeft)  dx -= DefaultSpeed * elapsed;
        if (MoveRight) dx += DefaultSpeed * elapsed;
        X = Math.Clamp(X + dx, 0, gameWidth - Width);
    }

    public void Die()
    {
        Lives--;
        IsAlive = false;
        IsInvulnerable = false;
        HasDualFighter = false;
        if (Lives > 0)
        {
            IsRespawning = true;
            RespawnTimer = RespawnDuration;
        }
    }

    public void GrantDualFighter() => HasDualFighter = true;

    public void CancelRespawn()
    {
        IsRespawning = false;
        RespawnTimer = 0;
    }

    public void Reset()
    {
        Lives = 3;
        IsAlive = true;
        IsRespawning = false;
        RespawnTimer = 0;
        IsInvulnerable = false;
        _invulnTimer = 0;
        HasDualFighter = false;
        MoveLeft = false;
        MoveRight = false;
        X = DefaultX;
        Y = DefaultY;
    }
}
