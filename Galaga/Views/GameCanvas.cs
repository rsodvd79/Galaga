using System.Globalization;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media;
using Avalonia.Media.Imaging;
using Avalonia.Threading;
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

    private static readonly IBrush[] StarBrushes =
    {
        new SolidColorBrush(Color.FromRgb(120, 120, 120)),
        new SolidColorBrush(Color.FromRgb(190, 190, 190)),
        new SolidColorBrush(Color.FromRgb(255, 255, 255)),
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

    // ─── Starfield (deterministic seed) ─────────────────────────────────────
    private readonly (double x, double y, int level)[] _stars;

    public GameCanvas()
    {
        _autoScreenshotMode = Environment.GetCommandLineArgs()
            .Any(a => a == "--screenshots");
        _state  = new GameState();
        _engine = new GameEngine(_state);
        _sound  = new SoundPlayer();

        var rng = new Random(42);
        _stars  = new (double, double, int)[90];
        for (int i = 0; i < _stars.Length; i++)
            _stars[i] = (rng.NextDouble() * GameState.GameWidth,
                         rng.NextDouble() * GameState.GameHeight,
                         rng.Next(3));

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
                else
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
                    _state.Phase = GamePhase.Menu;
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

        // Stars
        foreach (var (sx, sy, lvl) in _stars)
        {
            double sz = lvl == 2 ? 2 : 1;
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

        // Player
        if (_state.Player.IsAlive)
            SpriteRenderer.DrawPlayer(ctx, _state.Player.X, _state.Player.Y);

        // Bullets
        foreach (var bullet in _state.Bullets)
        {
            var brush = bullet.Owner == BulletOwner.Player ? Green : Red;
            ctx.FillRectangle(brush, new Rect(bullet.X, bullet.Y, bullet.Width, bullet.Height));
        }

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
    }

    private void DrawMenu(DrawingContext ctx)
    {
        DrawText(ctx, "GALAGA",                         null, 190, 52, Yellow,  centered: true);
        DrawText(ctx, "PRESS SPACE TO START",           null, 290, 24, White,   centered: true);
        DrawText(ctx, $"HIGH SCORE  {_state.HighScore}", null, 350, 18, Cyan,   centered: true);
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
        ctx.DrawText(ft, new Point(px, y));
    }
}
