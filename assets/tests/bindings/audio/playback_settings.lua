-- presets
local PlaybackMode = import_namespace("audio").PlaybackMode
local PlaybackSettings = import_namespace("audio").PlaybackSettings
local once = PlaybackSettings.once()
assert(once.mode == PlaybackMode.Once, "once mode")
assert(once.speed == 1.0 and not once.paused and not once.spatial, "once defaults")
assert(PlaybackSettings.looping().mode == PlaybackMode.Loop, "looping mode")
assert(PlaybackSettings.despawn().mode == PlaybackMode.Despawn, "despawn mode")
assert(PlaybackSettings.remove().mode == PlaybackMode.Remove, "remove mode")

-- builders return a new value and leave the original alone
local quiet = once:with_volume(0.5)
assert(quiet.volume[1] == 0.5, "with_volume")
assert(once.volume[1] == 1.0, "with_volume does not change the original")
assert(once:with_speed(2.0).speed == 2.0, "with_speed")
assert(once:with_spatial(true).spatial, "with_spatial")
assert(not once.spatial, "with_spatial does not change the original")
