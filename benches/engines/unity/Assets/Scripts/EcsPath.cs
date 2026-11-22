// ECS path: Entities systems compiled with Burst, parallel jobs where the work is per entity.
// Run the player with `--burst-disable-compilation -job-worker-count 0` for the plain C# variant.
using Unity.Burst;
using Unity.Collections;
using Unity.Entities;
using Unity.Mathematics;
using Unity.Transforms;

namespace EcsBench
{
    public struct Config : IComponentData
    {
        public float Dt;
        public float2 Arena;
        public int SpawnCapDiv;
        // S1
        public float2 Emitter;
        public float Gravity, Drag;
        public float2 Speed, AngleDeg, Lifetime, Restitution, Bounces;
        public int TrailEvery, TrailLen;
        // S2
        public float Cell, AgentSpeed, ChaseSpeed, Jitter, Radius, PInfect, InfectTime, DamagePerS, Hp;
        public int SeedEvery, MemoryCap;
        public FixedList512Bytes<FixedString32Bytes> Strains;
    }

    public struct FrameInput : IComponentData
    {
        public int Frame, Target;
    }

    public struct RngState : IComponentData
    {
        public long Value;
    }

    public struct NextId : IComponentData
    {
        public int Value;
    }

    /// `per_wall` is four counters, Burst has no string keyed maps.
    public struct Score : IComponentData
    {
        public long Total;
        public int Left, Right, Floor, Ceiling;
    }

    public struct Velocity : IComponentData
    {
        public float3 Value;
        public bool Chasing;
    }

    public struct Lifetime : IComponentData
    {
        public float Value;
    }

    public struct Tint : IComponentData
    {
        public float3 Value;
    }

    public struct Bouncy : IComponentData
    {
        public float Restitution;
        public int Left;
        public int LastWall;
    }

    /// Enabled for one frame when the ball hit a wall, the ECS way of sending an event.
    public struct Bounce : IComponentData, IEnableableComponent { }

    [InternalBufferCapacity(8)]
    public struct TrailPoint : IBufferElementData
    {
        public float2 Value;
    }

    public struct AgentId : IComponentData
    {
        public int Value;
    }

    public struct Health : IComponentData
    {
        public float Value;
    }

    public struct Infected : IComponentData
    {
        public float Timer;
        public FixedString32Bytes Strain;
    }

    public struct Target : IComponentData
    {
        public Entity Value;
    }

    [InternalBufferCapacity(16)]
    public struct MemoryId : IBufferElementData
    {
        public int Value;
    }

    public static class Util
    {
        public static double Rand(ref long rng)
        {
            rng = rng * 48271 % 2147483647;
            return rng / 2147483647.0;
        }

        public static double Hash01(long id, long frame, long salt)
        {
            long x = (id * 7919 + frame * 104729 + salt) % 2147483646 + 1;
            x = x * 48271 % 2147483647;
            x = x * 48271 % 2147483647;
            return x / 2147483647.0;
        }

        public static float Lerp(float2 range, double t) => range.x + (range.y - range.x) * (float)t;

        public static int SpawnCount(int target, int alive, int capDiv) =>
            math.min(target - alive, math.max(1, target / capDiv));
    }

    // S1 Fountain

    [DisableAutoCreation, BurstCompile]
    public partial struct S1LifecycleSystem : ISystem
    {
        EntityArchetype archetype;

        public void OnCreate(ref SystemState state)
        {
            archetype = state.EntityManager.CreateArchetype(
                typeof(LocalTransform), typeof(Velocity), typeof(Lifetime), typeof(Tint),
                typeof(Bouncy), typeof(Bounce), typeof(TrailPoint));
        }

        [BurstCompile]
        public void OnUpdate(ref SystemState state)
        {
            var cfg = SystemAPI.GetSingleton<Config>();
            var input = SystemAPI.GetSingleton<FrameInput>();
            long rng = SystemAPI.GetSingleton<RngState>().Value;

            var ecb = new EntityCommandBuffer(Allocator.Temp);
            int alive = 0;
            foreach (var (life, e) in SystemAPI.Query<RefRO<Lifetime>>().WithEntityAccess())
            {
                if (life.ValueRO.Value <= 0)
                    ecb.DestroyEntity(e);
                else
                    alive++;
            }
            ecb.Playback(state.EntityManager);

            int count = Util.SpawnCount(input.Target, alive, cfg.SpawnCapDiv);
            if (count <= 0)
                return;
            var entities = state.EntityManager.CreateEntity(archetype, count, Allocator.Temp);
            foreach (var e in entities)
            {
                float angle = math.radians(Util.Lerp(cfg.AngleDeg, Util.Rand(ref rng)));
                float speed = Util.Lerp(cfg.Speed, Util.Rand(ref rng));
                float lifetime = Util.Lerp(cfg.Lifetime, Util.Rand(ref rng));
                float restitution = Util.Lerp(cfg.Restitution, Util.Rand(ref rng));
                int left = (int)cfg.Bounces.x + (int)math.floor(Util.Rand(ref rng) * (cfg.Bounces.y - cfg.Bounces.x + 1));
                state.EntityManager.SetComponentData(e, LocalTransform.FromPosition(new float3(cfg.Emitter, 0)));
                state.EntityManager.SetComponentData(e, new Velocity { Value = new float3(math.cos(angle), math.sin(angle), 0) * speed });
                state.EntityManager.SetComponentData(e, new Lifetime { Value = lifetime });
                state.EntityManager.SetComponentData(e, new Tint { Value = new float3(1) });
                state.EntityManager.SetComponentData(e, new Bouncy { Restitution = restitution, Left = left });
                state.EntityManager.SetComponentEnabled<Bounce>(e, false);
            }
            SystemAPI.SetSingleton(new RngState { Value = rng });
        }
    }

    [BurstCompile, WithOptions(EntityQueryOptions.IgnoreComponentEnabledState)]
    public partial struct S1IntegrateJob : IJobEntity
    {
        public float Dt, Damp, Gravity, W, H;
        public bool PushTrail;
        public int TrailLen;

        void Execute(ref LocalTransform transform, ref Velocity vel, ref Lifetime life, ref Bouncy bouncy,
            EnabledRefRW<Bounce> bounce, DynamicBuffer<TrailPoint> trail)
        {
            var v = (vel.Value.xy + new float2(0, Gravity)) * Damp;
            var p = transform.Position.xy + v * Dt;
            int wall = 0;
            if (p.x < 0) { p.x = -p.x; v.x = -v.x; wall = 1; }
            else if (p.x > W) { p.x = 2 * W - p.x; v.x = -v.x; wall = 2; }
            if (p.y < 0) { p.y = -p.y; v.y = -v.y; wall = 3; }
            else if (p.y > H) { p.y = 2 * H - p.y; v.y = -v.y; wall = 4; }
            if (wall != 0)
            {
                v *= bouncy.Restitution;
                bouncy.LastWall = wall;
                bounce.ValueRW = true;
            }
            vel.Value = new float3(v, 0);
            transform.Position = new float3(p, 0);
            life.Value -= Dt;
            if (PushTrail)
            {
                trail.Insert(0, new TrailPoint { Value = p });
                if (trail.Length > TrailLen)
                    trail.RemoveAt(TrailLen);
            }
        }
    }

    [DisableAutoCreation, BurstCompile]
    public partial struct S1IntegrateSystem : ISystem
    {
        [BurstCompile]
        public void OnUpdate(ref SystemState state)
        {
            var cfg = SystemAPI.GetSingleton<Config>();
            var input = SystemAPI.GetSingleton<FrameInput>();
            new S1IntegrateJob
            {
                Dt = cfg.Dt,
                Damp = 1 - cfg.Drag * cfg.Dt,
                Gravity = cfg.Gravity * cfg.Dt,
                W = cfg.Arena.x,
                H = cfg.Arena.y,
                PushTrail = input.Frame % cfg.TrailEvery == 0,
                TrailLen = cfg.TrailLen,
            }.ScheduleParallel();
            state.CompleteDependency();
        }
    }

    [DisableAutoCreation, BurstCompile]
    public partial struct S1CallbacksSystem : ISystem
    {
        [BurstCompile]
        public void OnUpdate(ref SystemState state)
        {
            ref var score = ref SystemAPI.GetSingletonRW<Score>().ValueRW;
            foreach (var (bouncy, tint, life, bounce) in SystemAPI
                         .Query<RefRW<Bouncy>, RefRW<Tint>, RefRW<Lifetime>, EnabledRefRW<Bounce>>())
            {
                bounce.ValueRW = false;
                OnBounce(ref score, ref bouncy.ValueRW, ref tint.ValueRW, ref life.ValueRW);
            }
        }

        static void OnBounce(ref Score score, ref Bouncy bouncy, ref Tint tint, ref Lifetime life)
        {
            score.Total++;
            switch (bouncy.LastWall)
            {
                case 1: score.Left++; break;
                case 2: score.Right++; break;
                case 3: score.Floor++; break;
                default: score.Ceiling++; break;
            }
            bouncy.Left--;
            float k = bouncy.Left / 6f;
            tint.Value = new float3(1, k, k);
            if (bouncy.Left <= 0)
                life.Value = 0;
        }
    }

    // S2 Infection

    [DisableAutoCreation, BurstCompile]
    public partial struct S2LifecycleSystem : ISystem
    {
        EntityArchetype archetype;

        public void OnCreate(ref SystemState state)
        {
            archetype = state.EntityManager.CreateArchetype(
                typeof(LocalTransform), typeof(Velocity), typeof(AgentId), typeof(Health), typeof(MemoryId));
        }

        [BurstCompile]
        public void OnUpdate(ref SystemState state)
        {
            var cfg = SystemAPI.GetSingleton<Config>();
            var input = SystemAPI.GetSingleton<FrameInput>();
            int nextId = SystemAPI.GetSingleton<NextId>().Value;

            var ecb = new EntityCommandBuffer(Allocator.Temp);
            int alive = 0;
            foreach (var (health, e) in SystemAPI.Query<RefRO<Health>>().WithEntityAccess())
            {
                if (health.ValueRO.Value <= 0)
                    ecb.DestroyEntity(e);
                else
                    alive++;
            }
            ecb.Playback(state.EntityManager);

            int count = Util.SpawnCount(input.Target, alive, cfg.SpawnCapDiv);
            if (count <= 0)
                return;
            var entities = state.EntityManager.CreateEntity(archetype, count, Allocator.Temp);
            foreach (var e in entities)
            {
                int id = nextId++;
                float a = (float)(Util.Hash01(id, 0, 3) * math.PI_DBL * 2);
                var pos = new float3((float)Util.Hash01(id, 0, 1) * cfg.Arena.x, (float)Util.Hash01(id, 0, 2) * cfg.Arena.y, 0);
                state.EntityManager.SetComponentData(e, LocalTransform.FromPosition(pos));
                state.EntityManager.SetComponentData(e, new Velocity { Value = new float3(math.cos(a), math.sin(a), 0) * cfg.AgentSpeed });
                state.EntityManager.SetComponentData(e, new AgentId { Value = id });
                state.EntityManager.SetComponentData(e, new Health { Value = cfg.Hp });
                if (id % cfg.SeedEvery == 0)
                    state.EntityManager.AddComponentData(e, new Infected
                    {
                        Timer = cfg.InfectTime,
                        Strain = cfg.Strains[id / cfg.SeedEvery % cfg.Strains.Length],
                    });
            }
            SystemAPI.SetSingleton(new NextId { Value = nextId });
        }
    }

    [BurstCompile]
    public partial struct S2DamageJob : IJobEntity
    {
        public float Damage, Dt;

        void Execute(ref Health health, ref Infected infected)
        {
            health.Value -= Damage;
            infected.Timer -= Dt;
        }
    }

    /// Random access: reads the target's position.
    [BurstCompile]
    public partial struct S2ChaseJob : IJobEntity
    {
        [ReadOnly] public ComponentLookup<LocalTransform> Transforms;
        public float Speed;

        void Execute(ref Velocity vel, in LocalTransform transform, in Target target)
        {
            if (!Transforms.HasComponent(target.Value))
                return;
            vel.Value = math.normalizesafe(Transforms[target.Value].Position - transform.Position) * Speed;
            vel.Chasing = true;
        }
    }

    [BurstCompile]
    public partial struct S2MoveJob : IJobEntity
    {
        public int Frame;
        public float Dt, Jitter;
        public float2 Arena;

        void Execute(in AgentId id, ref LocalTransform transform, ref Velocity vel)
        {
            if (!vel.Chasing)
            {
                float a = (float)(Util.Hash01(id.Value, Frame, 4) - 0.5) * Jitter;
                math.sincos(a, out float s, out float c);
                var v = vel.Value;
                vel.Value = new float3(v.x * c - v.y * s, v.x * s + v.y * c, 0);
            }
            vel.Chasing = false;
            var p = transform.Position.xy + vel.Value.xy * Dt;
            p -= Arena * math.floor(p / Arena);
            transform.Position = new float3(p, 0);
        }
    }

    [DisableAutoCreation, BurstCompile]
    public partial struct S2IntegrateSystem : ISystem
    {
        [BurstCompile]
        public void OnUpdate(ref SystemState state)
        {
            var cfg = SystemAPI.GetSingleton<Config>();
            var input = SystemAPI.GetSingleton<FrameInput>();
            new S2DamageJob { Damage = cfg.DamagePerS * cfg.Dt, Dt = cfg.Dt }.ScheduleParallel();
            new S2ChaseJob
            {
                Transforms = SystemAPI.GetComponentLookup<LocalTransform>(true),
                Speed = cfg.ChaseSpeed,
            }.ScheduleParallel();
            new S2MoveJob { Frame = input.Frame, Dt = cfg.Dt, Jitter = cfg.Jitter, Arena = cfg.Arena }.ScheduleParallel();
            state.CompleteDependency();
        }
    }

    public struct Neighbour
    {
        public Entity Entity;
        public int Id;
        public float2 Pos;
    }

    public struct Infector
    {
        public int Id;
        public FixedString32Bytes Strain;
    }

    /// Healthy agents bucketed by cell, infected agents are never neighbours.
    [BurstCompile, WithNone(typeof(Infected))]
    public partial struct S2GridJob : IJobEntity
    {
        public NativeParallelMultiHashMap<int, Neighbour>.ParallelWriter Grid;
        public float Cell;
        public int Cols, Rows;

        void Execute(Entity e, in AgentId id, in LocalTransform transform)
        {
            var p = transform.Position.xy;
            int key = math.min((int)(p.y / Cell), Rows - 1) * Cols + math.min((int)(p.x / Cell), Cols - 1);
            Grid.Add(key, new Neighbour { Entity = e, Id = id.Value, Pos = p });
        }
    }

    [BurstCompile]
    public partial struct S2InfectJob : IJobEntity
    {
        [ReadOnly] public NativeParallelMultiHashMap<int, Neighbour> Grid;
        [ReadOnly] public ComponentLookup<Target> Targets;
        [ReadOnly] public ComponentLookup<Infected> InfectedLookup;
        [ReadOnly] public EntityStorageInfoLookup Exists;
        public BufferLookup<MemoryId> Memory;
        public NativeHashMap<Entity, Infector> Infections;
        public EntityCommandBuffer Ecb;
        public int Frame, Cols, Rows, MemoryCap;
        public float Cell, Radius;
        public double PInfect;

        void Execute(Entity e, in AgentId id, in LocalTransform transform, in Infected infected)
        {
            if (infected.Timer <= 0)
            {
                Ecb.RemoveComponent<Infected>(e);
                Ecb.RemoveComponent<Target>(e);
                return;
            }
            var p = transform.Position.xy;
            int cx = math.min((int)(p.x / Cell), Cols - 1), cy = math.min((int)(p.y / Cell), Rows - 1);
            var nearest = Entity.Null;
            int nid = 0;
            float nd = 0;
            for (int y = math.max(cy - 1, 0); y <= math.min(cy + 1, Rows - 1); y++)
            for (int x = math.max(cx - 1, 0); x <= math.min(cx + 1, Cols - 1); x++)
            {
                if (!Grid.TryGetFirstValue(y * Cols + x, out var other, out var it))
                    continue;
                do
                {
                    float d = math.distance(p, other.Pos);
                    if (d < Radius)
                    {
                        var memory = Memory[other.Entity];
                        if (memory.Length < MemoryCap)
                        {
                            bool seen = false;
                            for (int i = 0; i < memory.Length; i++)
                                seen |= memory[i].Value == id.Value;
                            if (!seen)
                                memory.Add(new MemoryId { Value = id.Value });
                        }
                        if (Util.Hash01(other.Id, Frame, 5) < PInfect
                            && (!Infections.TryGetValue(other.Entity, out var current) || id.Value < current.Id))
                            Infections[other.Entity] = new Infector { Id = id.Value, Strain = infected.Strain };
                    }
                    if (nearest == Entity.Null || d < nd || (d == nd && other.Id < nid))
                    {
                        nearest = other.Entity;
                        nid = other.Id;
                        nd = d;
                    }
                } while (Grid.TryGetNextValue(out other, ref it));
            }

            if (!Targets.HasComponent(e))
            {
                if (nearest != Entity.Null)
                    Ecb.AddComponent(e, new Target { Value = nearest });
            }
            else
            {
                var t = Targets[e].Value;
                if (!Exists.Exists(t) || InfectedLookup.HasComponent(t))
                    Ecb.RemoveComponent<Target>(e);
            }
        }
    }

    [DisableAutoCreation, BurstCompile]
    public partial struct S2BehaviourSystem : ISystem
    {
        [BurstCompile]
        public void OnUpdate(ref SystemState state)
        {
            var cfg = SystemAPI.GetSingleton<Config>();
            var input = SystemAPI.GetSingleton<FrameInput>();
            int cols = (int)math.ceil(cfg.Arena.x / cfg.Cell), rows = (int)math.ceil(cfg.Arena.y / cfg.Cell);

            var grid = new NativeParallelMultiHashMap<int, Neighbour>(input.Target, Allocator.TempJob);
            var infections = new NativeHashMap<Entity, Infector>(64, Allocator.TempJob);
            var ecb = new EntityCommandBuffer(Allocator.TempJob);
            new S2GridJob { Grid = grid.AsParallelWriter(), Cell = cfg.Cell, Cols = cols, Rows = rows }.ScheduleParallel();
            new S2InfectJob
            {
                Grid = grid,
                Targets = SystemAPI.GetComponentLookup<Target>(true),
                InfectedLookup = SystemAPI.GetComponentLookup<Infected>(true),
                Exists = SystemAPI.GetEntityStorageInfoLookup(),
                Memory = SystemAPI.GetBufferLookup<MemoryId>(),
                Infections = infections,
                Ecb = ecb,
                Frame = input.Frame,
                Cols = cols,
                Rows = rows,
                MemoryCap = cfg.MemoryCap,
                Cell = cfg.Cell,
                Radius = cfg.Radius,
                PInfect = cfg.PInfect * 5000.0 / input.Target,
            }.Schedule();
            state.CompleteDependency();

            foreach (var kv in infections)
                ecb.AddComponent(kv.Key, new Infected { Timer = cfg.InfectTime, Strain = kv.Value.Strain });
            ecb.Playback(state.EntityManager);
            ecb.Dispose();
            infections.Dispose();
            grid.Dispose();
        }
    }
}

public class EcsPath : IBenchPath
{
    World world;
    string scenario;
    Entity singletons;
    EntityQuery alive, infected, positions;
    (string, SystemHandle)[] phases;

    public void Setup(Scenario cfg, string scenario)
    {
        this.scenario = scenario;
        world = new World("bench");
        var em = world.EntityManager;
        var strains = new FixedList512Bytes<FixedString32Bytes>();
        foreach (var s in cfg.s2.strains)
            strains.Add(s);
        singletons = em.CreateEntity();
        em.AddComponentData(singletons, new EcsBench.Config
        {
            Dt = cfg.dt,
            Arena = new float2(cfg.arena[0], cfg.arena[1]),
            SpawnCapDiv = cfg.s1.spawn_cap_div,
            Emitter = new float2(cfg.s1.emitter[0], cfg.s1.emitter[1]),
            Gravity = cfg.s1.gravity,
            Drag = cfg.s1.drag,
            Speed = new float2(cfg.s1.speed[0], cfg.s1.speed[1]),
            AngleDeg = new float2(cfg.s1.angle_deg[0], cfg.s1.angle_deg[1]),
            Lifetime = new float2(cfg.s1.lifetime[0], cfg.s1.lifetime[1]),
            Restitution = new float2(cfg.s1.restitution[0], cfg.s1.restitution[1]),
            Bounces = new float2(cfg.s1.bounces[0], cfg.s1.bounces[1]),
            TrailEvery = cfg.s1.trail_every,
            TrailLen = cfg.s1.trail_len,
            Cell = cfg.s2.cell,
            AgentSpeed = cfg.s2.speed,
            ChaseSpeed = cfg.s2.chase_speed,
            Jitter = cfg.s2.jitter,
            Radius = cfg.s2.radius,
            PInfect = cfg.s2.p_infect,
            InfectTime = cfg.s2.infect_time,
            DamagePerS = cfg.s2.damage_per_s,
            Hp = cfg.s2.hp,
            SeedEvery = cfg.s2.seed_every,
            MemoryCap = cfg.s2.memory_cap,
            Strains = strains,
        });
        em.AddComponentData(singletons, new EcsBench.FrameInput());
        em.AddComponentData(singletons, new EcsBench.RngState { Value = cfg.seed });
        em.AddComponentData(singletons, new EcsBench.NextId());
        em.AddComponentData(singletons, new EcsBench.Score());

        phases = scenario == "s1"
            ? new[]
            {
                ("lifecycle", world.GetOrCreateSystem<EcsBench.S1LifecycleSystem>()),
                ("integrate", world.GetOrCreateSystem<EcsBench.S1IntegrateSystem>()),
                ("callbacks", world.GetOrCreateSystem<EcsBench.S1CallbacksSystem>()),
            }
            : new[]
            {
                ("lifecycle", world.GetOrCreateSystem<EcsBench.S2LifecycleSystem>()),
                ("integrate", world.GetOrCreateSystem<EcsBench.S2IntegrateSystem>()),
                ("behaviour", world.GetOrCreateSystem<EcsBench.S2BehaviourSystem>()),
            };
        alive = em.CreateEntityQuery(typeof(EcsBench.Velocity));
        infected = em.CreateEntityQuery(typeof(EcsBench.Infected));
        positions = em.CreateEntityQuery(typeof(LocalTransform), typeof(EcsBench.Velocity));
    }

    public void Phase(string phase, int frame, int target)
    {
        world.EntityManager.SetComponentData(singletons, new EcsBench.FrameInput { Frame = frame, Target = target });
        foreach (var (name, system) in phases)
            if (name == phase)
                system.Update(world.Unmanaged);
    }

    public int Alive() => alive.CalculateEntityCount();

    public int Infected() => infected.CalculateEntityCount();

    public (int, double, long) Checksum()
    {
        double sum = 0;
        using var transforms = positions.ToComponentDataArray<LocalTransform>(Allocator.Temp);
        foreach (var t in transforms)
            sum += t.Position.x + t.Position.y;
        var score = world.EntityManager.GetComponentData<EcsBench.Score>(singletons);
        return (transforms.Length, sum, score.Total);
    }
}
