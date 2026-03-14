namespace Galaga.Entities;

public enum BulletOwner { Player, Enemy }

public class Bullet : Entity
{
    public BulletOwner Owner { get; }
    private readonly double _speed;

    public Bullet(double centerX, double y, BulletOwner owner)
        : base(centerX - 2, y, 4, 12)
    {
        Owner = owner;
        // Player bullets travel up (negative Y), enemy bullets travel down
        _speed = owner == BulletOwner.Player ? -500.0 : 260.0;
    }

    public void Update(double elapsed) => Y += _speed * elapsed;
}
