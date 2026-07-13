using Silk.NET.OpenAL;

namespace Galaga.Audio;

public enum SoundEffect { Shoot, EnemyExplosion, PlayerDeath, StageClear, Capture, Rescue }

/// <summary>
/// Synthesizes and plays retro arcade sounds via OpenAL.
/// All initialization is wrapped in try/catch — the game runs silently if audio
/// is unavailable.
/// </summary>
public sealed unsafe class SoundPlayer : IDisposable
{
    private readonly AL?         _al;
    private readonly ALContext?  _alc;
    private readonly Device*     _device;
    private readonly Context*    _ctx;

    private uint _shootBuf;
    private uint _explosionBuf;
    private uint _playerDeathBuf;
    private uint _stageClearBuf;
    private uint _captureBuf;
    private uint _rescueBuf;

    // Small source pool for overlapping playback
    private readonly uint[] _sources = new uint[8];
    private int _nextSource;

    public bool IsAvailable { get; private set; }

    private const int SampleRate = 22050;

    public SoundPlayer()
    {
        try
        {
            _al  = AL.GetApi();
            _alc = ALContext.GetApi();

            _device = _alc.OpenDevice(null);
            if (_device == null) return;

            _ctx = _alc.CreateContext(_device, (int*)null);
            _alc.MakeContextCurrent(_ctx);

            for (int i = 0; i < _sources.Length; i++)
                _sources[i] = _al.GenSource();

            _shootBuf       = CreateBuffer(_al, SynthShoot());
            _explosionBuf   = CreateBuffer(_al, SynthExplosion());
            _playerDeathBuf = CreateBuffer(_al, SynthPlayerDeath());
            _stageClearBuf  = CreateBuffer(_al, SynthStageClear());
            _captureBuf     = CreateBuffer(_al, SynthCapture());
            _rescueBuf      = CreateBuffer(_al, SynthRescue());

            IsAvailable = true;
        }
        catch
        {
            IsAvailable = false;
        }
    }

    public void Play(SoundEffect effect)
    {
        if (!IsAvailable || _al is null) return;

        uint buf = effect switch
        {
            SoundEffect.Shoot          => _shootBuf,
            SoundEffect.EnemyExplosion => _explosionBuf,
            SoundEffect.PlayerDeath    => _playerDeathBuf,
            SoundEffect.StageClear     => _stageClearBuf,
            SoundEffect.Capture        => _captureBuf,
            SoundEffect.Rescue         => _rescueBuf,
            _                          => _shootBuf
        };

        uint src = _sources[_nextSource++ % _sources.Length];
        _al.SourceStop(src);
        _al.SetSourceProperty(src, SourceInteger.Buffer, (int)buf);
        _al.SetSourceProperty(src, SourceFloat.Gain, 0.38f);
        _al.SourcePlay(src);
    }

    // ─── Buffer helpers ──────────────────────────────────────────────────────

    private static uint CreateBuffer(AL al, short[] data)
    {
        uint buf = al.GenBuffer();
        al.BufferData<short>(buf, BufferFormat.Mono16, data, SampleRate);
        return buf;
    }

    // ─── Sound synthesis ─────────────────────────────────────────────────────

    private static short[] SynthShoot()
        => FreqSweep(startHz: 820, endHz: 160, seconds: 0.09f, amp: 0.28f);

    private static short[] SynthExplosion()
    {
        int n   = (int)(SampleRate * 0.22f);
        var buf = new short[n];
        var rng = new Random(7);
        double phase = 0;
        for (int i = 0; i < n; i++)
        {
            float t      = (float)i / n;
            float amp    = (1f - t) * (1f - t) * 0.45f;
            float noise  = (float)(rng.NextDouble() * 2.0 - 1.0);
            phase += 80.0 / SampleRate;
            float rumble = (float)Math.Sin(phase * Math.Tau) * 0.4f;
            buf[i] = (short)((noise * 0.6f + rumble) * amp * short.MaxValue);
        }
        return buf;
    }

    private static short[] SynthPlayerDeath()
        => FreqSweep(startHz: 580, endHz: 55, seconds: 0.6f, amp: 0.35f);

    private static short[] SynthStageClear()
    {
        // C4-E4-G4-C5 arpeggio
        float[] freqs    = { 261.6f, 329.6f, 392f, 523.3f };
        int     perNote  = SampleRate / 7;
        var     buf      = new short[freqs.Length * perNote];
        for (int n = 0; n < freqs.Length; n++)
        {
            double phase = 0;
            for (int i = 0; i < perNote; i++)
            {
                float t   = (float)i / perNote;
                float amp = 0.3f * (1f - t * 0.4f);
                phase += freqs[n] / SampleRate;
                float val = (phase % 1.0) < 0.5 ? 1f : -1f;
                buf[n * perNote + i] = (short)(val * amp * short.MaxValue);
            }
        }
        return buf;
    }

    private static short[] SynthCapture()
    {
        // Warbling tractor-beam: descending tone with wobble
        int n = (int)(SampleRate * 0.5f);
        var buf = new short[n];
        double ph = 0;
        for (int i = 0; i < n; i++)
        {
            float t    = (float)i / n;
            float freq = 420f - 280f * t + (float)Math.Sin(t * 40.0) * 45f;
            float amp  = 0.3f * (1f - t * 0.5f);
            ph += freq / SampleRate;
            float val = (ph % 1.0) < 0.5 ? 1f : -1f;
            buf[i] = (short)(val * amp * short.MaxValue);
        }
        return buf;
    }

    private static short[] SynthRescue()
    {
        // Ascending C4-G4-C5-E5 arpeggio (the dual-fighter reward)
        float[] freqs   = { 261.6f, 392f, 523.3f, 659.3f };
        int     perNote = SampleRate / 9;
        var     buf     = new short[freqs.Length * perNote];
        for (int k = 0; k < freqs.Length; k++)
        {
            double phase = 0;
            for (int i = 0; i < perNote; i++)
            {
                float t   = (float)i / perNote;
                float amp = 0.32f * (1f - t * 0.4f);
                phase += freqs[k] / SampleRate;
                float val = (phase % 1.0) < 0.5 ? 1f : -1f;
                buf[k * perNote + i] = (short)(val * amp * short.MaxValue);
            }
        }
        return buf;
    }

    private static short[] FreqSweep(float startHz, float endHz, float seconds, float amp)
    {
        int    n   = (int)(SampleRate * seconds);
        var    buf = new short[n];
        double ph  = 0;
        for (int i = 0; i < n; i++)
        {
            float t    = (float)i / n;
            float freq = startHz + (endHz - startHz) * t;
            float a    = amp * (1f - t * 0.75f);
            ph += freq / SampleRate;
            float val = (ph % 1.0) < 0.5 ? 1f : -1f;
            buf[i] = (short)(val * a * short.MaxValue);
        }
        return buf;
    }

    // ─── Dispose ─────────────────────────────────────────────────────────────

    public void Dispose()
    {
        if (!IsAvailable || _al is null || _alc is null) return;
        IsAvailable = false;

        foreach (var src in _sources) _al.DeleteSource(src);
        _al.DeleteBuffer(_shootBuf);
        _al.DeleteBuffer(_explosionBuf);
        _al.DeleteBuffer(_playerDeathBuf);
        _al.DeleteBuffer(_stageClearBuf);
        _al.DeleteBuffer(_captureBuf);
        _al.DeleteBuffer(_rescueBuf);
        _alc.DestroyContext(_ctx);
        _alc.CloseDevice(_device);
        _al.Dispose();
        _alc.Dispose();
    }
}
