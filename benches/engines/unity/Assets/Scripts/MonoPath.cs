// Non-ECS path: one manager looping over plain C# objects, the natural layout for many entities outside of ECS.
using System;
using System.Collections.Generic;
using UnityEngine;

public class MonoPath : IBenchPath
{
    class Bouncy
    {
        public float Restitution;
        public int Left;
    }

    class Ball
    {
        public Vector2 Pos, Vel;
        public float Life;
        public Color Tint;
        public Bouncy Bouncy;
        public readonly List<Vector2> Trail = new();
    }

    class Infection
    {
        public float Timer;
        public string Strain;
    }

    class Agent
    {
        public int Id;
        public Vector2 Pos, Vel;
        public float Hp;
        public Infection Infected;
        public Agent Target;
        public readonly List<int> Memory = new();
        public bool Dead;
    }

    Scenario cfg;
    string scenario;
    long rng;
    Vector2 arena;

    readonly List<Ball> balls = new();
    readonly List<(Ball, int)> hits = new();
    event Action<Ball, int> Bounced;
    long scoreTotal;
    readonly Dictionary<string, long> perWall = new();

    List<Agent> agents = new();
    int nextId;

    public void Setup(Scenario cfg, string scenario)
    {
        this.cfg = cfg;
        this.scenario = scenario;
        rng = cfg.seed;
        arena = new Vector2(cfg.arena[0], cfg.arena[1]);
        foreach (var w in Shared.Walls)
            perWall[w] = 0;
        Bounced += OnBounce;
    }

    double Rand()
    {
        rng = rng * 48271 % 2147483647;
        return rng / 2147483647.0;
    }

    public void Phase(string phase, int frame, int target)
    {
        switch (scenario, phase)
        {
            case ("s1", "lifecycle"): S1Lifecycle(target); break;
            case ("s1", "integrate"): S1Integrate(frame); break;
            case ("s1", "callbacks"): S1Callbacks(); break;
            case ("s2", "lifecycle"): S2Lifecycle(target); break;
            case ("s2", "integrate"): S2Integrate(frame); break;
            case ("s2", "behaviour"): S2Behaviour(frame, target); break;
        }
    }

    public int Alive() => balls.Count + agents.Count;

    public int Infected()
    {
        int n = 0;
        foreach (var a in agents)
            if (a.Infected != null)
                n++;
        return n;
    }

    public (int, double, long) Checksum()
    {
        double sum = 0;
        foreach (var b in balls)
            sum += b.Pos.x + b.Pos.y;
        foreach (var a in agents)
            sum += a.Pos.x + a.Pos.y;
        return (Alive(), sum, scoreTotal);
    }

    // S1 Fountain

    void S1Lifecycle(int target)
    {
        var s1 = cfg.s1;
        balls.RemoveAll(b => b.Life <= 0);
        int count = Math.Min(target - balls.Count, Math.Max(1, target / s1.spawn_cap_div));
        for (int i = 0; i < count; i++)
        {
            float angle = Shared.Lerp(s1.angle_deg, Rand()) * Mathf.Deg2Rad;
            float speed = Shared.Lerp(s1.speed, Rand());
            var b = new Ball
            {
                Pos = new Vector2(s1.emitter[0], s1.emitter[1]),
                Vel = new Vector2(Mathf.Cos(angle), Mathf.Sin(angle)) * speed,
                Life = Shared.Lerp(s1.lifetime, Rand()),
                Tint = Color.white,
            };
            b.Bouncy = new Bouncy { Restitution = Shared.Lerp(s1.restitution, Rand()) };
            b.Bouncy.Left = (int)s1.bounces[0] + (int)Math.Floor(Rand() * (s1.bounces[1] - s1.bounces[0] + 1));
            balls.Add(b);
        }
    }

    void S1Integrate(int frame)
    {
        var s1 = cfg.s1;
        float dt = cfg.dt;
        var gravity = new Vector2(0, s1.gravity * dt);
        float damp = 1 - s1.drag * dt;
        bool pushTrail = frame % s1.trail_every == 0;
        float w = arena.x, h = arena.y;
        foreach (var b in balls)
        {
            var v = (b.Vel + gravity) * damp;
            var p = b.Pos + v * dt;
            int wall = 0;
            if (p.x < 0) { p.x = -p.x; v.x = -v.x; wall = 1; }
            else if (p.x > w) { p.x = 2 * w - p.x; v.x = -v.x; wall = 2; }
            if (p.y < 0) { p.y = -p.y; v.y = -v.y; wall = 3; }
            else if (p.y > h) { p.y = 2 * h - p.y; v.y = -v.y; wall = 4; }
            if (wall != 0)
            {
                v *= b.Bouncy.Restitution;
                hits.Add((b, wall));
            }
            b.Vel = v;
            b.Pos = p;
            b.Life -= dt;
            if (pushTrail)
            {
                b.Trail.Insert(0, p);
                if (b.Trail.Count > s1.trail_len)
                    b.Trail.RemoveAt(s1.trail_len);
            }
        }
    }

    void S1Callbacks()
    {
        foreach (var (b, wall) in hits)
            Bounced?.Invoke(b, wall);
        hits.Clear();
    }

    void OnBounce(Ball b, int wall)
    {
        scoreTotal++;
        perWall[Shared.Walls[wall - 1]]++;
        b.Bouncy.Left--;
        float k = b.Bouncy.Left / 6f;
        b.Tint = new Color(1, k, k);
        if (b.Bouncy.Left <= 0)
            b.Life = 0;
    }

    // S2 Infection

    void S2Lifecycle(int target)
    {
        var s2 = cfg.s2;
        foreach (var a in agents)
            if (a.Hp <= 0)
                a.Dead = true;
        agents.RemoveAll(a => a.Dead);
        int count = Math.Min(target - agents.Count, Math.Max(1, target / cfg.s1.spawn_cap_div));
        for (int i = 0; i < count; i++)
        {
            int id = nextId++;
            float angle = (float)(Shared.Hash01(id, 0, 3) * Math.PI * 2);
            var a = new Agent
            {
                Id = id,
                Pos = new Vector2((float)Shared.Hash01(id, 0, 1) * arena.x, (float)Shared.Hash01(id, 0, 2) * arena.y),
                Vel = new Vector2(Mathf.Cos(angle), Mathf.Sin(angle)) * s2.speed,
                Hp = s2.hp,
            };
            if (id % s2.seed_every == 0)
                a.Infected = new Infection
                {
                    Timer = s2.infect_time,
                    Strain = s2.strains[id / s2.seed_every % s2.strains.Length],
                };
            agents.Add(a);
        }
    }

    void S2Integrate(int frame)
    {
        var s2 = cfg.s2;
        float dt = cfg.dt;
        foreach (var a in agents)
            if (a.Infected != null)
            {
                a.Hp -= s2.damage_per_s * dt;
                a.Infected.Timer -= dt;
            }
        // random access: read every chaser's target position first
        var chase = new Dictionary<Agent, Vector2>();
        foreach (var a in agents)
            if (a.Target != null && !a.Target.Dead)
                chase[a] = (a.Target.Pos - a.Pos).normalized * s2.chase_speed;
        foreach (var a in agents)
        {
            if (chase.TryGetValue(a, out var v))
                a.Vel = v;
            else
            {
                float angle = (float)(Shared.Hash01(a.Id, frame, 4) - 0.5) * s2.jitter;
                float c = Mathf.Cos(angle), s = Mathf.Sin(angle);
                a.Vel = new Vector2(a.Vel.x * c - a.Vel.y * s, a.Vel.x * s + a.Vel.y * c);
            }
            var p = a.Pos + a.Vel * dt;
            a.Pos = new Vector2(Mod(p.x, arena.x), Mod(p.y, arena.y));
        }
    }

    static float Mod(float x, float m) => x - m * Mathf.Floor(x / m);

    void S2Behaviour(int frame, int target)
    {
        var s2 = cfg.s2;
        int cols = Mathf.CeilToInt(arena.x / s2.cell), rows = Mathf.CeilToInt(arena.y / s2.cell);
        double pInfect = s2.p_infect * 5000.0 / target;

        // healthy agents bucketed by cell, infected agents are never neighbours
        var cells = new List<Agent>[cols * rows];
        var infected = new List<Agent>();
        foreach (var a in agents)
        {
            if (a.Infected != null)
            {
                infected.Add(a);
                continue;
            }
            int key = Math.Min((int)(a.Pos.y / s2.cell), rows - 1) * cols + Math.Min((int)(a.Pos.x / s2.cell), cols - 1);
            (cells[key] ??= new List<Agent>()).Add(a);
        }

        var infections = new Dictionary<Agent, Agent>();
        foreach (var i in infected)
        {
            if (i.Infected.Timer <= 0)
            {
                i.Infected = null;
                i.Target = null;
                continue;
            }
            int cx = Math.Min((int)(i.Pos.x / s2.cell), cols - 1), cy = Math.Min((int)(i.Pos.y / s2.cell), rows - 1);
            Agent nearest = null;
            float nd = 0;
            for (int y = Math.Max(cy - 1, 0); y <= Math.Min(cy + 1, rows - 1); y++)
            for (int x = Math.Max(cx - 1, 0); x <= Math.Min(cx + 1, cols - 1); x++)
            {
                var bucket = cells[y * cols + x];
                if (bucket == null)
                    continue;
                foreach (var other in bucket)
                {
                    float d = Vector2.Distance(i.Pos, other.Pos);
                    if (d < s2.radius)
                    {
                        if (other.Memory.Count < s2.memory_cap && !other.Memory.Contains(i.Id))
                            other.Memory.Add(i.Id);
                        if (Shared.Hash01(other.Id, frame, 5) < pInfect
                            && (!infections.TryGetValue(other, out var current) || i.Id < current.Id))
                            infections[other] = i;
                    }
                    if (nearest == null || d < nd || (d == nd && other.Id < nearest.Id))
                    {
                        nearest = other;
                        nd = d;
                    }
                }
            }
            if (i.Target == null)
                i.Target = nearest;
            else if (i.Target.Dead || i.Target.Infected != null)
                i.Target = null;
        }

        foreach (var (other, by) in infections)
            other.Infected = new Infection { Timer = s2.infect_time, Strain = by.Infected.Strain };
    }
}
