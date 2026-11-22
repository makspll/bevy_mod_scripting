// Engine benchmarks, Unity harness. See ../../../README.md for the rules.
// bench.x86_64 -batchmode -nographics -- --path mono|ecs --name <label> --scenario s1 --config scenario.json [--stages 1000,5000] [--out dir]
// The ECS path runs with Burst and jobs; `--burst-disable-compilation -job-worker-count 0` gives the plain C# variant.
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text;
using UnityEngine;

[Serializable]
public class S1Config
{
    public float[] emitter;
    public float gravity;
    public float drag;
    public float[] speed;
    public float[] angle_deg;
    public float[] lifetime;
    public float[] restitution;
    public float[] bounces;
    public int spawn_cap_div;
    public int trail_every;
    public int trail_len;
}

[Serializable]
public class S2Config
{
    public float cell;
    public float speed;
    public float chase_speed;
    public float jitter;
    public float radius;
    public float p_infect;
    public float infect_time;
    public float damage_per_s;
    public float hp;
    public int seed_every;
    public string[] strains;
    public int memory_cap;
}

[Serializable]
public class Scenario
{
    public long seed;
    public float dt;
    public float[] arena;
    public int[] stages;
    public int warmup_frames;
    public int measure_frames;
    public S1Config s1;
    public S2Config s2;
}

/// A path through Unity: runs the phases of one frame.
public interface IBenchPath
{
    void Setup(Scenario cfg, string scenario);
    void Phase(string phase, int frame, int target);
    int Alive();
    int Infected();
    /// alive, sum of x + y, score
    (int, double, long) Checksum();
}

public static class Shared
{
    public static readonly string[] Walls = { "left", "right", "floor", "ceiling" };

    /// Order independent random number for S2.
    public static double Hash01(long id, long frame, long salt)
    {
        long x = (id * 7919 + frame * 104729 + salt) % 2147483646 + 1;
        x = x * 48271 % 2147483647;
        x = x * 48271 % 2147483647;
        return x / 2147483647.0;
    }

    public static float Lerp(float[] range, double t) => range[0] + (range[1] - range[0]) * (float)t;
}

/// No default ECS world: the benchmark creates its own, and the default one crashes the player's first frame under
/// `-nographics`.
public class NoDefaultWorld : Unity.Entities.ICustomBootstrap
{
    public bool Initialize(string defaultWorldName) => true;
}

public static class Bench
{
    // Runs before the first frame, then kills the player: under `-nographics` its first frame segfaults and
    // `Environment.Exit` hangs. Success is the result file existing, the workflow checks it.
    [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
    static void Init()
    {
        var args = ParseArgs();
        if (!args.ContainsKey("scenario"))
            return;
        try
        {
            RunBench(args);
        }
        catch (Exception e)
        {
            Console.Error.WriteLine(e);
        }
        Console.Error.Flush();
        Process.GetCurrentProcess().Kill();
    }

    static Dictionary<string, string> ParseArgs()
    {
        var args = Environment.GetCommandLineArgs();
        var outArgs = new Dictionary<string, string>();
        for (int i = 0; i < args.Length - 1; i++)
            if (args[i].StartsWith("--") && !args[i + 1].StartsWith("-"))
                outArgs[args[i].Substring(2)] = args[i + 1];
        return outArgs;
    }

    static void RunBench(Dictionary<string, string> args)
    {
        var cfg = JsonUtility.FromJson<Scenario>(File.ReadAllText(args["config"]));
        string scenario = args["scenario"];
        string name = args.TryGetValue("name", out var n) ? n : args["path"];
        int[] stages = args.TryGetValue("stages", out var s) ? s.Split(',').Select(int.Parse).ToArray() : cfg.stages;
        string outDir = args.TryGetValue("out", out var o) ? o : "results";

        IBenchPath path = args["path"] switch
        {
            "mono" => new MonoPath(),
            "ecs" => new EcsPath(),
            var other => throw new ArgumentException($"unknown path {other}"),
        };
        path.Setup(cfg, scenario);

        string[] phases = scenario == "s1"
            ? new[] { "lifecycle", "integrate", "callbacks" }
            : new[] { "lifecycle", "integrate", "behaviour" };
        var stageJson = new List<string>();
        var infected = new List<int>();
        int frame = 0;
        var watch = new Stopwatch();
        foreach (int target in stages)
        {
            var samples = phases.Concat(new[] { "frame" }).ToDictionary(p => p, _ => new List<double>());
            long alive = 0;
            for (int i = 0; i < cfg.warmup_frames + cfg.measure_frames; i++)
            {
                double frameMs = 0;
                foreach (var p in phases)
                {
                    watch.Restart();
                    path.Phase(p, frame, target);
                    double ms = watch.Elapsed.TotalMilliseconds;
                    frameMs += ms;
                    if (i >= cfg.warmup_frames)
                        samples[p].Add(ms);
                }
                if (i >= cfg.warmup_frames)
                {
                    samples["frame"].Add(frameMs);
                    alive += path.Alive();
                }
                if (frame % 60 == 0)
                    infected.Add(path.Infected());
                frame++;
            }
            var sb = new StringBuilder();
            sb.Append($"{{\"target\":{target},\"alive\":{alive / cfg.measure_frames}");
            foreach (var kv in samples)
                sb.Append($",\"{kv.Key}\":{Stats(kv.Value)}");
            sb.Append('}');
            Console.Error.WriteLine($"{name} {scenario}: {sb}");
            stageJson.Add(sb.ToString());
        }

        var (count, sum, score) = path.Checksum();
        string checksum = $"{{\"alive\":{count},\"sum\":{F(sum)},\"score\":{score}}}";
        string result = $"{{\"engine\":\"unity\",\"path\":\"{name}\",\"scenario\":\"{scenario}\",\"frames\":{frame}," +
                        $"\"stages\":[{string.Join(",", stageJson)}],\"checksum\":{checksum}," +
                        $"\"infected\":[{string.Join(",", infected)}]}}";
        Directory.CreateDirectory(outDir);
        File.WriteAllText(Path.Combine(outDir, $"{scenario}_{name}.json"), result);
        Console.Error.WriteLine($"checksum: {checksum}");
    }

    static string F(double v) => v.ToString("R", CultureInfo.InvariantCulture);

    static string Stats(List<double> samples)
    {
        samples.Sort();
        double At(double q) => samples[(int)Math.Round((samples.Count - 1) * q)];
        return $"{{\"median\":{F(At(0.5))},\"p95\":{F(At(0.95))}}}";
    }
}
