# Engine benchmarks, GDScript path. See ../README.md for the rules.
# godot --headless --path benches/engines/godot --script res://main.gd -- --scenario s1 [--stages 1000,5000] [--out dir]
extends SceneTree

const WALLS = ["left", "right", "floor", "ceiling"]

signal bounced(ball: Ball, wall: int)

# S1, `Bouncy` and the trail are the script-defined components
class Bouncy extends RefCounted:
	var restitution: float
	var left: int

class Ball extends RefCounted:
	var pos: Vector2
	var vel: Vector2
	var life: float
	var tint: Color
	var bouncy: Bouncy
	var trail: Array[Vector2] = []

# S2, `memory` is the script-defined component
class Infection extends RefCounted:
	var timer: float
	var strain: String

class Agent extends RefCounted:
	var id: int
	var pos: Vector2
	var vel: Vector2
	var hp: float
	var infected: Infection = null
	var target: Agent = null
	var memory: PackedInt32Array = PackedInt32Array()
	var dead := false

var cfg: Dictionary
var scenario: String
var dt: float
var arena: Vector2
var rng: int

var balls: Array[Ball] = []
var hits: Array = []
var score := {"total": 0, "per_wall": {"left": 0, "right": 0, "floor": 0, "ceiling": 0}}

var agents: Array[Agent] = []
var next_id := 0


func rand() -> float:
	rng = (rng * 48271) % 2147483647
	return float(rng) / 2147483647.0


static func hash01(id: int, frame: int, salt: int) -> float:
	var x := (id * 7919 + frame * 104729 + salt) % 2147483646 + 1
	x = (x * 48271) % 2147483647
	x = (x * 48271) % 2147483647
	return float(x) / 2147483647.0


static func lerp_range(r: Array, t: float) -> float:
	return r[0] + (r[1] - r[0]) * t


func _initialize() -> void:
	var args := parse_args()
	var root := ProjectSettings.globalize_path("res://").path_join("..")
	cfg = JSON.parse_string(FileAccess.get_file_as_string(root.path_join("scenario.json")))
	scenario = args.get("scenario", "s1")
	dt = cfg.dt
	arena = Vector2(cfg.arena[0], cfg.arena[1])
	rng = int(cfg.seed)
	bounced.connect(on_bounce)

	var stages: Array = cfg.stages
	if args.has("stages"):
		stages = Array(args.stages.split(",")).map(func(s): return int(s))
	var phases: Array = ["lifecycle", "integrate", "callbacks"] if scenario == "s1" else ["lifecycle", "integrate", "behaviour"]
	var warmup: int = cfg.warmup_frames
	var measure: int = cfg.measure_frames

	var frame := 0
	var results := []
	var infected_samples := []
	for target in stages:
		var samples := {}
		for p in phases + ["frame"]:
			samples[p] = []
		var alive := 0
		for i in warmup + measure:
			var frame_ms := 0.0
			for p in phases:
				var start := Time.get_ticks_usec()
				run_phase(p, frame, target)
				var ms := (Time.get_ticks_usec() - start) / 1000.0
				frame_ms += ms
				if i >= warmup:
					samples[p].append(ms)
			if i >= warmup:
				samples["frame"].append(frame_ms)
				alive += balls.size() + agents.size()
			if frame % 60 == 0:
				infected_samples.append(agents.filter(func(a): return a.infected != null).size())
			frame += 1
		var stage := {"target": target, "alive": alive / measure}
		for p in samples:
			stage[p] = stats(samples[p])
		printerr("gdscript %s: %s" % [scenario, JSON.stringify(stage)])
		results.append(stage)

	var result := {
		"engine": "godot",
		"path": "gdscript",
		"scenario": scenario,
		"frames": frame,
		"stages": results,
		"checksum": checksum(),
		"infected": infected_samples,
	}
	var out: String = args.get("out", root.path_join("results"))
	DirAccess.make_dir_recursive_absolute(out)
	var file := FileAccess.open(out.path_join("%s_gdscript.json" % scenario), FileAccess.WRITE)
	file.store_string(JSON.stringify(result, "  "))
	file.close()
	printerr("checksum: ", JSON.stringify(result.checksum))
	quit()


func parse_args() -> Dictionary:
	var out := {}
	var args := OS.get_cmdline_user_args()
	for i in range(0, args.size() - 1, 2):
		out[args[i].trim_prefix("--")] = args[i + 1]
	return out


static func stats(samples: Array) -> Dictionary:
	samples.sort()
	var at := func(q: float): return samples[roundi((samples.size() - 1) * q)]
	return {"median": at.call(0.5), "p95": at.call(0.95)}


func run_phase(phase: String, frame: int, target: int) -> void:
	match [scenario, phase]:
		["s1", "lifecycle"]: s1_lifecycle(target)
		["s1", "integrate"]: s1_integrate(frame)
		["s1", "callbacks"]: s1_callbacks()
		["s2", "lifecycle"]: s2_lifecycle(target)
		["s2", "integrate"]: s2_integrate(frame)
		["s2", "behaviour"]: s2_behaviour(frame, target)


func checksum() -> Dictionary:
	var sum := 0.0
	var count := 0
	for b in balls:
		sum += b.pos.x + b.pos.y
	for a in agents:
		sum += a.pos.x + a.pos.y
	return {"alive": balls.size() + agents.size(), "sum": sum, "score": score.total}


# S1 Fountain

func s1_lifecycle(target: int) -> void:
	var s1: Dictionary = cfg.s1
	balls = balls.filter(func(b): return b.life > 0)
	var count := mini(target - balls.size(), maxi(1, floori(target / s1.spawn_cap_div)))
	for _i in count:
		var angle := deg_to_rad(lerp_range(s1.angle_deg, rand()))
		var speed := lerp_range(s1.speed, rand())
		var b := Ball.new()
		b.pos = Vector2(s1.emitter[0], s1.emitter[1])
		b.vel = Vector2(cos(angle), sin(angle)) * speed
		b.life = lerp_range(s1.lifetime, rand())
		b.tint = Color.WHITE
		b.bouncy = Bouncy.new()
		b.bouncy.restitution = lerp_range(s1.restitution, rand())
		b.bouncy.left = int(s1.bounces[0]) + floori(rand() * (s1.bounces[1] - s1.bounces[0] + 1))
		balls.append(b)


func s1_integrate(frame: int) -> void:
	var s1: Dictionary = cfg.s1
	var gravity := Vector2(0, s1.gravity * dt)
	var damp: float = 1.0 - s1.drag * dt
	var push_trail: bool = frame % int(s1.trail_every) == 0
	var trail_len: int = s1.trail_len
	var w := arena.x
	var h := arena.y
	for b in balls:
		var v := (b.vel + gravity) * damp
		var p := b.pos + v * dt
		var wall := 0
		if p.x < 0:
			p.x = -p.x; v.x = -v.x; wall = 1
		elif p.x > w:
			p.x = 2 * w - p.x; v.x = -v.x; wall = 2
		if p.y < 0:
			p.y = -p.y; v.y = -v.y; wall = 3
		elif p.y > h:
			p.y = 2 * h - p.y; v.y = -v.y; wall = 4
		if wall != 0:
			v *= b.bouncy.restitution
			hits.append([b, wall])
		b.vel = v
		b.pos = p
		b.life -= dt
		if push_trail:
			b.trail.push_front(p)
			if b.trail.size() > trail_len:
				b.trail.resize(trail_len)


func s1_callbacks() -> void:
	for hit in hits:
		bounced.emit(hit[0], hit[1])
	hits.clear()


func on_bounce(b: Ball, wall: int) -> void:
	score.total += 1
	score.per_wall[WALLS[wall - 1]] += 1
	b.bouncy.left -= 1
	var k := b.bouncy.left / 6.0
	b.tint = Color(1, k, k)
	if b.bouncy.left <= 0:
		b.life = 0


# S2 Infection

func s2_lifecycle(target: int) -> void:
	var s2: Dictionary = cfg.s2
	for a in agents:
		if a.hp <= 0:
			a.dead = true
	agents = agents.filter(func(a): return not a.dead)
	var count := mini(target - agents.size(), maxi(1, floori(target / cfg.s1.spawn_cap_div)))
	var strains: Array = s2.strains
	var seed_every: int = s2.seed_every
	for _i in count:
		var a := Agent.new()
		a.id = next_id
		next_id += 1
		a.pos = Vector2(hash01(a.id, 0, 1) * arena.x, hash01(a.id, 0, 2) * arena.y)
		a.vel = Vector2.from_angle(hash01(a.id, 0, 3) * TAU) * s2.speed
		a.hp = s2.hp
		if a.id % seed_every == 0:
			a.infected = Infection.new()
			a.infected.timer = s2.infect_time
			a.infected.strain = strains[(a.id / seed_every) % strains.size()]
		agents.append(a)


func s2_integrate(frame: int) -> void:
	var s2: Dictionary = cfg.s2
	var damage: float = s2.damage_per_s * dt
	var jitter: float = s2.jitter
	var chase_speed: float = s2.chase_speed
	for a in agents:
		if a.infected != null:
			a.hp -= damage
			a.infected.timer -= dt
	# random access: read every chaser's target position first
	var chase := {}
	for a in agents:
		if a.target != null and not a.target.dead:
			chase[a] = (a.target.pos - a.pos).normalized() * chase_speed
	for a in agents:
		if chase.has(a):
			a.vel = chase[a]
		else:
			a.vel = a.vel.rotated((hash01(a.id, frame, 4) - 0.5) * jitter)
		var p := a.pos + a.vel * dt
		a.pos = Vector2(fposmod(p.x, arena.x), fposmod(p.y, arena.y))


func s2_behaviour(frame: int, target: int) -> void:
	var s2: Dictionary = cfg.s2
	var cell: float = s2.cell
	var cols := ceili(arena.x / cell)
	var rows := ceili(arena.y / cell)
	var radius: float = s2.radius
	var p_infect: float = s2.p_infect * 5000.0 / target
	var cap: int = s2.memory_cap
	var infect_time: float = s2.infect_time

	# healthy agents bucketed by cell, infected agents are never neighbours
	var cells := []
	cells.resize(cols * rows)
	var infected: Array[Agent] = []
	for a in agents:
		if a.infected != null:
			infected.append(a)
			continue
		var key := mini(floori(a.pos.y / cell), rows - 1) * cols + mini(floori(a.pos.x / cell), cols - 1)
		if cells[key] == null:
			cells[key] = [a]
		else:
			cells[key].append(a)

	var infections := {}
	for i in infected:
		if i.infected.timer <= 0:
			i.infected = null
			i.target = null
			continue
		var cx := mini(floori(i.pos.x / cell), cols - 1)
		var cy := mini(floori(i.pos.y / cell), rows - 1)
		var nearest: Agent = null
		var nd := 0.0
		for y in range(maxi(cy - 1, 0), mini(cy + 1, rows - 1) + 1):
			for x in range(maxi(cx - 1, 0), mini(cx + 1, cols - 1) + 1):
				var bucket = cells[y * cols + x]
				if bucket == null:
					continue
				for other: Agent in bucket:
					var d := i.pos.distance_to(other.pos)
					if d < radius:
						if other.memory.size() < cap and not other.memory.has(i.id):
							other.memory.append(i.id)
						if hash01(other.id, frame, 5) < p_infect:
							var current = infections.get(other)
							if current == null or i.id < current.id:
								infections[other] = i
					if nearest == null or d < nd or (d == nd and other.id < nearest.id):
						nearest = other
						nd = d
		if i.target == null:
			i.target = nearest
		elif i.target.dead or i.target.infected != null:
			i.target = null

	for other in infections:
		var inf := Infection.new()
		inf.timer = infect_time
		inf.strain = infections[other].infected.strain
		other.infected = inf
