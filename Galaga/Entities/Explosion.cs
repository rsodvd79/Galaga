namespace Galaga.Entities;

public class Explosion
{
    public double CenterX  { get; }
    public double CenterY  { get; }
    public double Radius   { get; }
    public double Timer    { get; set; }

    public const double Duration = 0.5;

    public double Progress  => 1.0 - Timer / Duration;
    public bool   IsFinished => Timer <= 0;

    public Explosion(double cx, double cy, double radius = 22)
    {
        CenterX = cx;
        CenterY = cy;
        Radius  = radius;
        Timer   = Duration;
    }
}
