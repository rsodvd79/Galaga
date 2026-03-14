using Avalonia;

namespace Galaga.Entities;

public abstract class Entity
{
    public double X { get; set; }
    public double Y { get; set; }
    public double Width { get; }
    public double Height { get; }
    public bool IsAlive { get; set; } = true;

    protected Entity(double x, double y, double width, double height)
    {
        X = x;
        Y = y;
        Width = width;
        Height = height;
    }

    public Rect Bounds => new(X, Y, Width, Height);

    public bool CollidesWith(Entity other) =>
        IsAlive && other.IsAlive &&
        X < other.X + other.Width &&
        X + Width > other.X &&
        Y < other.Y + other.Height &&
        Y + Height > other.Y;
}
