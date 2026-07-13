using System.IO;
using System.Text.Json;

namespace Galaga.Engine;

/// <summary>
/// Persists the all-time high score to a JSON file under the user's
/// local application data folder. All I/O is best-effort: failures are
/// silently ignored so the game keeps running.
/// </summary>
public static class HighScoreStore
{
    private static readonly string FilePath = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
        "Galaga", "highscore.json");

    private static int _cached = -1;

    public static int Load()
    {
        if (_cached >= 0) return _cached;
        try
        {
            if (File.Exists(FilePath))
                _cached = JsonSerializer.Deserialize<int>(File.ReadAllText(FilePath));
        }
        catch
        {
            // ignore corrupted / unreadable file
        }
        if (_cached < 0) _cached = 0;
        return _cached;
    }

    public static void Save(int score)
    {
        if (score <= _cached) return;
        _cached = score;
        try
        {
            Directory.CreateDirectory(Path.GetDirectoryName(FilePath)!);
            File.WriteAllText(FilePath, JsonSerializer.Serialize(score));
        }
        catch
        {
            // ignore write failures (read-only disk, etc.)
        }
    }
}
