-- S2 Infection, see benches/engines/README.md
local bms = import_namespace("bms")
local world = bms.world
local construct = bms.construct
local DynamicComponent = bms.DynamicComponent
local Vec3 = import_namespace("math").Vec3

local Transform = world.get_type_by_name("bevy_transform::components::transform::Transform")
local Velocity = world.get_type_by_name("engines::common::Velocity")
local Agent = world.get_type_by_name("engines::common::Agent")
local Health = world.get_type_by_name("engines::common::Health")
local Infected = world.get_type_by_name("engines::common::Infected")
local Target = world.get_type_by_name("engines::common::Target")
local Memory

local cfg, s2, W, H, cols, rows, spawn_cap_div
local next_id = 0
local TAU = 2 * math.pi

local function hash01(id, frame, salt)
    local x = (id * 7919 + frame * 104729 + salt) % 2147483646 + 1
    x = (x * 48271) % 2147483647
    x = (x * 48271) % 2147483647
    return x / 2147483647
end

function on_setup(scenario)
    cfg = scenario
    s2 = scenario.s2
    W, H = scenario.arena[1], scenario.arena[2]
    cols, rows = math.ceil(W / s2.cell), math.ceil(H / s2.cell)
    spawn_cap_div = scenario.s1.spawn_cap_div
    Memory = world.register_new_component("script.bench", "Memory")
end

function on_lifecycle(frame, target)
    local alive, dead = 0, {}
    for e, health in world.query():component(Health):iter() do
        if health.hp <= 0 then
            dead[#dead + 1] = e
        else
            alive = alive + 1
        end
    end
    for i = 1, #dead do
        world.despawn(dead[i])
    end

    local count = math.min(target - alive, math.max(1, math.floor(target / spawn_cap_div)))
    for _ = 1, count do
        local id = next_id
        next_id = next_id + 1
        local a = hash01(id, 0, 3) * TAU
        local e = world.spawn()
        world.insert_component(e, Transform, Transform.from_xyz(hash01(id, 0, 1) * W, hash01(id, 0, 2) * H, 0))
        world.insert_component(e, Velocity,
            construct(Velocity, { v = Vec3.new(math.cos(a) * s2.speed, math.sin(a) * s2.speed, 0) }))
        world.insert_component(e, Agent, construct(Agent, { id = id }))
        world.insert_component(e, Health, construct(Health, { hp = s2.hp }))
        world.insert_component(e, Memory, DynamicComponent.new({ data = {} }))
        if id % s2.seed_every == 0 then
            local strain = s2.strains[math.floor(id / s2.seed_every) % #s2.strains + 1]
            world.insert_component(e, Infected, construct(Infected, { timer = s2.infect_time, strain = strain }))
        end
    end
end

function on_integrate(frame)
    local dt = cfg.dt
    local damage = s2.damage_per_s * dt
    for _, health, inf in world.query():component(Health):component(Infected):iter() do
        health.hp = health.hp - damage
        inf.timer = inf.timer - dt
    end

    -- random access: read every chaser's target position first
    local chase = {}
    for _, agent, target, transform in world.query():component(Agent):component(Target):component(Transform):iter() do
        local other = target.entity
        if world.has_entity(other) then
            local to = world.get_component(other, Transform).translation - transform.translation
            chase[agent.id] = to:normalize_or_zero() * s2.chase_speed
        end
    end

    for _, agent, transform, vel in world.query():component(Agent):component(Transform):component(Velocity):iter() do
        local v = chase[agent.id]
        if v == nil then
            local a = (hash01(agent.id, frame, 4) - 0.5) * s2.jitter
            local c, s = math.cos(a), math.sin(a)
            local old = vel.v
            v = Vec3.new(old.x * c - old.y * s, old.x * s + old.y * c, 0)
        end
        vel.v = v
        local p = transform.translation + v * dt
        p.x = p.x % W
        p.y = p.y % H
        transform.translation = p
    end
end

local function cell_of(x, y)
    return math.min(math.floor(x / s2.cell), cols - 1), math.min(math.floor(y / s2.cell), rows - 1)
end

function on_behaviour(frame, target)
    -- healthy agents bucketed by cell, infected agents are never neighbours
    local cells = {}
    for e, agent, transform in world.query():component(Agent):component(Transform):without(Infected):iter() do
        local p = transform.translation
        local cx, cy = cell_of(p.x, p.y)
        local key = cy * cols + cx
        local bucket = cells[key]
        if bucket == nil then
            bucket = {}
            cells[key] = bucket
        end
        bucket[#bucket + 1] = { e = e, id = agent.id, x = p.x, y = p.y }
    end

    local targets = {}
    for _, agent, target in world.query():component(Agent):component(Target):iter() do
        targets[agent.id] = target.entity
    end

    local recovered, untarget, retarget, infections = {}, {}, {}, {}
    local radius, p_infect, cap = s2.radius, s2.p_infect * 5000 / target, s2.memory_cap
    for e, agent, transform, inf in world.query():component(Agent):component(Transform):component(Infected):iter() do
        if inf.timer <= 0 then
            recovered[#recovered + 1] = e
        else
            local p = transform.translation
            local px, py = p.x, p.y
            local cx, cy = cell_of(px, py)
            local nearest, nd, nid
            for y = math.max(cy - 1, 0), math.min(cy + 1, rows - 1) do
                for x = math.max(cx - 1, 0), math.min(cx + 1, cols - 1) do
                    local bucket = cells[y * cols + x]
                    if bucket ~= nil then
                        for i = 1, #bucket do
                            local other = bucket[i]
                            local dx, dy = other.x - px, other.y - py
                            local d = math.sqrt(dx * dx + dy * dy)
                            if d < radius then
                                local mem = world.get_component(other.e, Memory)
                                local met = mem.data
                                if #met < cap then
                                    local seen = false
                                    for j = 1, #met do
                                        if met[j] == agent.id then seen = true break end
                                    end
                                    if not seen then
                                        met[#met + 1] = agent.id
                                        mem.data = met
                                    end
                                end
                                if hash01(other.id, frame, 5) < p_infect then
                                    local current = infections[other.id]
                                    if current == nil or agent.id < current.by then
                                        infections[other.id] = { e = other.e, by = agent.id, strain = inf.strain }
                                    end
                                end
                            end
                            if nearest == nil or d < nd or (d == nd and other.id < nid) then
                                nearest, nd, nid = other.e, d, other.id
                            end
                        end
                    end
                end
            end
            local current = targets[agent.id]
            if current == nil then
                if nearest ~= nil then
                    retarget[#retarget + 1] = { e = e, target = nearest }
                end
            elseif not world.has_entity(current) or world.has_component(current, Infected) then
                untarget[#untarget + 1] = e
            end
        end
    end

    for i = 1, #recovered do
        world.remove_component(recovered[i], Infected)
        world.remove_component(recovered[i], Target)
    end
    for i = 1, #untarget do
        world.remove_component(untarget[i], Target)
    end
    for i = 1, #retarget do
        local r = retarget[i]
        world.insert_component(r.e, Target, construct(Target, { entity = r.target }))
    end
    for _, inf in pairs(infections) do
        world.insert_component(inf.e, Infected, construct(Infected, { timer = s2.infect_time, strain = inf.strain }))
    end
end

function on_checksum()
    local alive, sum = 0, 0
    for _, transform in world.query():component(Transform):with(Velocity):iter() do
        local p = transform.translation
        alive = alive + 1
        sum = sum + p.x + p.y
    end
    return { alive = alive, sum = sum, score = 0 }
end
