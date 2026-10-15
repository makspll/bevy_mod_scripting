// presets
let PlaybackMode = import_namespace.call("audio").PlaybackMode;
let PlaybackSettings = import_namespace.call("audio").PlaybackSettings;
let once = PlaybackSettings.once.call();
assert(once.mode == PlaybackMode.Once, "once mode");
assert(once.speed == 1.0 && !once.paused && !once.spatial, "once defaults");
assert(PlaybackSettings.looping.call().mode == PlaybackMode.Loop, "looping mode");
assert(PlaybackSettings.despawn.call().mode == PlaybackMode.Despawn, "despawn mode");
assert(PlaybackSettings.remove.call().mode == PlaybackMode.Remove, "remove mode");

// builders return a new value and leave the original alone
let quiet = once.with_volume.call(0.5);
assert(quiet.volume[0] == 0.5, "with_volume");
assert(once.volume[0] == 1.0, "with_volume does not change the original");
assert(once.with_speed.call(2.0).speed == 2.0, "with_speed");
assert(once.with_spatial.call(true).spatial, "with_spatial");
assert(!once.spatial, "with_spatial does not change the original");
