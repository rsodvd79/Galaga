using Avalonia;
using Avalonia.Media;
using Galaga.Entities;

namespace Galaga.Views;

public static class SpriteRenderer
{
    // ─── Cached brushes ──────────────────────────────────────────────────────
    // ─── Bee palette ─────────────────────────────────────────────────────────
    private static readonly IBrush BeeBody    = new SolidColorBrush(Color.FromRgb(255, 215, 0));
    private static readonly IBrush BeeDark    = new SolidColorBrush(Color.FromRgb(200, 150, 0));
    private static readonly IBrush BeeWing    = new SolidColorBrush(Color.FromRgb(255, 180, 0));
    private static readonly IBrush BeeEye     = new SolidColorBrush(Color.FromRgb(220, 50,  50));
    private static readonly IBrush BeeAntenna = new SolidColorBrush(Color.FromRgb(255, 255, 160));

    // ─── Butterfly palette ───────────────────────────────────────────────────
    private static readonly IBrush BflyWing      = new SolidColorBrush(Color.FromRgb(30,  200, 255));
    private static readonly IBrush BflyWingDark   = new SolidColorBrush(Color.FromRgb(10,  120, 190));
    private static readonly IBrush BflyWingLight  = new SolidColorBrush(Color.FromRgb(140, 235, 255));
    private static readonly IBrush BflyBody       = new SolidColorBrush(Color.FromRgb(220, 245, 255));
    private static readonly IBrush BflyBodyDark   = new SolidColorBrush(Color.FromRgb(130, 180, 200));
    private static readonly IBrush BflyStripe     = new SolidColorBrush(Color.FromRgb(255, 255, 255));

    // ─── Boss palette ────────────────────────────────────────────────────────
    private static readonly IBrush BossOuter    = new SolidColorBrush(Color.FromRgb(220, 40,  40));
    private static readonly IBrush BossDark     = new SolidColorBrush(Color.FromRgb(150, 20,  20));
    private static readonly IBrush BossOutline  = new SolidColorBrush(Color.FromRgb(100, 0,   0));
    private static readonly IBrush BossMid      = new SolidColorBrush(Color.FromRgb(255, 130, 0));
    private static readonly IBrush BossCore     = new SolidColorBrush(Color.FromRgb(255, 230, 80));
    private static readonly IBrush BossEye      = new SolidColorBrush(Color.FromRgb(255, 255, 200));
    private static readonly IBrush BossBeam     = new SolidColorBrush(Color.FromRgb(255, 100, 100));
    private static readonly IBrush BossHighlight = new SolidColorBrush(Color.FromRgb(255, 255, 255));
    private static readonly IBrush TrailHot     = new SolidColorBrush(Color.FromRgb(255, 200, 80));

    // ─── Player palette ──────────────────────────────────────────────────────
    private static readonly IBrush PlayerHull       = new SolidColorBrush(Color.FromRgb(50,  210, 90));
    private static readonly IBrush PlayerHullDark   = new SolidColorBrush(Color.FromRgb(30,  160, 60));
    private static readonly IBrush PlayerHullLight  = new SolidColorBrush(Color.FromRgb(120, 240, 150));
    private static readonly IBrush PlayerOutline    = new SolidColorBrush(Color.FromRgb(20,  100, 40));
    private static readonly IBrush PlayerCockpit    = new SolidColorBrush(Color.FromRgb(80,  210, 255));
    private static readonly IBrush PlayerCockpitDark = new SolidColorBrush(Color.FromRgb(30,  120, 190));
    private static readonly IBrush PlayerEngine     = new SolidColorBrush(Color.FromRgb(255, 110, 0));
    private static readonly IBrush PlayerEngineGlow = new SolidColorBrush(Color.FromRgb(255, 200, 80));
    private static readonly IBrush PlayerEngineHot  = new SolidColorBrush(Color.FromRgb(255, 250, 220));
    private static readonly IBrush PlayerAccent     = new SolidColorBrush(Color.FromRgb(100, 255, 130));
    private static readonly IBrush PlayerRedLight   = new SolidColorBrush(Color.FromRgb(255, 60,  60));
    private static readonly IBrush PlayerGreenLight = new SolidColorBrush(Color.FromRgb(100, 255, 130));

    private static readonly IBrush MiniShipColor = new SolidColorBrush(Color.FromRgb(50,  210, 90));

    private const double PlayerPixelScale = 2.0;

    // ─── Bullet palette ──────────────────────────────────────────────────────
    private static readonly IBrush PlayerBulletHead   = new SolidColorBrush(Color.FromRgb(255, 255, 220));
    private static readonly IBrush PlayerBulletCore   = new SolidColorBrush(Color.FromRgb(150, 255, 150));
    private static readonly IBrush PlayerBulletTail   = new SolidColorBrush(Color.FromRgb(40,  180, 40));
    private static readonly IBrush EnemyBulletHead    = new SolidColorBrush(Color.FromRgb(255, 255, 220));
    private static readonly IBrush EnemyBulletCore    = new SolidColorBrush(Color.FromRgb(255, 80,  80));
    private static readonly IBrush EnemyBulletTail    = new SolidColorBrush(Color.FromRgb(180, 30,  30));

    // ─── Explosion palette ───────────────────────────────────────────────────
    private static readonly IBrush ExpWhite  = Brushes.White;
    private static readonly IBrush ExpYellow = new SolidColorBrush(Color.FromRgb(255, 230, 60));
    private static readonly IBrush ExpOrange = new SolidColorBrush(Color.FromRgb(255, 130, 20));
    private static readonly IBrush ExpRed    = new SolidColorBrush(Color.FromRgb(200, 40,  10));
    private static readonly IBrush ExpSmoke  = new SolidColorBrush(Color.FromRgb(80,  80,  80));

    // ─── Pixel-art sprite maps ────────────────────────────────────────────────
    // '#' = main color, 'o' = outline/shadow, 'd' = dark shade, 'h' = highlight, '.' = empty.
    // 8×8 chars → rendered at 3×3 px = 24×24 total.

    private const double Ps = 3.0;

    // Enemies remain 8×8 logical pixels rendered at 3× → 24×24.
    // '.' empty, '#' main, '=' dark/detail, 'o' outline, 'h' highlight,
    // 'a' antenna, 'e' eye, 'b' body, 'w' white stripe, 'y' boss eye,
    // 'm' boss mouth/beam, 'c' captured ship cyan.

    // BEE — frame 0 (wings spread)
    private static readonly string[] BeePx0 =
    {
        ".a....a.",
        ".#====#.",
        "##====##",
        "#=ebbe=#",
        "###bb###",
        ".##==##.",
        ".#=##=#.",
        "..#..#..",
    };
    // BEE — frame 1 (wings flap up)
    private static readonly string[] BeePx1 =
    {
        "a......a",
        "#=....=#",
        "##====##",
        ".=ebbe=.",
        ".##bb##.",
        ".#=##=#.",
        "..####..",
        "........",
    };

    // BUTTERFLY — frame 0 (wings spread)
    private static readonly string[] ButterflyPx0 =
    {
        "o.hddh.o",
        "#hdwwdh#",
        "###ww###",
        ".=bwwb=.",
        ".=bwwb=.",
        "###ww###",
        "#hdwwdh#",
        "o.hddh.o",
    };
    // BUTTERFLY — frame 1 (wings flap)
    private static readonly string[] ButterflyPx1 =
    {
        "...aa...",
        ".h####h.",
        "##dwwd##",
        "..bwwb..",
        "..bwwb..",
        "##dwwd##",
        ".h####h.",
        "........",
    };

    // BOSS — frame 0
    private static readonly string[] BossPx0 =
    {
        "...##...",
        "..#mm#..",
        ".##yy##.",
        "##h##h##",
        "#m####m#",
        "##o##o##",
        ".#mmmm#.",
        "..####..",
    };
    // BOSS — frame 1
    private static readonly string[] BossPx1 =
    {
        "...##...",
        ".##mm##.",
        "##hyyh##",
        "#h####h#",
        "#m####m#",
        "##o##o##",
        ".#mmmm#.",
        "..####..",
    };

    private static readonly Dictionary<char, IBrush> BeePalette = new()
    {
        ['#'] = BeeBody,
        ['='] = BeeDark,
        ['o'] = BeeWing,
        ['a'] = BeeAntenna,
        ['e'] = BeeEye,
        ['b'] = BeeBody,
    };

    private static readonly Dictionary<char, IBrush> ButterflyPalette = new()
    {
        ['#'] = BflyWing,
        ['d'] = BflyWingDark,
        ['h'] = BflyWingLight,
        ['o'] = BflyWingDark,
        ['w'] = BflyStripe,
        ['b'] = BflyBody,
        ['a'] = BflyBodyDark,
    };

    private static readonly Dictionary<char, IBrush> BossPalette = new()
    {
        ['#'] = BossOuter,
        ['d'] = BossDark,
        ['o'] = BossOutline,
        ['m'] = BossMid,
        ['h'] = BossHighlight,
        ['y'] = BossEye,
    };

    // ─── Public draw methods ─────────────────────────────────────────────────

    public static void DrawEnemy(DrawingContext ctx, Enemy enemy, int frame)
    {
        DrawEnemyTrail(ctx, enemy);
        switch (enemy.Type)
        {
            case EnemyType.Bee:        DrawBee(ctx, enemy, frame);        break;
            case EnemyType.Butterfly:  DrawButterfly(ctx, enemy, frame);  break;
            case EnemyType.BossGalaga: DrawBossGalaga(ctx, enemy, frame); break;
        }
    }

    private static void DrawBee(DrawingContext ctx, Enemy enemy, int frame)
    {
        var px = frame == 0 ? BeePx0 : BeePx1;
        DrawPixels(ctx, px, enemy.X, enemy.Y, BeePalette, Ps);

        // Antenna tips in frame 0, nascoste in frame 1 per dinamismo
        if (frame == 0)
        {
            ctx.FillRectangle(BeeAntenna, new Rect(enemy.X + 1 * Ps, enemy.Y - Ps, Ps, Ps));
            ctx.FillRectangle(BeeAntenna, new Rect(enemy.X + 6 * Ps, enemy.Y - Ps, Ps, Ps));
        }
    }

    private static void DrawButterfly(DrawingContext ctx, Enemy enemy, int frame)
    {
        var px = frame == 0 ? ButterflyPx0 : ButterflyPx1;
        DrawPixels(ctx, px, enemy.X, enemy.Y, ButterflyPalette, Ps);

        // Antennae only in frame 1
        if (frame == 1)
        {
            ctx.FillRectangle(BflyWingLight, new Rect(enemy.X + 3 * Ps, enemy.Y - Ps, Ps, Ps));
            ctx.FillRectangle(BflyWingLight, new Rect(enemy.X + 4 * Ps, enemy.Y - Ps, Ps, Ps));
        }
    }

    private static void DrawBossGalaga(DrawingContext ctx, Enemy enemy, int frame)
    {
        var px = frame == 0 ? BossPx0 : BossPx1;
        DrawPixels(ctx, px, enemy.X, enemy.Y, BossPalette, Ps);

        // Captured player fighter held beneath the boss
        if (enemy.CarriesCapturedShip)
            DrawCapturedShip(ctx, enemy.X + 3, enemy.Y + 26);
    }

    private static void DrawCapturedShip(DrawingContext ctx, double x, double y)
    {
        // Tractor-beam tint (cyan) so it reads as "captured"
        ctx.FillRectangle(PlayerCockpit,    new Rect(x + 6, y,      4, 11));
        ctx.FillRectangle(PlayerCockpit,    new Rect(x,     y + 5,  16, 6));
        ctx.FillRectangle(PlayerAccent,     new Rect(x + 7, y + 2,  2, 3));
        ctx.FillRectangle(PlayerEngineGlow, new Rect(x + 1, y + 9,  3, 3));
        ctx.FillRectangle(PlayerEngineGlow, new Rect(x + 12, y + 9, 3, 3));
    }

    private static void DrawEnemyTrail(DrawingContext ctx, Enemy enemy)
    {
        if (enemy.State is not (EnemyState.Diving or EnemyState.Returning)) return;

        using var opacity = ctx.PushOpacity(0.35);
        double len = enemy.State == EnemyState.Diving ? 10 : 6;
        double width = enemy.Width / 3.0;
        double cx = enemy.X + enemy.Width / 2.0 - width / 2.0;
        double cy = enemy.Y - len;
        ctx.FillRectangle(TrailHot, new Rect(cx, cy, width, len));
    }

    // 16×16 pixel-art player sprite maps.
    // # = hull, = = dark hull, o = outline, c = cockpit, d = cockpit dark,
    // l = green nav light, r = red nav light, e = engine, g = engine glow,
    // f = flame, w = white flame core.
    private static readonly string[] PlayerPx0 =
    {
        "................",
        ".......cc.......",
        ".......cc.......",
        "......cccc......",
        ".....##cc##.....",
        "....###==###....",
        "...####==####...",
        "...####==####...",
        "..#####==#####..",
        "..##gge==egg##..",
        ".=#o=ge==eg=o=#.",
        ".=#ooe====eoo=#.",
        ".=#ooe====eoo=#.",
        "..=ooe=ff=eoo=..",
        "..=oof=ww=foo=..",
        "...fww=ww=wwf...",
    };

    private static readonly string[] PlayerPx1 =
    {
        "................",
        ".......cc.......",
        ".......cc.......",
        "......cccc......",
        ".....##cc##.....",
        "....###==###....",
        "...####==####...",
        "...####==####...",
        "..#####==#####..",
        "..##gge==egg##..",
        ".=#o=ge==eg=o=#.",
        ".=#ooe====eoo=#.",
        ".=#ooe====eoo=#.",
        "..=ooe=ff=eoo=..",
        "..=oof=ff=foo=..",
        "...fff=ff=fff...",
    };

    private static readonly Dictionary<char, IBrush> PlayerPalette0 = new()
    {
        ['#'] = PlayerHull,
        ['='] = PlayerHullDark,
        ['o'] = PlayerOutline,
        ['c'] = PlayerCockpit,
        ['d'] = PlayerCockpitDark,
        ['e'] = PlayerEngine,
        ['g'] = PlayerEngineGlow,
        ['f'] = PlayerEngineHot,
        ['w'] = Brushes.White,
        ['l'] = PlayerGreenLight,
        ['r'] = PlayerRedLight,
    };

    private static readonly Dictionary<char, IBrush> PlayerPalette1 = new()
    {
        ['#'] = PlayerHull,
        ['='] = PlayerHullDark,
        ['o'] = PlayerOutline,
        ['c'] = PlayerCockpit,
        ['d'] = PlayerCockpitDark,
        ['e'] = PlayerEngine,
        ['g'] = PlayerEngineGlow,
        ['f'] = PlayerEngineGlow,
        ['w'] = PlayerEngineHot,
        ['l'] = PlayerGreenLight,
        ['r'] = PlayerRedLight,
    };

    // ─── Player ship ─────────────────────────────────────────────────────────
    public static void DrawPlayer(DrawingContext ctx, Player player, int frame = 0)
    {
        DrawPlayerAt(ctx, player.X, player.Y, frame, player.MoveLeft, player.MoveRight);
    }

    private static void DrawPlayerAt(
        DrawingContext ctx, double x, double y, int frame,
        bool moveLeft = false, bool moveRight = false)
    {
        var pixels  = frame == 0 ? PlayerPx0 : PlayerPx1;
        var palette = frame == 0 ? PlayerPalette0 : PlayerPalette1;

        // Motion blur / engine trail — draw a faint copy shifted opposite to movement.
        if (moveLeft)
        {
            using var ghost = ctx.PushOpacity(0.25);
            DrawPixels(ctx, pixels, x + PlayerPixelScale * 2, y, palette, PlayerPixelScale);
        }
        else if (moveRight)
        {
            using var ghost = ctx.PushOpacity(0.25);
            DrawPixels(ctx, pixels, x - PlayerPixelScale * 2, y, palette, PlayerPixelScale);
        }

        DrawPixels(ctx, pixels, x, y, palette, PlayerPixelScale);

        // Blinking running lights on the wing tips (alternate each frame).
        if (frame == 0)
        {
            ctx.FillRectangle(PlayerGreenLight, new Rect(x + 1 * PlayerPixelScale, y + 10 * PlayerPixelScale,
                PlayerPixelScale, PlayerPixelScale));
            ctx.FillRectangle(PlayerRedLight, new Rect(x + 14 * PlayerPixelScale, y + 10 * PlayerPixelScale,
                PlayerPixelScale, PlayerPixelScale));
        }
    }

    public static void DrawPlayer(DrawingContext ctx, double x, double y, int frame)
        => DrawPlayerAt(ctx, x, y, frame);

    // 8×6 mini player for the HUD
    private static readonly string[] MiniPlayerPx =
    {
        "...cc...",
        "..cccc..",
        ".##cc##.",
        "###==###",
        "##geegg#",
        ".=oeo=..",
    };

    private static readonly Dictionary<char, IBrush> MiniPlayerPalette = new()
    {
        ['#'] = PlayerHull,
        ['='] = PlayerHullDark,
        ['c'] = PlayerCockpit,
        ['e'] = PlayerEngine,
        ['g'] = PlayerEngineGlow,
        ['o'] = PlayerOutline,
        ['.'] = Brushes.Transparent,
    };

    // ─── Mini player (HUD) ───────────────────────────────────────────────────
    public static void DrawMiniPlayer(DrawingContext ctx, double x, double y)
    {
        DrawPixels(ctx, MiniPlayerPx, x, y, MiniPlayerPalette, 2.0);
    }

    // ─── Bullet ─────────────────────────────────────────────────────────────-
    public static void DrawBullet(DrawingContext ctx, Bullet bullet)
    {
        bool isPlayer = bullet.Owner == BulletOwner.Player;
        var head = isPlayer ? PlayerBulletHead : EnemyBulletHead;
        var core = isPlayer ? PlayerBulletCore : EnemyBulletCore;
        var tail = isPlayer ? PlayerBulletTail : EnemyBulletTail;

        double cx = bullet.X + bullet.Width / 2.0;
        double cy = bullet.Y;
        double w  = bullet.Width;

        // Tail
        ctx.FillRectangle(tail, new Rect(cx - w / 2, cy + bullet.Height * 0.35, w, bullet.Height * 0.65));
        // Core
        ctx.FillRectangle(core, new Rect(cx - w / 2 + 1, cy + bullet.Height * 0.15, w - 2, bullet.Height * 0.65));
        // Head
        ctx.FillRectangle(head, new Rect(cx - w / 2 + 1, cy, w - 2, bullet.Height * 0.35));
    }

    // ─── Explosion ───────────────────────────────────────────────────────────
    public static void DrawExplosion(DrawingContext ctx, Explosion exp)
    {
        double p = exp.Progress;

        // Shock ring
        if (p < 0.55)
        {
            using var ringOpacity = ctx.PushOpacity(0.6 * (1 - p / 0.55));
            double ringR = exp.Radius * p * 1.4;
            double ringW = Math.Max(1.0, 3 * (1 - p / 0.55));
            ctx.DrawEllipse(null, new Pen(ExpYellow, ringW), new Point(exp.CenterX, exp.CenterY), ringR, ringR);
        }

        IBrush brush = p < 0.20 ? ExpWhite
                     : p < 0.45 ? ExpYellow
                     : p < 0.70 ? ExpOrange
                     :            ExpRed;

        double radius   = exp.Radius * p;
        int particleCount = 12;

        for (int i = 0; i < particleCount; i++)
        {
            double baseAngle = i * 2.0 * Math.PI / particleCount;
            double angle = baseAngle + p * (i % 2 == 0 ? 0.9 : -0.9);
            double dist = radius * (0.6 + 0.4 * ((i * 7) % 5 / 4.0));
            double px = exp.CenterX + Math.Cos(angle) * dist;
            double py = exp.CenterY + Math.Sin(angle) * dist;
            double partSize = Math.Max(1.5, 5.0 * (1.0 - p));
            ctx.FillRectangle(brush, new Rect(px - partSize / 2, py - partSize / 2, partSize, partSize));
        }

        // Inner flash + smoke core
        if (p < 0.25)
        {
            double flash = 10 * (1 - p / 0.25);
            ctx.FillRectangle(ExpWhite, new Rect(exp.CenterX - flash / 2, exp.CenterY - flash / 2, flash, flash));
        }
        else if (p > 0.55)
        {
            using var smokeOpacity = ctx.PushOpacity(0.5 * (1 - (p - 0.55) / 0.45));
            double smokeR = exp.Radius * 0.5 * p;
            ctx.FillRectangle(ExpSmoke,
                new Rect(exp.CenterX - smokeR / 2, exp.CenterY - smokeR / 2, smokeR, smokeR));
        }
    }

    // ─── Helper ──────────────────────────────────────────────────────────────

    private static void DrawPixels(DrawingContext ctx, string[] pixels,
        double ox, double oy, Dictionary<char, IBrush> palette, double scale = Ps)
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
                    new Rect(ox + col * scale, oy + row * scale, scale, scale));
            }
        }
    }
}
