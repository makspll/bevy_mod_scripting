-- The reactive graph running inside a Lua script: signals, computeds, effects, selectors and property bindings.
-- Rust mounts the ui and calls `on_ui_flush` once per flush, property values go back in one `ui.commit` call.
--
--   local rx = load(ui.lua_runtime())()
--   ui.text({ text = rx.bind(function() return "x" .. count:get() end) })

-- one graph per script, kept across reloads like the rest of the script's globals
if __REACTIVE_UI_RX then return __REACTIVE_UI_RX end

local ui = import_namespace("reactive_ui")
local rx = {}

local next_id = 0
local current = nil   -- the observer recording what it reads
local queue = {}      -- observers to run: [observer] = true (certain) or false (only if a computed it read changed)
local writes = {}     -- flat { slot, value, ... } for `ui.commit`
local requested = false
local watched = {}    -- rust watch id -> watch node

local function request()
    if not requested then
        requested = true
        ui.request_flush()
    end
end

local function node(kind, fields)
    next_id = next_id + 1
    fields.kind, fields.id, fields.subs, fields.deps, fields.version = kind, next_id, {}, {}, 0
    return fields
end

local function track(source)
    if current then
        source.subs[current] = true
        current.deps[source] = source.version
    end
end

local function forget_deps(observer)
    for dep in pairs(observer.deps) do
        dep.subs[observer] = nil
        if dep.by_key then
            for _, readers in pairs(dep.by_key) do readers[observer] = nil end
        end
    end
    observer.deps = {}
end

-- marks what read `source`: certain for direct readers, to be checked behind computeds
local function notify(source, certain)
    for sub in pairs(source.subs) do
        if sub.kind == "computed" then
            if not sub.stale then
                sub.stale = true
                notify(sub, false)
            end
        elseif sub.kind == "selector" then
            sub:move()
        else
            queue[sub] = queue[sub] or certain
        end
    end
    request()
end

local function run(observer)
    forget_deps(observer)
    local previous = current
    current = observer
    local ok, result = pcall(observer.fn)
    current = previous
    if not ok then error(result, 0) end
    return result
end

local Signal = {}
Signal.__index = Signal
function Signal:get() track(self); return self.value end
function Signal:peek() return self.value end
function Signal:set(value)
    if value ~= self.value then
        self.value = value
        self.version = self.version + 1
        notify(self, true)
    end
end

function rx.signal(value) return setmetatable(node("signal", { value = value }), Signal) end

local Computed = {}
Computed.__index = Computed
-- brings it up to date, its version only moves when its value changed
function Computed:refresh()
    if self.stale then
        self.stale = false
        local value = run(self)
        if value ~= self.value then
            self.value = value
            self.version = self.version + 1
        end
    end
end
function Computed:get() self:refresh(); track(self); return self.value end
function Computed:peek() self:refresh(); return self.value end

function rx.computed(fn) return setmetatable(node("computed", { fn = fn, stale = true }), Computed) end

-- whether anything the observer read changed since it last ran
local function changed(observer)
    for dep, seen in pairs(observer.deps) do
        if dep.refresh then dep:refresh() end
        if dep.version ~= seen then return true end
    end
    return false
end

--- A property value computed by `fn`, re-run when what it reads changes.
function rx.bind(fn)
    local binding = node("binding", { fn = fn })
    binding.slot = binding.id
    queue[binding] = true
    request()
    return ui.slot(binding.slot)
end

function rx.effect(fn)
    local effect = node("effect", { fn = fn })
    queue[effect] = true
    request()
end

local Watch = {}
Watch.__index = Watch
function Watch:get()
    track(self)
    if self.ref == nil then self.ref = self.handle:get() end
    return self.ref
end

--- A watched component: `:get()` is the live component, readers re-run when it changes.
function rx.watch(entity, component)
    local handle = ui.watch(entity, component)
    local watch = setmetatable(node("watch", { handle = handle }), Watch)
    watched[ui.subscribe(handle)] = watch
    return watch
end

local Selector = {}
Selector.__index = Selector
function Selector:move()
    local new = self.source.value
    if new == self.current then return end
    for _, key in ipairs({ self.current, new }) do
        for reader in pairs(self.by_key[key] or {}) do queue[reader] = true end
    end
    self.current = new
    self.version = self.version + 1
end
function Selector:is(key)
    if current then
        self.by_key[key] = self.by_key[key] or {}
        self.by_key[key][current] = true
        current.deps[self] = self.version
    end
    return self.current == key
end

function rx.selector(source)
    local selector = setmetatable(node("selector", { source = source, current = source.value, by_key = {} }), Selector)
    source.subs[selector] = true
    return selector
end

-- graph objects can't cross into Rust (i.e. in component props), pass a number instead
local refs = {}
function rx.ref(object)
    next_id = next_id + 1
    refs[next_id] = object
    return next_id
end
function rx.deref(id) return refs[id] end

local function by_id(a, b) return a.id < b.id end

--- Runs what changed, then sends the property values to Rust. Called by Rust with the changed watch ids.
function rx.flush(changed_watches)
    -- what changes while flushing runs in this flush, it needs no other one
    requested = true
    for _, id in ipairs(changed_watches or {}) do
        local watch = watched[id]
        if watch then
            watch.ref = nil
            watch.version = watch.version + 1
            notify(watch, true)
        end
    end
    for _ = 1, 16 do
        if next(queue) == nil then break end
        local batch = {}
        for observer, certain in pairs(queue) do
            batch[#batch + 1] = observer
            observer.certain = certain
        end
        queue = {}
        table.sort(batch, by_id)
        for _, effects in ipairs({ false, true }) do
            for _, observer in ipairs(batch) do
                if (observer.kind == "effect") == effects and (observer.certain or changed(observer)) then
                    if observer.cleanup then observer.cleanup() end
                    local result = run(observer)
                    -- nil would leave a hole in the list, it keeps the previous value
                    if observer.kind == "binding" and result ~= nil then
                        writes[#writes + 1] = observer.slot
                        writes[#writes + 1] = result
                    elseif type(result) == "function" then
                        observer.cleanup = result
                    end
                end
            end
        end
    end
    requested = false
    if #writes > 0 then
        ui.commit(writes)
        writes = {}
    end
end

on_ui_flush = rx.flush
__REACTIVE_UI_RX = rx
return rx
