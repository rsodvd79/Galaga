using System.Globalization;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media;
using Avalonia.Media.Imaging;
using Avalonia.Threading;
using Avalonia.VisualTree;
using Galaga.Audio;
using Galaga.Engine;
using Galaga.Entities;

namespace Galaga.Views;

public class GameCanvas : Control
{
    // ─── Brushes (only what GameCanvas itself uses) ──────────────────────────
    private static readonly IBrush Black  = Brushes.Black;
    private static readonly IBrush White  = Brushes.White;
    private static readonly IBrush Cyan   = Brushes.Cyan;
    private static readonly IBrush Yellow = Brushes.Yellow;
    private static readonly IBrush Red    = Brushes.OrangeRed;
    private static readonly IBrush Green  = Brushes.LimeGreen;
    private static readonly IBrush Gray   = Brushes.DimGray;

    private static readonly IBrush TitleGlow = new RadialGradientBrush
    {
        Center    = new RelativePoint(0.5, 0.5, RelativeUnit.Relative),
        RadiusX   = new RelativeScalar(0.5, RelativeUnit.Relative),
        RadiusY   = new RelativeScalar(0.5, RelativeUnit.Relative),
        GradientStops = new GradientStops
        {
            new GradientStop { Color = Color.FromRgb(255, 170, 60), Offset = 0.0 },
            new GradientStop { Color = Color.FromRgb(255, 80, 160), Offset = 0.4 },
            new GradientStop { Color = Colors.Transparent,           Offset = 1.0 },
        }
    };

    private static readonly IBrush[] StarBrushes =
    {
        new SolidColorBrush(Color.FromRgb(90,  90,  90)),   // distant / dim
        new SolidColorBrush(Color.FromRgb(170, 170, 170)), // mid
        new SolidColorBrush(Color.FromRgb(255, 255, 255)), // near / bright
        new SolidColorBrush(Color.FromRgb(200, 220, 255)), // near / blue-ish
    };

    private static readonly double[] StarSpeeds =
    {
        15.0,
        35.0,
        60.0,
        90.0,
    };

    // ─── Game state ─────────────────────────────────────────────────────────
    private readonly GameState   _state;
    private readonly GameEngine  _engine;
    private readonly SoundPlayer _sound;
    private readonly bool        _autoScreenshotMode;

    // ─── Loop ───────────────────────────────────────────────────────────────
    private readonly DispatcherTimer _timer;
    private DateTime _lastTick;

    // ─── Animation ──────────────────────────────────────────────────────────
    private int _tickCount;
    private int AnimFrame => (_tickCount / 8) % 2; // flips every 8 ticks ≈ 7.5 Hz

    // ─── Starfield ──────────────────────────────────────────────────────────
    private (double x, double y, int level)[] _stars = [];

    // ─── Menu decoration (attract-mode enemy formation) ──────────────────────
    private readonly List<Enemy> _menuDecor = new();

    private void InitStars()
    {
        var rng = Random.Shared;
        _stars  = new (double, double, int)[110];
        for (int i = 0; i < _stars.Length; i++)
            _stars[i] = (rng.NextDouble() * GameState.GameWidth,
                         rng.NextDouble() * GameState.GameHeight,
                         rng.Next(StarBrushes.Length));
    }

    private void UpdateStars(double elapsed)
    {
        var h = GameState.GameHeight;
        var rng = Random.Shared;
        for (int i = 0; i < _stars.Length; i++)
        {
            var speed = StarSpeeds[_stars[i].level];
            _stars[i].y += speed * elapsed;
            if (_stars[i].y >= h)
            {
                _stars[i].y = 0;
                _stars[i].x = rng.NextDouble() * GameState.GameWidth;
            }
        }
    }

    private void InitMenuDecor()
    {
        var defs = new (EnemyType type, double x)[]
        {
            (EnemyType.BossGalaga, 400),
            (EnemyType.Butterfly, 312),
            (EnemyType.Butterfly, 488),
            (EnemyType.Bee,        224),
            (EnemyType.Bee,        576),
        };
        foreach (var (type, x) in defs)
        {
            var e = new Enemy(type, x, 118, 0) { Y = 118 };
            _menuDecor.Add(e);
        }
    }

    public GameCanvas()
    {
        _autoScreenshotMode = Environment.GetCommandLineArgs()
            .Any(a => a == "--screenshots");
        _state  = new GameState();
        _engine = new GameEngine(_state);
        _sound  = new SoundPlayer();

        _state.HighScore = HighScoreStore.Load();

        InitStars();
        InitMenuDecor();

        Focusable = true;

        _lastTick = DateTime.UtcNow;
        _timer    = new DispatcherTimer { Interval = TimeSpan.FromMilliseconds(16) };
        _timer.Tick += OnTick;
        _timer.Start();
    }

    // ─── Game loop ──────────────────────────────────────────────────────────
    private void OnTick(object? sender, EventArgs e)
    {
        var now   = DateTime.UtcNow;
        double dt = Math.Min((now - _lastTick).TotalSeconds, 0.05);
        _lastTick = now;
        _tickCount++;

        UpdateStars(dt);
        _engine.Tick(dt);
        _engine.UpdateStageClear(dt);

        while (_state.PendingSounds.TryDequeue(out var sfx))
            _sound.Play(sfx);

        if (_autoScreenshotMode) HandleAutoScreenshot();

        InvalidateVisual();
    }

    // ─── Input ──────────────────────────────────────────────────────────────
    protected override void OnKeyDown(KeyEventArgs e)
    {
        base.OnKeyDown(e);
        switch (e.Key)
        {
            case Key.Left:
            case Key.A:
                _state.Player.MoveLeft = true; break;

            case Key.Right:
            case Key.D:
                _state.Player.MoveRight = true; break;

            case Key.Space:
                if (_state.Phase is GamePhase.Menu or GamePhase.GameOver)
                    _state.Reset();
                else if (_state.Phase == GamePhase.Playing)
                    _state.ShootPressed = true;
                break;

            case Key.P:
                if (_state.Phase == GamePhase.Playing)
                    _state.Phase = GamePhase.Paused;
                else if (_state.Phase == GamePhase.Paused)
                    _state.Phase = GamePhase.Playing;
                break;

            case Key.Escape:
                if (_state.Phase != GamePhase.Menu)
                {
                    _state.Phase = GamePhase.Menu;
                    _state.ShootPressed = false;
                }
                break;

            case Key.F12:
                SaveScreenshot();
                break;
        }
    }

    protected override void OnKeyUp(KeyEventArgs e)
    {
        base.OnKeyUp(e);
        switch (e.Key)
        {
            case Key.Left:  case Key.A: _state.Player.MoveLeft  = false; break;
            case Key.Right: case Key.D: _state.Player.MoveRight = false; break;
        }
    }

    // ─── Auto-screenshot mode ────────────────────────────────────────────────
    private bool _menuShotDone;
    private bool _playShotDone;
    private void HandleAutoScreenshot()
    {
        // Tick 120 (~2s) → capture menu, then start game
        if (!_menuShotDone && _tickCount >= 120 && _state.Phase == GamePhase.Menu)
        {
            _menuShotDone = true;
            Dispatcher.UIThread.Post(() =>
            {
                SaveScreenshot();
                // Start game
                _state.Reset();
                _state.Phase = GamePhase.Playing;
            });
        }
        // Tick 300 (~3s after game start) → capture gameplay, then close
        if (!_playShotDone && _menuShotDone && _state.Phase == GamePhase.Playing
            && _tickCount >= 300)
        {
            _playShotDone = true;
            Dispatcher.UIThread.Post(() =>
            {
                SaveScreenshot();
                (TopLevel.GetTopLevel(this) as Window)?.Close();
            });
        }
    }

    // ─── Cleanup ────────────────────────────────────────────────────────────
    protected override void OnDetachedFromVisualTree(VisualTreeAttachmentEventArgs e)
    {
        base.OnDetachedFromVisualTree(e);
        _sound.Dispose();
    }

    // ─── Screenshot ─────────────────────────────────────────────────────────
    private void SaveScreenshot()
    {
        try
        {
            var dir  = Path.GetFullPath(
                Path.Combine(Directory.GetCurrentDirectory(), "screenshots"));
            Directory.CreateDirectory(dir);
            var phase = _state.Phase.ToString().ToLowerInvariant();
            var path  = Path.GetFullPath(Path.Combine(dir, $"galaga_{phase}.png"));

            var size   = new PixelSize((int)GameState.GameWidth, (int)GameState.GameHeight);
            var bitmap = new RenderTargetBitmap(size, new Vector(96, 96));
            bitmap.Render(this);
            bitmap.Save(path);
            Console.WriteLine($"Screenshot saved: {path}");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Screenshot failed: {ex.Message}");
        }
    }

    // ─── Rendering ──────────────────────────────────────────────────────────
    public override void Render(DrawingContext ctx)
    {
        // Background
        ctx.FillRectangle(Black, new Rect(0, 0, GameState.GameWidth, GameState.GameHeight));

        // Stars — size and opacity vary by depth level
        foreach (var (sx, sy, lvl) in _stars)
        {
            double sz = lvl switch { 3 => 3, 2 => 2, _ => 1 };
            double opacity = lvl switch { 3 => 1.0, 2 => 0.9, 1 => 0.65, _ => 0.45 };
            using var starOpacity = ctx.PushOpacity(opacity);
            ctx.FillRectangle(StarBrushes[lvl], new Rect(sx, sy, sz, sz));
        }

        if (_state.Phase == GamePhase.Menu)
        {
            DrawMenu(ctx);
            return;
        }

        DrawHud(ctx);
        DrawEntities(ctx);
        DrawOverlay(ctx);
    }

    private void DrawEntities(DrawingContext ctx)
    {
        // Enemies
        foreach (var enemy in _state.Formation.Enemies.Where(e => e.IsAlive))
            SpriteRenderer.DrawEnemy(ctx, enemy, AnimFrame);

        // Player (blinks while invulnerable after respawn)
        if (_state.Player.IsAlive)
        {
            bool blink = _state.Player.IsInvulnerable &&
                         Math.Floor(_state.Player.InvulnerabilityRemaining * 6) % 2 == 0;
            if (!blink)
            {
                SpriteRenderer.DrawPlayer(ctx, _state.Player, AnimFrame);
                if (_state.Player.HasDualFighter)
                    SpriteRenderer.DrawPlayer(ctx, _state.Player.X + _state.Player.Width,
                        _state.Player.Y, AnimFrame);
            }
        }

        // Bullets
        foreach (var bullet in _state.Bullets)
            SpriteRenderer.DrawBullet(ctx, bullet);

        // Explosions (drawn on top)
        foreach (var exp in _state.Explosions)
            SpriteRenderer.DrawExplosion(ctx, exp);
    }

    private void DrawHud(DrawingContext ctx)
    {
        DrawText(ctx, $"SCORE {_state.Score}", 10, 10, 16, White);
        DrawText(ctx, $"HI    {_state.HighScore}", 300, 10, 16, Cyan);
        DrawText(ctx, $"LEVEL {_state.Level}", 660, 10, 16, Yellow);

        for (int i = 0; i < _state.Player.Lives; i++)
            SpriteRenderer.DrawMiniPlayer(ctx, 10 + i * 22, 578);

        if (_state.Player.HasDualFighter)
        {
            int dx = 10 + _state.Player.Lives * 22 + 10;
            SpriteRenderer.DrawMiniPlayer(ctx, dx,      578);
            SpriteRenderer.DrawMiniPlayer(ctx, dx + 18, 578);
            DrawText(ctx, "DUAL", dx + 38, 580, 12, Cyan);
        }
    }

    private void DrawGalagaTitle(DrawingContext ctx)
    {
        const string text = "GALAGA";
        const double size = 52;
        const double y = 190;

        var colors = new IBrush[]
        {
            new SolidColorBrush(Color.FromRgb(255, 80, 160)),
            new SolidColorBrush(Color.FromRgb(255, 215, 0)),
            new SolidColorBrush(Color.FromRgb(50, 205, 50)),
            new SolidColorBrush(Color.FromRgb(255, 215, 0)),
            new SolidColorBrush(Color.FromRgb(255, 80, 160)),
            new SolidColorBrush(Color.FromRgb(255, 215, 0)),
        };

        var widths = new double[text.Length];
        double totalWidth = 0;
        for (int i = 0; i < text.Length; i++)
        {
            var ft = new FormattedText(text[i].ToString(), CultureInfo.InvariantCulture,
                FlowDirection.LeftToRight, Typeface.Default, size, Brushes.White);
            widths[i] = ft.Width;
            totalWidth += ft.Width;
        }

        double startX = (GameState.GameWidth - totalWidth) / 2;
        double shadowOffset = 3;

        foreach (var shadowColor in new[] { Color.FromRgb(60, 0, 0), Color.FromRgb(120, 30, 0) })
        {
            double sx = startX + shadowOffset;
            var shadow = new SolidColorBrush(shadowColor);
            for (int i = 0; i < text.Length; i++)
            {
                var ft = new FormattedText(text[i].ToString(), CultureInfo.InvariantCulture,
                    FlowDirection.LeftToRight, Typeface.Default, size, shadow);
                ctx.DrawText(ft, new Point(sx, y + shadowOffset));
                sx += widths[i];
            }
        }

        double cx = startX;
        for (int i = 0; i < text.Length; i++)
        {
            var ft = new FormattedText(text[i].ToString(), CultureInfo.InvariantCulture,
                FlowDirection.LeftToRight, Typeface.Default, size, colors[i]);
            ctx.DrawText(ft, new Point(cx, y));
            cx += widths[i];
        }
    }

    private void DrawMenu(DrawingContext ctx)
    {
        // Pulsing glow behind the title
        double pulse = 0.5 + 0.5 * Math.Sin(_tickCount * 0.05);
        double glowR = 130 + pulse * 26;
        ctx.DrawEllipse(TitleGlow, null, new Point(GameState.GameWidth / 2, 214), glowR, glowR * 0.55);

        // Drifting attract-mode enemy formation
        double sway = Math.Sin(_tickCount * 0.03) * 26;
        int i = 0;
        foreach (var e in _menuDecor)
        {
            e.X = e.FormationX + sway;
            e.Y = 118 + Math.Sin(_tickCount * 0.05 + i) * 6;
            i++;
            SpriteRenderer.DrawEnemy(ctx, e, AnimFrame);
        }

        DrawGalagaTitle(ctx);

        // Blinking prompt
        if (Math.Floor(_tickCount / 30.0) % 2 == 0)
            DrawText(ctx, "PRESS SPACE TO START", null, 290, 24, White, centered: true);

        DrawText(ctx, $"HIGH SCORE  {_state.HighScore}", null, 350, 18, Cyan,    centered: true);
        DrawText(ctx, "← → / A D  MOVE    SPACE  SHOOT    P  PAUSE",
                                                        null, 410, 13, Gray,    centered: true);
    }

    private void DrawOverlay(DrawingContext ctx)
    {
        switch (_state.Phase)
        {
            case GamePhase.GameOver:
                DrawText(ctx, "GAME OVER",           null, 250, 42, Red,    centered: true);
                DrawText(ctx, "PRESS SPACE TO RETRY",null, 310, 20, White,  centered: true);
                break;
            case GamePhase.StageClear:
                DrawText(ctx, $"STAGE {_state.Level} CLEAR!", null, 260, 32, Cyan, centered: true);
                break;
            case GamePhase.Paused:
                DrawText(ctx, "PAUSED", null, 270, 32, Yellow, centered: true);
                break;
        }
    }

    // ─── Text helper ─────────────────────────────────────────────────────────
    private static readonly IBrush TextShadow = new SolidColorBrush(Colors.Black);

    private static void DrawText(
        DrawingContext ctx,
        string text,
        double? x,
        double y,
        double size,
        IBrush brush,
        bool centered = false)
    {
        var ft = new FormattedText(
            text,
            CultureInfo.InvariantCulture,
            FlowDirection.LeftToRight,
            Typeface.Default,
            size,
            brush);

        double px = centered ? (GameState.GameWidth - ft.Width) / 2 : x ?? 0;

        // Drop shadow for readability over stars/explosions.
        using (ctx.PushOpacity(0.7))
        {
            var shadowFt = new FormattedText(
                text,
                CultureInfo.InvariantCulture,
                FlowDirection.LeftToRight,
                Typeface.Default,
                size,
                TextShadow);
            ctx.DrawText(shadowFt, new Point(px + 1, y + 1));
        }

        ctx.DrawText(ft, new Point(px, y));
    }
}
