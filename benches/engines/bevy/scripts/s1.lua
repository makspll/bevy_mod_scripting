-- S1 Fountain, see benches/engines/README.md
local bms = import_namespace("bms")
local world = bms.world
local construct = bms.construct
local DynamicComponent = bms.DynamicComponent
local Vec3 = import_namespace("math").Vec3
local bench = import_namespace("bench")

local Transform = world.get_type_by_name("bevy_transform::components::transform::Transform")
local Velocity = world.get_type_by_name("engines::common::Velocity")
local Lifetime = world.get_type_by_name("engines::common::Lifetime")
local Tint = world.get_type_by_name("engines::common::Tint")
local Score = world.get_type_by_name("engines::common::Score")
local Bouncy, Trail

local WALLS = { "left", "right", "floor", "ceiling" }
local cfg, s1, rng
local W, H

local function rand()
    rng = (rng * 48271) % 2147483647
    return rng / 2147483647
end

local function lerp(range, t)
    return range[1] + (range[2] - range[1]) * t
end

function on_setup(scenario)
    cfg = scenario
    s1 = scenario.s1
    rng = scenario.seed
    W, H = scenario.arena[1], scenario.arena[2]
    Bouncy = world.register_new_component("script.bench", "Bouncy")
    Trail = world.register_new_component("script.bench", "Trail")
end

function on_lifecycle(frame, target)
    local alive, dead = 0, {}
    for e, life in world.query():component(Lifetime):iter() do
        if life.t <= 0 then
            dead[#dead + 1] = e
        else
            alive = alive + 1
        end
    end
    for i = 1, #dead do
        world.despawn(dead[i])
    end

    local count = math.min(target - alive, math.max(1, math.floor(target / s1.spawn_cap_div)))
    local ex, ey = s1.emitter[1], s1.emitter[2]
    for _ = 1, count do
        local angle = math.rad(lerp(s1.angle_deg, rand()))
        local speed = lerp(s1.speed, rand())
        local lifetime = lerp(s1.lifetime, rand())
        local restitution = lerp(s1.restitution, rand())
        local left = s1.bounces[1] + math.floor(rand() * (s1.bounces[2] - s1.bounces[1] + 1))

        local e = world.spawn()
        world.insert_component(e, Transform, Transform.from_xyz(ex, ey, 0))
        world.insert_component(e, Velocity,
            construct(Velocity, { v = Vec3.new(math.cos(angle) * speed, math.sin(angle) * speed, 0) }))
        world.insert_component(e, Lifetime, construct(Lifetime, { t = lifetime }))
        world.insert_component(e, Tint, construct(Tint, { r = 1, g = 1, b = 1 }))
        world.insert_component(e, Bouncy, DynamicComponent.new({ data = { restitution = restitution, left = left } }))
        world.insert_component(e, Trail, DynamicComponent.new({ data = {} }))
    end
end

function on_integrate(frame)
    local dt = cfg.dt
    local gravity = Vec3.new(0, s1.gravity * dt, 0)
    local damp = 1 - s1.drag * dt
    local push_trail = frame % s1.trail_every == 0
    local trail_len = s1.trail_len * 2

    for e, transform, vel, life, bouncy, trail in world.query()
        :component(Transform):component(Velocity):component(Lifetime)
        :component(Bouncy):component(Trail):iter() do
        local v = (vel.v + gravity) * damp
        local p = transform.translation + v * dt
        local wall = 0
        if p.x < 0 then
            p.x = -p.x; v.x = -v.x; wall = 1
        elseif p.x > W then
            p.x = 2 * W - p.x; v.x = -v.x; wall = 2
        end
        if p.y < 0 then
            p.y = -p.y; v.y = -v.y; wall = 3
        elseif p.y > H then
            p.y = 2 * H - p.y; v.y = -v.y; wall = 4
        end
        if wall ~= 0 then
            v = v * bouncy.data.restitution
            bench.report_hit(e, wall)
        end
        vel.v = v
        transform.translation = p
        life.t = life.t - dt

        if push_trail then
            -- flat list of x, y pairs, newest first
            local old = trail.data
            local new = { p.x, p.y }
            for i = 1, math.min(#old, trail_len - 2) do
                new[i + 2] = old[i]
            end
            trail.data = new
        end
    end
end

function on_bounce(e, wall)
    local score = world.get_resource(Score)
    score.total = score.total + 1
    local name = WALLS[wall]
    score.per_wall[name] = score.per_wall[name] + 1

    local bouncy = world.get_component(e, Bouncy)
    if bouncy == nil then return end
    local data = bouncy.data
    data.left = data.left - 1
    bouncy.data = data

    local k = data.left / 6
    local tint = world.get_component(e, Tint)
    tint.r = 1; tint.g = k; tint.b = k
    if data.left <= 0 then
        world.get_component(e, Lifetime).t = 0
    end
end

function on_checksum()
    local alive, sum = 0, 0
    for _, transform in world.query():component(Transform):with(Lifetime):iter() do
        local p = transform.translation
        alive = alive + 1
        sum = sum + p.x + p.y
    end
    return { alive = alive, sum = sum, score = world.get_resource(Score).total }
end
