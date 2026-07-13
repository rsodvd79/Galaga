using Avalonia;
using Avalonia.Media;
using Galaga.Entities;

namespace Galaga.Views;

public static class SpriteRenderer
{
    // ─── Cached brushes ──────────────────────────────────────────────────────
    private static readonly IBrush BeeBody    = new SolidColorBrush(Color.FromRgb(255, 215, 0));
    private static readonly IBrush BeeEye     = new SolidColorBrush(Color.FromRgb(220, 50,  50));
    private static readonly IBrush BeeAntenna = new SolidColorBrush(Color.FromRgb(255, 255, 160));

    private static readonly IBrush BflyWing      = new SolidColorBrush(Color.FromRgb(30,  200, 255));
    private static readonly IBrush BflyWingDark   = new SolidColorBrush(Color.FromRgb(10,  120, 190));
    private static readonly IBrush BflyWingLight  = new SolidColorBrush(Color.FromRgb(140, 235, 255));
    private static readonly IBrush BflyBody       = new SolidColorBrush(Color.FromRgb(180, 240, 255));
    private static readonly IBrush BflyStripe     = new SolidColorBrush(Color.FromRgb(255, 255, 255));

    private static readonly IBrush BossOuter   = new SolidColorBrush(Color.FromRgb(220, 40,  40));
    private static readonly IBrush BossOutline = new SolidColorBrush(Color.FromRgb(110, 10,  10));
    private static readonly IBrush BossMid     = new SolidColorBrush(Color.FromRgb(255, 130, 0));
    private static readonly IBrush BossCore    = new SolidColorBrush(Color.FromRgb(255, 230, 80));
    private static readonly IBrush BossEye     = new SolidColorBrush(Color.FromRgb(255, 255, 200));
    private static readonly IBrush BossHighlight = new SolidColorBrush(Color.FromRgb(255, 190, 190));

    private static readonly IBrush PlayerHull       = new SolidColorBrush(Color.FromRgb(50,  210, 90));
    private static readonly IBrush PlayerHullDark   = new SolidColorBrush(Color.FromRgb(30,  160, 60));
    private static readonly IBrush PlayerHullLight  = new SolidColorBrush(Color.FromRgb(120, 240, 150));
    private static readonly IBrush PlayerOutline    = new SolidColorBrush(Color.FromRgb(15,  90,  35));
    private static readonly IBrush PlayerCockpit    = new SolidColorBrush(Color.FromRgb(80,  210, 255));
    private static readonly IBrush PlayerCockpitDark = new SolidColorBrush(Color.FromRgb(30,  120, 190));
    private static readonly IBrush PlayerEngine     = new SolidColorBrush(Color.FromRgb(255, 110, 0));
    private static readonly IBrush PlayerEngineGlow = new SolidColorBrush(Color.FromRgb(255, 200, 80));
    private static readonly IBrush PlayerEngineHot  = new SolidColorBrush(Color.FromRgb(255, 250, 220));
    private static readonly IBrush PlayerAccent     = new SolidColorBrush(Color.FromRgb(100, 255, 130));
    private static readonly IBrush PlayerRedLight   = new SolidColorBrush(Color.FromRgb(255, 60,  60));
    private static readonly IBrush PlayerGreenLight = new SolidColorBrush(Color.FromRgb(100, 255, 130));

    private static readonly IBrush MiniShipColor = new SolidColorBrush(Color.FromRgb(50,  210, 90));

    private static readonly IBrush ExpWhite  = Brushes.White;
    private static readonly IBrush ExpYellow = new SolidColorBrush(Color.FromRgb(255, 230, 60));
    private static readonly IBrush ExpOrange = new SolidColorBrush(Color.FromRgb(255, 130, 20));
    private static readonly IBrush ExpRed    = new SolidColorBrush(Color.FromRgb(200, 40,  10));

    // ─── Pixel-art sprite maps ────────────────────────────────────────────────
    // '#' = main color, 'o' = outline/shadow, 'd' = dark shade, 'h' = highlight, '.' = empty.
    // 8×8 chars → rendered at 3×3 px = 24×24 total.

    private const double Ps = 3.0;

    // BEE — frame 0 (wings out)
    private static readonly string[] BeePx0 =
    {
        ".#....#.",
        "##.##.##",
        "########",
        "#.####.#",
        "########",
        ".##..##.",
        "##.##.##",
        ".##..##.",
    };
    // BEE — frame 1 (wings slightly raised)
    private static readonly string[] BeePx1 =
    {
        "#......#",
        ".##..##.",
        "########",
        ".######.",
        "########",
        "#.#..#.#",
        ".######.",
        "........",
    };

    // BUTTERFLY — frame 0
    private static readonly string[] ButterflyPx0 =
    {
        "o.hddh.o",
        "#hd##dh#",
        "########",
        ".######.",
        ".######.",
        "########",
        "#hd##dh#",
        "o.hddh.o",
    };
    // BUTTERFLY — frame 1 (wings shifted inward)
    private static readonly string[] ButterflyPx1 =
    {
        ".hd##dh.",
        "##.##.##",
        "########",
        "..####..",
        "..####..",
        "########",
        "##.##.##",
        ".hd##dh.",
    };

    // BOSS — frame 0
    private static readonly string[] BossPx0 =
    {
        "..####..",
        ".h####h.",
        "##d##d##",
        "########",
        "########",
        "##d##d##",
        ".h####h.",
        "..####..",
    };
    // BOSS — frame 1 (slightly different wing detail)
    private static readonly string[] BossPx1 =
    {
        "..####..",
        "##h##h##",
        "##d##d##",
        ".######.",
        ".######.",
        "##d##d##",
        "##h##h##",
        "..####..",
    };

    private static readonly Dictionary<char, IBrush> BeePalette = new()
    {
        ['#'] = BeeBody,
        ['o'] = BeeBody,
    };

    private static readonly Dictionary<char, IBrush> ButterflyPalette = new()
    {
        ['#'] = BflyWing,
        ['d'] = BflyWingDark,
        ['h'] = BflyWingLight,
        ['o'] = BflyWingDark,
    };

    private static readonly Dictionary<char, IBrush> BossPalette = new()
    {
        ['#'] = BossOuter,
        ['d'] = BossOutline,
        ['o'] = BossOutline,
        ['h'] = BossHighlight,
    };

    // ─── Public draw methods ─────────────────────────────────────────────────

    public static void DrawEnemy(DrawingContext ctx, Enemy enemy, int frame)
    {
        switch (enemy.Type)
        {
            case EnemyType.Bee:        DrawBee(ctx, enemy.X, enemy.Y, frame);        break;
            case EnemyType.Butterfly:  DrawButterfly(ctx, enemy.X, enemy.Y, frame);  break;
            case EnemyType.BossGalaga: DrawBossGalaga(ctx, enemy.X, enemy.Y, frame); break;
        }
    }

    private static void DrawBee(DrawingContext ctx, double x, double y, int frame)
    {
        var px = frame == 0 ? BeePx0 : BeePx1;
        DrawPixels(ctx, px, x, y, BeePalette);

        // Antenna tips
        ctx.FillRectangle(BeeAntenna, new Rect(x + 1 * Ps, y,         Ps, Ps));
        ctx.FillRectangle(BeeAntenna, new Rect(x + 6 * Ps, y,         Ps, Ps));
        // Eyes (row 3, cols 1 & 6)
        ctx.FillRectangle(BeeEye,     new Rect(x + 1 * Ps, y + 3 * Ps, Ps, Ps));
        ctx.FillRectangle(BeeEye,     new Rect(x + 6 * Ps, y + 3 * Ps, Ps, Ps));
    }

    private static void DrawButterfly(DrawingContext ctx, double x, double y, int frame)
    {
        var px = frame == 0 ? ButterflyPx0 : ButterflyPx1;
        DrawPixels(ctx, px, x, y, ButterflyPalette);
        ctx.FillRectangle(BflyBody,   new Rect(x + 3 * Ps, y + 2 * Ps, 2 * Ps, 4 * Ps));
        ctx.FillRectangle(BflyStripe, new Rect(x + 3 * Ps, y + 3 * Ps, 2 * Ps, 2 * Ps));
        // Head + antennae accents
        ctx.FillRectangle(BflyWingDark, new Rect(x + 3.4 * Ps, y + 1.6 * Ps, 0.4 * Ps, Ps));
        ctx.FillRectangle(BflyWingDark, new Rect(x + 4.2 * Ps, y + 1.6 * Ps, 0.4 * Ps, Ps));
    }

    private static void DrawBossGalaga(DrawingContext ctx, double x, double y, int frame)
    {
        var px = frame == 0 ? BossPx0 : BossPx1;
        DrawPixels(ctx, px, x, y, BossPalette);
        ctx.FillRectangle(BossMid,  new Rect(x + 1 * Ps, y + 2 * Ps, 6 * Ps, 4 * Ps));
        ctx.FillRectangle(BossCore, new Rect(x + 2 * Ps, y + 3 * Ps, 4 * Ps, 2 * Ps));
        ctx.FillRectangle(BossEye,  new Rect(x + 2 * Ps, y + 2 * Ps, Ps, Ps));
        ctx.FillRectangle(BossEye,  new Rect(x + 5 * Ps, y + 2 * Ps, Ps, Ps));
        // Core highlight glint
        ctx.FillRectangle(BossHighlight, new Rect(x + 3.5 * Ps, y + 3.3 * Ps, Ps * 0.6, Ps * 0.6));
    }

    // ─── Player ship ─────────────────────────────────────────────────────────
    public static void DrawPlayer(DrawingContext ctx, double x, double y, int frame = 0)
    {
        // Hull (main body + wings)
        var hull = new StreamGeometry();
        using (var gc = hull.Open())
        {
            gc.BeginFigure(new Point(x + 15, y), isFilled: true);
            gc.LineTo(new Point(x + 30, y + 16));
            gc.LineTo(new Point(x + 26, y + 14));
            gc.LineTo(new Point(x + 24, y + 18));
            gc.LineTo(new Point(x + 20, y + 20));
            gc.LineTo(new Point(x + 15, y + 18));
            gc.LineTo(new Point(x + 10, y + 20));
            gc.LineTo(new Point(x + 6,  y + 18));
            gc.LineTo(new Point(x + 4,  y + 14));
            gc.LineTo(new Point(x + 0,  y + 16));
            gc.EndFigure(isClosed: true);
        }
        ctx.DrawGeometry(PlayerHull, new Pen(PlayerOutline, 1), hull);

        // Center hull highlight (subtle raised panel down the spine)
        var spine = new StreamGeometry();
        using (var gc = spine.Open())
        {
            gc.BeginFigure(new Point(x + 15, y + 2), isFilled: true);
            gc.LineTo(new Point(x + 17, y + 17));
            gc.LineTo(new Point(x + 15, y + 19));
            gc.LineTo(new Point(x + 13, y + 17));
            gc.EndFigure(isClosed: true);
        }
        ctx.DrawGeometry(PlayerHullLight, null, spine);

        // Wing accents (dark outlines on the leading edges)
        var wingL = new StreamGeometry();
        using (var gc = wingL.Open())
        {
            gc.BeginFigure(new Point(x,     y + 16), isFilled: true);
            gc.LineTo(new Point(x + 4,     y + 14));
            gc.LineTo(new Point(x + 3,     y + 8));
            gc.EndFigure(isClosed: true);
        }
        ctx.DrawGeometry(PlayerHullDark, null, wingL);

        var wingR = new StreamGeometry();
        using (var gc = wingR.Open())
        {
            gc.BeginFigure(new Point(x + 30, y + 16), isFilled: true);
            gc.LineTo(new Point(x + 26,    y + 14));
            gc.LineTo(new Point(x + 27,    y + 8));
            gc.EndFigure(isClosed: true);
        }
        ctx.DrawGeometry(PlayerHullDark, null, wingR);

        // Wingtip running lights (red/green, aviation-style)
        ctx.FillRectangle(PlayerRedLight,   new Rect(x + 1,  y + 15, 2, 2));
        ctx.FillRectangle(PlayerGreenLight, new Rect(x + 27, y + 15, 2, 2));

        // Panel lines on the wings
        ctx.DrawLine(new Pen(PlayerOutline, 0.6), new Point(x + 8,  y + 18), new Point(x + 11, y + 15));
        ctx.DrawLine(new Pen(PlayerOutline, 0.6), new Point(x + 22, y + 18), new Point(x + 19, y + 15));

        // Cockpit
        ctx.FillRectangle(PlayerCockpit, new Rect(x + 12, y + 4,  6, 10));
        ctx.FillRectangle(PlayerCockpitDark, new Rect(x + 12, y + 10, 6, 4));

        // Cockpit frame outline
        ctx.DrawRectangle(new Pen(PlayerOutline, 0.6), new Rect(x + 12, y + 4, 6, 10));

        // Cockpit highlight
        ctx.FillRectangle(PlayerAccent,  new Rect(x + 13, y + 5,  4, 3));

        // Nose tip
        ctx.FillRectangle(PlayerAccent,  new Rect(x + 14, y + 1,  2, 3));

        // Engine pods
        ctx.FillRectangle(PlayerEngine,     new Rect(x + 4,  y + 15, 5, 5));
        ctx.FillRectangle(PlayerEngine,     new Rect(x + 21, y + 15, 5, 5));
        ctx.DrawRectangle(new Pen(PlayerOutline, 0.6), new Rect(x + 4,  y + 15, 5, 5));
        ctx.DrawRectangle(new Pen(PlayerOutline, 0.6), new Rect(x + 21, y + 15, 5, 5));

        // Engine glow (inner)
        ctx.FillRectangle(PlayerEngineGlow, new Rect(x + 5,  y + 16, 3, 4));
        ctx.FillRectangle(PlayerEngineGlow, new Rect(x + 22, y + 16, 3, 4));

        // Flickering thruster flame trailing behind the ship, alternating with frame
        double flameLen = frame == 0 ? 5 : 7;
        ctx.FillRectangle(PlayerEngineHot, new Rect(x + 5.5, y + 20, 2, flameLen));
        ctx.FillRectangle(PlayerEngineHot, new Rect(x + 22.5, y + 20, 2, flameLen));
    }

    // ─── Mini player (HUD) ───────────────────────────────────────────────────
    public static void DrawMiniPlayer(DrawingContext ctx, double x, double y)
    {
        ctx.FillRectangle(MiniShipColor, new Rect(x + 6,  y,     4, 11));
        ctx.FillRectangle(MiniShipColor, new Rect(x,      y + 5, 16, 6));
        ctx.FillRectangle(PlayerEngine,  new Rect(x + 1,  y + 9, 3, 3));
        ctx.FillRectangle(PlayerEngine,  new Rect(x + 12, y + 9, 3, 3));
        ctx.FillRectangle(PlayerCockpit, new Rect(x + 7,  y + 2, 2, 3));
    }

    // ─── Explosion ───────────────────────────────────────────────────────────
    public static void DrawExplosion(DrawingContext ctx, Explosion exp)
    {
        double p = exp.Progress;

        IBrush brush = p < 0.25 ? ExpWhite
                     : p < 0.55 ? ExpYellow
                     : p < 0.78 ? ExpOrange
                     :             ExpRed;

        double radius   = exp.Radius * p;
        double partSize = Math.Max(1.5, 5.0 * (1.0 - p));

        for (int i = 0; i < 8; i++)
        {
            double angle = i * Math.PI / 4.0 + p * 0.8;
            double px = exp.CenterX + Math.Cos(angle) * radius;
            double py = exp.CenterY + Math.Sin(angle) * radius;
            ctx.FillRectangle(brush,
                new Rect(px - partSize / 2, py - partSize / 2, partSize, partSize));
        }

        if (p < 0.30)
        {
            double flash = 8 * (1 - p / 0.30);
            ctx.FillRectangle(ExpWhite,
                new Rect(exp.CenterX - flash / 2, exp.CenterY - flash / 2, flash, flash));
        }
    }

    // ─── Helper ──────────────────────────────────────────────────────────────

    private static void DrawPixels(DrawingContext ctx, string[] pixels,
        double ox, double oy, Dictionary<char, IBrush> palette)
    {
        for (int row = 0; row < pixels.Length; row++)
        {
            var line = pixels[row];
            for (int col = 0; col < line.Length; col++)
            {
                char c = line[col];
                if (c == '.') continue;
                if (!palette.TryGetValue(c, out var brush)) continue;
                ctx.FillRectangle(brush,
                    new Rect(ox + col * Ps, oy + row * Ps, Ps, Ps));
            }
        }
    }
}
