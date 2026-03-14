namespace Galaga.Entities;

public enum EnemyType { Bee, Butterfly, BossGalaga }

public enum EnemyState { FormationEntry, InFormation, Diving, Returning }

public class Enemy : Entity
{
    public EnemyType Type { get; }
    public EnemyState State { get; set; } = EnemyState.FormationEntry;

    public double FormationX { get; set; }
    public double FormationY { get; set; }

    private double _diveAngle;
    private readonly double _diveSpeed;
    private double _loopProgress;
    private double _entryDelay;

    public int PointsInFormation => Type switch
    {
        EnemyType.Bee        => 50,
        EnemyType.Butterfly  => 80,
        EnemyType.BossGalaga => 150,
        _                    => 50
    };

    public int PointsDiving => Type switch
    {
        EnemyType.Bee        => 100,
        EnemyType.Butterfly  => 160,
        EnemyType.BossGalaga => 400,
        _                    => 100
    };

    public Enemy(EnemyType type, double formationX, double formationY, double entryDelay)
        : base(formationX, -50, 24, 24)
    {
        Type = type;
        FormationX = formationX;
        FormationY = formationY;
        _entryDelay = entryDelay;
        _diveSpeed = type == EnemyType.BossGalaga ? 160.0 : 200.0;
    }

    public void StartDive(double targetX, double targetY)
    {
        State = EnemyState.Diving;
        _diveAngle = Math.Atan2(targetY - Y, targetX - X);
        _loopProgress = 0;
    }

    public void Update(double elapsed, double formationOffsetX, double gameWidth, double gameHeight)
    {
        switch (State)
        {
            case EnemyState.FormationEntry:
                UpdateEntry(elapsed, formationOffsetX);
                break;
            case EnemyState.InFormation:
                X = FormationX + formationOffsetX;
                Y = FormationY;
                break;
            case EnemyState.Diving:
                UpdateDive(elapsed, gameWidth, gameHeight);
                break;
            case EnemyState.Returning:
                UpdateReturn(elapsed, formationOffsetX);
                break;
        }
    }

    private void UpdateEntry(double elapsed, double formationOffsetX)
    {
        if (_entryDelay > 0)
        {
            _entryDelay -= elapsed;
            return;
        }

        double targetX = FormationX + formationOffsetX;
        double targetY = FormationY;

        double dx = targetX - X;
        double dy = targetY - Y;
        double dist = Math.Sqrt(dx * dx + dy * dy);

        if (dist < 4)
        {
            X = targetX;
            Y = targetY;
            State = EnemyState.InFormation;
            return;
        }

        const double speed = 220.0;
        X += dx / dist * speed * elapsed;
        Y += dy / dist * speed * elapsed;
    }

    private void UpdateDive(double elapsed, double gameWidth, double gameHeight)
    {
        _loopProgress += elapsed;
        X += Math.Cos(_diveAngle) * _diveSpeed * elapsed;
        Y += Math.Sin(_diveAngle) * _diveSpeed * elapsed;
        _diveAngle += elapsed * 0.4; // gentle curve toward bottom

        if (Y > gameHeight + 50)
        {
            // Reenter from top offset to the side
            Y = -50;
            X = Math.Clamp(FormationX + Math.Sin(_loopProgress) * 80, 0, gameWidth - Width);
            State = EnemyState.Returning;
        }
    }

    private void UpdateReturn(double elapsed, double formationOffsetX)
    {
        double targetX = FormationX + formationOffsetX;
        double targetY = FormationY;

        double dx = targetX - X;
        double dy = targetY - Y;
        double dist = Math.Sqrt(dx * dx + dy * dy);

        if (dist < 4)
        {
            X = targetX;
            Y = targetY;
            State = EnemyState.InFormation;
            return;
        }

        const double speed = 180.0;
        X += dx / dist * speed * elapsed;
        Y += dy / dist * speed * elapsed;
    }
}
