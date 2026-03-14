namespace Galaga.Entities;

public class EnemyFormation
{
    public const int Cols = 8;
    public const int Rows = 5;
    public const double StartX = 180.0;
    public const double StartY = 100.0;
    public const double SpacingX = 56.0;
    public const double SpacingY = 48.0;
    private const double MaxOscillation = 40.0;
    private const double OscillationSpeed = 28.0;

    public List<Enemy> Enemies { get; } = new();

    private double _oscillationOffset;
    private double _oscillationDirection = 1;

    public double OscillationOffsetX => _oscillationOffset;

    public void Initialize()
    {
        Enemies.Clear();
        int delay = 0;

        for (int row = 0; row < Rows; row++)
        {
            for (int col = 0; col < Cols; col++)
            {
                var type = row == 0 && (col == 3 || col == 4)
                    ? EnemyType.BossGalaga
                    : row <= 1
                        ? EnemyType.Butterfly
                        : EnemyType.Bee;

                double fx = StartX + col * SpacingX;
                double fy = StartY + row * SpacingY;

                Enemies.Add(new Enemy(type, fx, fy, delay * 0.10));
                delay++;
            }
        }
    }

    public void Update(double elapsed, double gameWidth, double gameHeight)
    {
        _oscillationOffset += OscillationSpeed * _oscillationDirection * elapsed;
        if (Math.Abs(_oscillationOffset) >= MaxOscillation)
            _oscillationDirection *= -1;

        foreach (var enemy in Enemies)
            enemy.Update(elapsed, _oscillationOffset, gameWidth, gameHeight);
    }

    public bool AllInFormation =>
        Enemies.All(e => !e.IsAlive || e.State == EnemyState.InFormation);

    public int AliveCount => Enemies.Count(e => e.IsAlive);
}
